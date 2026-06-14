use anyhow::Result;
use std::env;

mod check_ports;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    let subcommand = &args[1];

    match subcommand.as_str() {
        "check-ports" => check_ports::check_ports(),
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
    eprintln!("  check-cardinality Verify port methods have cardinality docs (TODO)");
    eprintln!("  check-provenance  Verify port methods return Sourced<T> (TODO)");
    eprintln!("  check-msrv        Check MSRV build (TODO)");
    eprintln!("  audit             Run cargo audit + cargo deny (TODO)");
    eprintln!("  coverage          Generate coverage report (TODO)");
    eprintln!("  release-dry-run   Dry-run release with cargo release (TODO)");
}
