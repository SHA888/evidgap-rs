use anyhow::{Result, anyhow};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use syn::{Attribute, Item, ItemTrait, Meta, parse_file};

/// Enumerate port traits from evidgap-ports crate via syn parsing.
/// Verify each trait has >= 2 implementations (1 real + 1 fixture).
/// Exit with code 0 if all checks pass; non-zero if any port has < 2 impls.
pub fn check_ports() -> Result<()> {
    let workspace_root = std::env::current_dir()?;
    let ports_dir = workspace_root.join("crates/evidgap-ports/src");

    if !ports_dir.exists() {
        return Err(anyhow!(
            "evidgap-ports crate not found at {}",
            ports_dir.display()
        ));
    }

    // Step 1: Enumerate all port trait names from evidgap-ports/src/**/*.rs
    let port_traits = enumerate_port_traits(&ports_dir)?;

    if port_traits.is_empty() {
        eprintln!("ℹ  No port traits found in evidgap-ports (expected in v0.1.0+)");
        return Ok(());
    }

    eprintln!("📋 Found {} port trait(s):", port_traits.len());
    for trait_name in &port_traits {
        eprintln!("  • {}", trait_name);
    }

    // Step 2: For each port trait, count implementations in the entire workspace
    let implementations = count_implementations(&workspace_root, &port_traits)?;

    // Step 3: Verify each trait has >= 2 implementations
    let mut violations = Vec::new();
    for trait_name in &port_traits {
        let impl_count = implementations.get(trait_name).copied().unwrap_or(0);
        if impl_count < 2 {
            violations.push((trait_name.clone(), impl_count));
        }
    }

    if violations.is_empty() {
        eprintln!("✓ All port traits have >= 2 implementations (dual-adapter requirement met)");
        Ok(())
    } else {
        eprintln!("\n❌ Dual-adapter requirement violated:");
        for (trait_name, impl_count) in &violations {
            eprintln!(
                "  • {}: {} implementation(s) found (expected >= 2)",
                trait_name, impl_count
            );
        }
        eprintln!("\nEvery port trait must have:");
        eprintln!("  1. A real adapter (MCP, Arrow, or other transport)");
        eprintln!("  2. A FixtureAdapter (in-memory, under evidgap-ports/src/fixtures/)");
        Err(anyhow!(
            "check-ports failed: {} trait(s) need more implementations",
            violations.len()
        ))
    }
}

/// Enumerate all port trait names from evidgap-ports/src/**/*.rs files.
/// Returns trait names ending in "Port" with `#[async_trait]` attribute.
fn enumerate_port_traits(ports_dir: &Path) -> Result<HashSet<String>> {
    let mut port_traits = HashSet::new();

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
            if let Item::Trait(ItemTrait { ident, attrs, .. }) = item {
                let trait_name = ident.to_string();
                if trait_name.ends_with("Port") && has_async_trait_attr(&attrs) {
                    port_traits.insert(trait_name);
                }
            }
        }
    }

    Ok(port_traits)
}

/// Check if trait has `#[async_trait]` attribute.
fn has_async_trait_attr(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if let Meta::Path(path) = &attr.meta {
            path.is_ident("async_trait")
        } else {
            false
        }
    })
}

/// Count implementations of each port trait in the entire workspace.
/// Scans all Rust files for `impl PortTraitName for ...` patterns.
fn count_implementations(
    workspace_root: &Path,
    port_traits: &HashSet<String>,
) -> Result<HashMap<String, u32>> {
    let mut implementations: HashMap<String, u32> = HashMap::new();

    // Initialize count for each trait
    for trait_name in port_traits {
        implementations.insert(trait_name.clone(), 0);
    }

    // Scan all .rs files in workspace (skip target/)
    let glob_pattern = format!("{}/**/src/**/*.rs", workspace_root.display());
    for entry in glob::glob(&glob_pattern).map_err(|e| anyhow!("glob error: {}", e))? {
        let path = entry?;

        // Skip target/ and test files (for now; future: allow #[cfg(test)] fixture impls)
        let path_str = path.to_string_lossy();
        if path_str.contains("/target/") || path_str.ends_with("_test.rs") {
            continue;
        }

        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue, // Skip unreadable files
        };

        // Parse with syn and enumerate impl blocks
        let file = match parse_file(&content) {
            Ok(f) => f,
            Err(_) => continue, // Skip unparsable files (e.g., generated)
        };

        for item in file.items {
            if let syn::Item::Impl(impl_block) = item {
                // impl <TraitName> for ...
                if let Some((_, trait_path, _)) = &impl_block.trait_
                    && let Ok(trait_name) = extract_trait_name(trait_path)
                    && let Some(count) = implementations.get_mut(&trait_name)
                {
                    *count += 1;
                }
            }
        }
    }

    eprintln!("\n📊 Implementation counts:");
    for trait_name in port_traits {
        let count = implementations.get(trait_name).copied().unwrap_or(0);
        eprintln!("  {}: {} impl(s)", trait_name, count);
    }

    Ok(implementations)
}

/// Extract trait name from a trait path in an impl block.
/// Handles both simple (e.g., `MechanismPort`) and qualified (e.g., `foo::MechanismPort`) forms.
fn extract_trait_name(trait_path: &syn::Path) -> Result<String> {
    trait_path
        .segments
        .last()
        .ok_or_else(|| anyhow!("empty trait path"))
        .map(|seg| seg.ident.to_string())
}
