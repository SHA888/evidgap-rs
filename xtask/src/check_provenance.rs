use anyhow::{Result, anyhow};
use std::fs;
use std::path::Path;
use syn::{Item, ItemTrait, ReturnType, TraitItem, Type};

/// Verify each port method returns `Sourced<T>` wrapped types, never bare entity types.
/// Enforces ARCHITECTURE.md provenance principle: "Bare facts do not cross a port boundary."
pub fn check_provenance() -> Result<()> {
    let workspace_root = std::env::current_dir()?;
    let ports_dir = workspace_root.join("crates/evidgap-ports/src");

    if !ports_dir.exists() {
        return Err(anyhow!(
            "evidgap-ports crate not found at {}",
            ports_dir.display()
        ));
    }

    // Step 1: Enumerate port traits and their methods
    let port_methods = enumerate_port_methods(&ports_dir)?;

    if port_methods.is_empty() {
        eprintln!("ℹ  No port traits found in evidgap-ports (expected in v0.1.0+)");
        return Ok(());
    }

    eprintln!(
        "📋 Checking provenance for {} port trait(s):",
        port_methods.len()
    );

    // Step 2: For each method, verify return type is wrapped in Sourced<T> or PortResult<>
    let mut violations = Vec::new();
    for (trait_name, methods) in &port_methods {
        for (method_name, return_type_str) in methods {
            if !is_provenance_safe(return_type_str) {
                violations.push((
                    trait_name.clone(),
                    method_name.clone(),
                    return_type_str.clone(),
                ));
            }
        }
    }

    if violations.is_empty() {
        eprintln!("✓ All port methods return Sourced<T> (provenance discipline met)");
        Ok(())
    } else {
        eprintln!("\n❌ Provenance violations detected:");
        for (trait_name, method_name, return_type) in &violations {
            eprintln!(
                "  • {}::{} returns bare type: {}",
                trait_name, method_name, return_type
            );
        }
        eprintln!("\nEvery port method must wrap results in Sourced<T>:");
        eprintln!("  ✗ async fn compounds_for_target(...) -> PortResult<Vec<Compound>>;");
        eprintln!("  ✓ async fn compounds_for_target(...) -> PortResult<Vec<Sourced<Compound>>>;");
        eprintln!("\nAlternatively, for unbounded results:");
        eprintln!(
            "  ✓ async fn publications(...) -> PortResult<impl Stream<Item = Sourced<Publication>>>;"
        );
        Err(anyhow!(
            "check-provenance failed: {} method(s) return bare entity types",
            violations.len()
        ))
    }
}

/// Enumerate port methods and extract their return type strings from evidgap-ports/src/**/*.rs files.
fn enumerate_port_methods(
    ports_dir: &Path,
) -> Result<std::collections::HashMap<String, Vec<(String, String)>>> {
    let mut port_methods: std::collections::HashMap<String, Vec<(String, String)>> =
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

        // Walk items looking for port traits (ending in "Port")
        for item in file.items {
            if let Item::Trait(ItemTrait { ident, items, .. }) = item {
                let trait_name = ident.to_string();
                if trait_name.ends_with("Port") {
                    // Extract method signatures and return types
                    let mut methods = Vec::new();
                    for trait_item in items {
                        if let TraitItem::Fn(trait_fn) = trait_item {
                            let method_name = trait_fn.sig.ident.to_string();
                            let return_type_str = extract_return_type(&trait_fn.sig.output);
                            methods.push((method_name, return_type_str));
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

/// Extract return type as a string from a function signature's output.
fn extract_return_type(output: &ReturnType) -> String {
    match output {
        ReturnType::Default => "()".to_string(),
        ReturnType::Type(_, ty) => type_to_string(ty),
    }
}

/// Convert a syn::Type to a string representation.
fn type_to_string(ty: &Type) -> String {
    match ty {
        Type::Path(type_path) => {
            // For path types like Vec<Sourced<T>>, extract the full segment
            let segments = &type_path.path.segments;
            segments
                .iter()
                .map(|seg| {
                    let mut segment = seg.ident.to_string();
                    if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
                        let arg_str = args
                            .args
                            .iter()
                            .filter_map(|arg| {
                                if let syn::GenericArgument::Type(ty) = arg {
                                    Some(type_to_string(ty))
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<_>>()
                            .join(", ");
                        if !arg_str.is_empty() {
                            segment.push('<');
                            segment.push_str(&arg_str);
                            segment.push('>');
                        }
                    }
                    segment
                })
                .collect::<Vec<_>>()
                .join("::")
        }
        Type::ImplTrait(_) => "impl Trait".to_string(),
        _ => "unknown".to_string(),
    }
}

/// Check if a return type is provenance-safe (wrapped in Sourced or PortResult with Sourced).
/// Heuristic: look for "Sourced" in the type string or "PortResult" wrapper.
fn is_provenance_safe(return_type: &str) -> bool {
    // PortResult wrapping is OK as long as the inner type eventually has Sourced
    // Heuristic: if it contains "Sourced", it's safe
    // More sophisticated check would parse the full type, but this covers common cases:
    // - PortResult<Vec<Sourced<T>>>
    // - PortResult<impl Stream<Item = Sourced<T>>>
    // - Option<Sourced<T>>
    // - Result<Vec<Sourced<T>>, Error>
    let contains_sourced = return_type.contains("Sourced");

    // Also allow Result and Option types that wrap Sourced
    // and Stream types with Sourced items
    let is_stream_sourced = return_type.contains("Stream") && return_type.contains("Sourced");
    let is_result_sourced = return_type.contains("Result") && return_type.contains("Sourced");
    let is_option_sourced = return_type.contains("Option") && return_type.contains("Sourced");

    contains_sourced || is_stream_sourced || is_result_sourced || is_option_sourced
}
