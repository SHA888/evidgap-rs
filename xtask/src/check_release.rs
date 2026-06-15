use anyhow::Result;
use std::process::{Command, Stdio};

/// Simulate a release with `cargo release --dry-run`.
/// Verifies SemVer compliance and shows proposed tags without publishing.
/// Gracefully handles missing release.toml (expected in Phase 0.0.5).
pub fn check_release() -> Result<()> {
    eprintln!("📋 Running release dry-run simulation...");

    // Step 1: Check if cargo-release is installed
    eprintln!("\n  • Checking for cargo-release...");
    let version_check = Command::new("cargo")
        .args(["release", "--version"])
        .output();

    match version_check {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            eprintln!("  ✓ Found: {}", version.trim());
        }
        _ => {
            return Err(anyhow::anyhow!(
                "cargo-release not found (install: cargo install cargo-release)"
            ));
        }
    }

    // Step 2: Check if release.toml exists (workspace-relative)
    eprintln!("  • Checking for release.toml...");
    let workspace_root = std::env::current_dir()?;
    let release_toml = workspace_root.join("release.toml");
    if !release_toml.exists() {
        eprintln!("ℹ  release.toml not found (will be created in Phase 0.0.8)");
        eprintln!("ℹ  Skipping dry-run until release configuration is ready");
        return Ok(());
    }
    eprintln!("  ✓ Found: release.toml");

    // Step 3: Run cargo release dry-run (inherit stdout/stderr for live output)
    eprintln!("\n  • Running 'cargo release patch --workspace'...\n");
    let status = Command::new("cargo")
        .args(["release", "patch", "--workspace"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|e| anyhow::anyhow!("Failed to run cargo release: {}", e))?;

    if !status.success() {
        eprintln!("\n❌ Release dry-run failed; review SemVer and CHANGELOG entries");
        return Err(anyhow::anyhow!(
            "cargo release --dry-run failed; check output above for details"
        ));
    }

    eprintln!("\n✓ Release dry-run simulation passed");
    eprintln!("  Proposed tags and versions are valid for publishing");
    Ok(())
}
