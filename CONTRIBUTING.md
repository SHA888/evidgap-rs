# Contributing to evidgap-rs

Thanks for contributing to `evidgap-rs`.

## Development setup

### 1) Clone and enter the repository

- `git clone https://github.com/SHA888/evidgap-rs.git`
- `cd evidgap-rs`

### 2) Install Rust toolchain

This repository pins Rust in `rust-toolchain.toml`:

- Channel: `1.94.0`
- Edition: `2024`
- Components: `rustfmt`, `clippy`, `rust-src`, `rust-analyzer`

### 3) Install required developer tools

Every contributor should install the following:

- `cargo-skill` — layered AI agent skill deployment (project policy)
- `cargo-nextest` — parallel test runner
- `cargo-deny` — license and advisory policy checks
- `cargo-release` — workspace-aware release flow
- `cargo-audit` — RustSec advisory checks
- `cargo-semver-checks` — SemVer break detection
- `cargo-machete` — unused dependency detection
- `cargo-udeps` (optional; nightly only) — compile-time unused dependency checks
- `git-cliff` — changelog generation from Conventional Commits
- `typos-cli` — spelling checks

## Conventional Commits

Commits must follow Conventional Commits and use one of:

- `feat`
- `fix`
- `docs`
- `chore`
- `refactor`
- `test`
- `ci`
- `perf`

Examples:

- `feat(orchestrator): add gap-matrix markdown formatter`
- `fix(adapter-pubmed): handle empty author list`
- `docs(readme): clarify roadmap scope`

## Pre-commit

This repository provides `.pre-commit-config.yaml` with these checks:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `typos`
- `cargo deny check advisories`

If you use pre-commit framework, install it and run:

- `pre-commit install`
- `pre-commit run --all-files`

## Pull request process

1. Fork the repository.
2. Create a topic branch.
3. Make your changes with tests/docs as needed.
4. Ensure local checks pass.
5. Open a PR and wait for CI to pass.
