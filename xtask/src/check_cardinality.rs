use anyhow::{Result, anyhow};
use std::fs;
use std::path::Path;
use syn::{Item, ItemTrait};

/// Verify each port method has a `# Cardinality` rustdoc section explaining Vec vs Stream choice.
/// Enforces ARCHITECTURE.md cardinality discipline: "For each port method, expected result
/// cardinality is decided at port-definition time."
pub fn check_cardinality() -> Result<()> {
    let workspace_root = std::env::current_dir()?;
    let ports_dir = workspace_root.join("crates/evidgap-ports/src");

    if !ports_dir.exists() {
        return Err(anyhow!(
            "evidgap-ports crate not found at {}",
            ports_dir.display()
        ));
    }

    // Step 1: Enumerate port traits and their methods
    let port_traits = enumerate_port_traits_with_methods(&ports_dir)?;

    if port_traits.is_empty() {
        eprintln!("ℹ  No port traits found in evidgap-ports (expected in v0.1.0+)");
        return Ok(());
    }

    eprintln!(
        "📋 Checking cardinality documentation for {} port trait(s):",
        port_traits.len()
    );

    // Step 2: For each method, verify it has a "# Cardinality" section in rustdoc
    let mut violations = Vec::new();
    for (trait_name, methods) in &port_traits {
        for method_name in methods {
            if !has_cardinality_doc(&ports_dir, trait_name, method_name)? {
                violations.push((trait_name.clone(), method_name.clone()));
            }
        }
    }

    if violations.is_empty() {
        eprintln!("✓ All port methods have cardinality documentation (Vec/Stream discipline met)");
        Ok(())
    } else {
        eprintln!("\n❌ Cardinality documentation missing:");
        for (trait_name, method_name) in &violations {
            eprintln!("  • {}::{}", trait_name, method_name);
        }
        eprintln!("\nEvery port method must document its cardinality choice:");
        eprintln!("  /// # Cardinality");
        eprintln!(
            "  /// Returns `Vec<Sourced<T>>` for small bounded results (e.g., compounds for a single target)."
        );
        eprintln!(
            "  /// Would return `impl Stream<Item = Sourced<T>>` for unbounded or large results."
        );
        eprintln!("  async fn compounds_for_target(...) -> PortResult<Vec<Sourced<Compound>>>;");
        Err(anyhow!(
            "check-cardinality failed: {} method(s) missing cardinality docs",
            violations.len()
        ))
    }
}

/// Enumerate all port traits and their async methods from evidgap-ports/src/**/*.rs files.
/// Returns a map of (trait_name, vec_of_method_names).
fn enumerate_port_traits_with_methods(
    ports_dir: &Path,
) -> Result<std::collections::HashMap<String, Vec<String>>> {
    let mut port_methods: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();

    // Walk all .rs files in evidgap-ports/src
    for entry in glob::glob(&format!("{}/**/*.rs", ports_dir.display()))
        .map_err(|e| anyhow!("glob error: {}", e))?
    {
        let path = entry?;
        let content = fs::read_to_string(&path)
            .map_err(|e| anyhow!("failed to read {}: {}", path.display(), e))?;

        // Parse file with syn
        let file = syn::parse_file(&content)
            .map_err(|e| anyhow!("failed to parse {}: {}", path.display(), e))?;

        // Walk items looking for traits with #[async_trait] and names ending in "Port"
        for item in file.items {
            if let Item::Trait(ItemTrait { ident, items, .. }) = item {
                let trait_name = ident.to_string();
                if trait_name.ends_with("Port") {
                    // Extract method names from trait items
                    let mut methods = Vec::new();
                    for trait_item in items {
                        if let syn::TraitItem::Fn(trait_fn) = trait_item {
                            let method_name = trait_fn.sig.ident.to_string();
                            methods.push(method_name);
                        }
                    }
                    if !methods.is_empty() {
                        port_methods.insert(trait_name, methods);
                    }
                }
            }
        }
    }

    Ok(port_methods)
}

/// Check if a specific port method has a `# Cardinality` section in its rustdoc.
fn has_cardinality_doc(ports_dir: &Path, trait_name: &str, method_name: &str) -> Result<bool> {
    // Scan all files looking for the trait and method combination
    for entry in glob::glob(&format!("{}/**/*.rs", ports_dir.display()))
        .map_err(|e| anyhow!("glob error: {}", e))?
    {
        let path = entry?;
        let content = fs::read_to_string(&path).ok();
        if content.is_none() {
            continue;
        }

        let content = content.unwrap();

        // Look for the trait definition
        if !content.contains(&format!("trait {}", trait_name)) {
            continue;
        }

        // Parse and search for the method
        let file = match syn::parse_file(&content) {
            Ok(f) => f,
            Err(_) => continue,
        };

        for item in file.items {
            if let Item::Trait(ItemTrait { ident, items, .. }) = item
                && ident == trait_name
            {
                for trait_item in items {
                    if let syn::TraitItem::Fn(trait_fn) = trait_item
                        && trait_fn.sig.ident == method_name
                    {
                        // Check if the method has rustdoc with "# Cardinality"
                        return Ok(has_cardinality_in_docs(&trait_fn.attrs));
                    }
                }
            }
        }
    }

    // Method not found — conservative: treat as missing doc
    Ok(false)
}

/// Check if doc attributes contain "# Cardinality" section.
fn has_cardinality_in_docs(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if let syn::Meta::NameValue(nv) = &attr.meta
            && nv.path.is_ident("doc")
            && let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(lit_str),
                ..
            }) = &nv.value
        {
            return lit_str.value().contains("# Cardinality");
        }
        false
    })
}
