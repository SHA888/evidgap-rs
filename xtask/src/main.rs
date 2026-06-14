use anyhow::Result;
use std::env;

mod check_audit;
mod check_cardinality;
mod check_msrv;
mod check_ports;
mod check_provenance;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    let subcommand = &args[1];

    match subcommand.as_str() {
        "check-ports" => check_ports::check_ports(),
        "check-cardinality" => check_cardinality::check_cardinality(),
        "check-provenance" => check_provenance::check_provenance(),
        "check-msrv" => check_msrv::check_msrv(),
        "audit" => check_audit::check_audit(),
        _ => {
            eprintln!("Unknown subcommand: {}", subcommand);
            print_usage();
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    eprintln!("xtask — workspace governance scripts");
    eprintln!();
    eprintln!("Usage: xtask <SUBCOMMAND>");
    eprintln!();
    eprintln!("Subcommands:");
    eprintln!("  check-ports       Verify each port trait has >= 2 implementations");
    eprintln!("  check-cardinality Verify port methods have cardinality docs");
    eprintln!("  check-provenance  Verify port methods return Sourced<T>");
    eprintln!("  check-msrv        Check MSRV build (1.94.0)");
    eprintln!("  audit             Run cargo audit + cargo deny advisories check");
    eprintln!("  coverage          Generate coverage report (TODO)");
    eprintln!("  release-dry-run   Dry-run release with cargo release (TODO)");
}
