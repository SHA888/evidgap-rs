use anyhow::Result;
use std::process::Command;

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

    // Step 2: Check if release.toml exists
    eprintln!("  • Checking for release.toml...");
    if !std::path::Path::new("release.toml").exists() {
        eprintln!("ℹ  release.toml not found (will be created in Phase 0.0.8)");
        eprintln!("ℹ  Skipping dry-run until release configuration is ready");
        return Ok(());
    }
    eprintln!("  ✓ Found: release.toml");

    // Step 3: Run cargo release dry-run
    eprintln!("\n  • Running 'cargo release --dry-run'...");
    let release_output = Command::new("cargo")
        .args(["release", "--dry-run", "--allow-dirty"])
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to run cargo release: {}", e))?;

    let stdout = String::from_utf8_lossy(&release_output.stdout);
    let stderr = String::from_utf8_lossy(&release_output.stderr);

    if !release_output.status.success() {
        eprintln!(
            "\n❌ Release dry-run failed. Check output for details.\n{}\n{}",
            stdout, stderr
        );
        return Err(anyhow::anyhow!(
            "cargo release --dry-run failed; review SemVer and CHANGELOG entries"
        ));
    }

    // Step 4: Report results
    eprint!("{}", stdout);
    if !stderr.is_empty() {
        eprint!("{}", stderr);
    }

    eprintln!("\n✓ Release dry-run simulation passed");
    eprintln!("  Proposed tags and versions are valid for publishing");
    Ok(())
}
