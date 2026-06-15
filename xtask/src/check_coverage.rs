use anyhow::Result;
use std::process::{Command, Stdio};

/// Generate code coverage report for the workspace.
/// Requires cargo-llvm-cov and llvm-tools-preview component.
/// Exits gracefully if tools unavailable (optional for Phase 0).
pub fn check_coverage() -> Result<()> {
    eprintln!("📋 Generating workspace code coverage...");

    // Step 1: Check if cargo-llvm-cov is available
    eprintln!("\n  • Checking for cargo-llvm-cov...");
    let version_check = Command::new("cargo")
        .args(["llvm-cov", "--version"])
        .output();

    match version_check {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            eprintln!("  ✓ Found: {}", version.trim());
        }
        _ => {
            eprintln!("ℹ  cargo-llvm-cov not available (install: cargo install cargo-llvm-cov)");
            eprintln!("ℹ  llvm-tools not available (optional for Phase 0; coverage skipped)");
            eprintln!("ℹ  To enable coverage: rustup component add llvm-tools-preview");
            return Ok(());
        }
    }

    // Step 2: Run cargo llvm-cov to generate coverage report (inherit stdout/stderr for live output)
    eprintln!("\n  • Running 'cargo llvm-cov --workspace --html'...\n");
    let status = Command::new("cargo")
        .args(["llvm-cov", "--workspace", "--html"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|e| anyhow::anyhow!("Failed to run cargo llvm-cov: {}", e))?;

    if !status.success() {
        eprintln!("\n❌ Coverage generation failed; ensure llvm-tools-preview is installed");
        return Err(anyhow::anyhow!(
            "cargo llvm-cov failed; check output above for details"
        ));
    }

    eprintln!("\n✓ Workspace coverage report generated");
    eprintln!("  View report: target/llvm-cov/html/index.html");
    Ok(())
}
