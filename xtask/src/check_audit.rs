use anyhow::Result;
use std::process::Command;

/// Verify workspace passes both cargo audit and cargo deny advisories checks.
/// Runs both tools to aggregate all security findings in a single gate.
pub fn check_audit() -> Result<()> {
    eprintln!("📋 Auditing workspace for security advisories...");

    // Step 0: Check if cargo-audit is available
    eprintln!("\n  • Checking for cargo-audit...");
    let audit_check = Command::new("cargo").args(["audit", "--version"]).output();
    if audit_check.is_err() || !audit_check.unwrap().status.success() {
        return Err(anyhow::anyhow!(
            "cargo-audit not found (install: cargo install cargo-audit)"
        ));
    }

    // Step 1: Run cargo audit
    eprintln!("  • Running 'cargo audit'...");
    let audit_output = Command::new("cargo")
        .arg("audit")
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to run cargo audit: {}", e))?;

    let audit_success = audit_output.status.success();
    let audit_stdout = String::from_utf8_lossy(&audit_output.stdout);
    let audit_stderr = String::from_utf8_lossy(&audit_output.stderr);

    // Step 2: Run cargo deny check
    eprintln!("  • Running 'cargo deny check advisories'...");
    let deny_output = Command::new("cargo")
        .args(["deny", "check", "advisories"])
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to run cargo deny: {}", e))?;

    let deny_success = deny_output.status.success();
    let deny_stdout = String::from_utf8_lossy(&deny_output.stdout);
    let deny_stderr = String::from_utf8_lossy(&deny_output.stderr);

    // Step 3: Report results
    let mut all_passed = true;

    if audit_success {
        eprintln!("✓ cargo audit: no advisories found");
    } else {
        all_passed = false;
        eprintln!("\n❌ cargo audit found advisories:");
        eprint!("{}", audit_stdout);
        if !audit_stderr.is_empty() {
            eprint!("{}", audit_stderr);
        }
    }

    if deny_success {
        eprintln!("✓ cargo deny check advisories: passed");
    } else {
        all_passed = false;
        eprintln!("\n❌ cargo deny check advisories found issues:");
        eprint!("{}", deny_stdout);
        if !deny_stderr.is_empty() {
            eprint!("{}", deny_stderr);
        }
    }

    if all_passed {
        eprintln!("\n✓ Workspace security audit passed");
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Workspace has unresolved security advisories. Address above issues and re-run."
        ))
    }
}
