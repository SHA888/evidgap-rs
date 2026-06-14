use anyhow::{Result, anyhow};
use std::process::Command;

/// Verify workspace builds on Minimum Supported Rust Version (MSRV).
/// Installs MSRV toolchain if needed and runs `cargo check --workspace`.
pub fn check_msrv() -> Result<()> {
    const MSRV: &str = "1.94.0";

    eprintln!(
        "📋 Checking Minimum Supported Rust Version (MSRV): {}",
        MSRV
    );

    // Step 1: Ensure MSRV toolchain is installed
    eprintln!("  • Installing/updating MSRV toolchain...");
    let install_status = Command::new("rustup")
        .args(["toolchain", "install", MSRV])
        .status()
        .map_err(|e| anyhow!("Failed to run rustup: {}", e))?;

    if !install_status.success() {
        return Err(anyhow!(
            "Failed to install MSRV toolchain {}. Ensure rustup is installed.",
            MSRV
        ));
    }

    // Step 2: Run cargo check on MSRV toolchain
    eprintln!("  • Running 'cargo +{} check --workspace'...", MSRV);
    let check_status = Command::new("cargo")
        .arg(format!("+{}", MSRV))
        .args(["check", "--workspace"])
        .status()
        .map_err(|e| anyhow!("Failed to run cargo: {}", e))?;

    if check_status.success() {
        eprintln!("✓ Workspace builds successfully on MSRV {}", MSRV);
        Ok(())
    } else {
        Err(anyhow!(
            "Workspace failed to build on MSRV {}. Check above for compilation errors.",
            MSRV
        ))
    }
}
