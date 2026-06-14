# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project overview

`evidgap-rs` is a federated orchestration layer for biomedical evidence. It fans out queries across N evidence sources (PubMed, ChEMBL, ClinicalTrials.gov, etc.), resolves identities across vocabularies, and emits `KnowledgeState` + `GapMatrix` describing what is known, unknown, and which queries would close gaps.

**Load-bearing design principles** (from `ARCHITECTURE.md`):
1. **Ports model the domain, not the transport** — every port trait testable in-memory; CI enforces dual-adapter requirement (1 real, 1 fixture)
2. **Identity resolution is tri-state** — `Xref<T>` distinguishes `Resolved`, `NotFound`, and `Unattempted` (conflating the latter two is the most common gap-accounting failure mode)
3. **Provenance is mandatory** — facts cross port boundaries wrapped in `Sourced<T>` carrying connector, query, and timestamps; bare facts do not exist on the public surface
4. **Gap matrix is the deliverable** — prose summarization is downstream and optional; every cell of the matrix is falsifiable
5. **Apache Arrow on the local edge, MCP on the remote edge** — no third format on the contract surface
6. **Library-first, binary-last** — `evidgap-cli` is a thin orchestration over the public crate API

## Workspace structure

```
evidgap-rs/
├── crates/
│   ├── evidgap-id/                    # canonical IDs, Xref<T>, parse-don't-validate
│   ├── evidgap-prov/                  # Sourced<T>, Connector enum, temporal types
│   ├── evidgap-graph/                 # entities, relations, GapMatrix, KnowledgeState
│   ├── evidgap-ports/                 # port traits + fixture impls (feature: `fixtures`)
│   ├── evidgap-orchestrator/          # 5-phase pipeline engine
│   ├── evidgap-adapter-pubmed/        # MCP adapter for PubMed (v0.1.0)
│   ├── evidgap-adapter-chembl/        # REST adapter for ChEMBL (v0.1.0)
│   └── evidgap-adapter-clinicaltrials/# REST adapter for ClinicalTrials.gov (v0.1.0)
├── apps/
│   └── evidgap-cli/                   # binary orchestration frontend
├── xtask/                             # workspace utility crate (CI gate enforcement)
├── ARCHITECTURE.md                    # design and open questions
├── TODO.md                            # v0.1.0 → v0.3.0 roadmap with SemVer milestones
└── Cargo.toml                         # workspace manifest with shared deps + lints
```

**Four core crates move in lockstep** (through 0.x; may diverge post-1.0):
- `evidgap-id` — type-system anchors
- `evidgap-prov` — provenance carriers
- `evidgap-graph` — domain entities and gap matrix
- `evidgap-ports` — port surface + fixtures

**Adapters version independently** — a breaking change in `evidgap-adapter-chembl` does not force a major bump in `evidgap-adapter-pubmed`.

## The 5-phase pipeline

```
Anchor (disease, gene, compound, trial, outcome)
  ↓
[1] Resolve       — normalize anchor to canonical IDs across vocabularies
  ↓
[2] Fan out       — parallel queries to N ports via async adapters
  ↓
[3] Join          — cross-link results via Xref<T>; cycle detection via union-find
  ↓
[4] Score         — provenance weight + recency decay + study-design weighting
  ↓
[5] Synthesize    — emit KnowledgeState + GapMatrix (no prose)
```

See `ARCHITECTURE.md` § "The 5-phase pipeline" for phase responsibilities and open design questions.

## Common development tasks

### Build and test

```bash
# Full workspace check
cargo check --workspace

# Build all targets
cargo build --workspace

# Test with parallel runner (much faster than `cargo test`)
cargo nextest run --workspace

# Test single crate
cargo nextest run -p evidgap-orchestrator

# Test single test function
cargo nextest run -p evidgap-orchestrator test_gap_matrix_cell --exact

# Benchmarks (criterion)
cargo bench --workspace
```

### Code quality

```bash
# Format (idempotent)
cargo fmt --all

# Lint (clippy with all warnings as errors; CI gate)
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Spell check
typos

# License and advisory checks
cargo deny check advisories
cargo deny check licenses

# SemVer compliance (runs on release tag in CI)
cargo semver-checks
```

### Pre-commit checks (local development)

```bash
# One-shot: run all hooks
pre-commit run --all-files

# Install hooks (if using pre-commit framework)
pre-commit install

# Hooks run on every commit: fmt, clippy, typos, cargo deny
```

### Dependency health

```bash
# Detect unused dependencies
cargo machete

# RustSec advisory scan
cargo audit

# Unused dep compile-time check (nightly only; optional)
cargo +nightly udeps --all-targets
```

## Requirements

- **Rust 1.94.0+** (pinned in `rust-toolchain.toml`; verify with `rustup show`)
- **Edition:** 2024
- **Components:** `rustfmt`, `clippy`, `rust-src`, `rust-analyzer`
- **Dev tools** (listed in `CONTRIBUTING.md`):
  - `cargo-skill` — layered AI agent skill deployment (project policy)
  - `cargo-nextest` — parallel test runner
  - `cargo-deny` — license and advisory policy
  - `cargo-release` — workspace-aware SemVer release flow
  - `cargo-audit` — RustSec advisory checks
  - `cargo-semver-checks` — SemVer break detection
  - `cargo-machete` — unused dependency detection
  - `git-cliff` — changelog generation from Conventional Commits
  - `typos-cli` — spelling checks

Install with:
```bash
cargo install cargo-skill cargo-nextest cargo-deny cargo-release cargo-audit cargo-semver-checks cargo-machete git-cliff typos-cli
```

## Versioning and release

**Policy:** All crates SemVer-versioned independently, except:
- `evidgap-id`, `evidgap-prov`, `evidgap-graph`, `evidgap-ports` move in lockstep through 0.x (form one cohesive type-system surface)
- May diverge post-1.0

**Pre-1.0 breaking changes** are permitted in minor versions per SemVer §4; mark clearly in `CHANGELOG.md`.

**Release flow:**
1. Update crate `version` and `CHANGELOG.md` (Conventional Commits link)
2. Run `cargo semver-checks` locally
3. Tag with `<crate>-<version>` (e.g., `evidgap-orchestrator-0.1.0`)
4. CI runs `cargo semver-checks` and gates on success

See `CONTRIBUTING.md` for Conventional Commits format (`feat`, `fix`, `docs`, `chore`, `refactor`, `test`, `ci`, `perf`).

## Port traits and adapters

### Port design

Every port trait lives in `evidgap-ports` as an `async_trait`. Examples:

```rust
#[async_trait]
pub trait MechanismPort: Send + Sync {
    async fn compounds_for_target(&self, target: &UniProt)
        -> PortResult<Vec<Sourced<Compound>>>;
    async fn targets_for_compound(&self, compound: &Chembl)
        -> PortResult<Vec<Sourced<Target>>>;
}
```

### Dual-adapter requirement (CI gate)

**Every port trait must have at least two implementations:**
1. The real adapter (MCP client, REST wrapper, or Arrow scanner)
2. An in-memory `FixtureAdapter` behind the `fixtures` feature flag

`xtask check-ports` (to be implemented) enumerates all port traits and fails CI if any has fewer than two implementations. The fixture adapter is the falsifiability gate: if a port can be satisfied in-memory, Arrow can satisfy it later without transport leakage.

### Adapter responsibilities

Each adapter handles internally without leaking upward:
- Authentication (API keys, OAuth, MCP credentials)
- Rate limiting (token bucket; configurable at construction)
- Retry/backoff (exponential with jitter)
- Pagination cursors (collapsed into `Vec` or `Stream` per port cardinality)
- Schema mapping (connector-native → `Sourced<T>`)
- Error classification (`Permanent`, `Quota`, `Unauthorized`)

### Cardinality discipline

Decide at port-definition time:
- **`Vec<Sourced<T>>`** — small bounded result (compounds for a target, trials for a compound)
- **`impl Stream<Item = Sourced<T>>`** — unbounded or large result (publications matching a query, evidence rows)

v0.1.0 uses `Vec` exclusively (MCP transports return small bounded sets). `Stream` enters in v0.3.0 with Arrow adapters.

## Key types and concepts

### `evidgap-id::Xref<T>` — tri-state identity

```rust
pub enum Xref<T> {
    Resolved(T),        // mapped to canonical ID
    NotFound,           // queried, no mapping exists
    Unattempted,        // never queried for this layer
}
```

Never conflate `NotFound` and `Unattempted` — this distinction is load-bearing for gap accounting.

### `evidgap-prov::Sourced<T>` — mandatory provenance

```rust
pub struct Sourced<T> {
    pub value: T,
    pub connector: Connector,              // enum: Pubmed, Chembl, ClinicalTrials, …
    pub query: String,                     // verbatim query
    pub retrieved_at: DateTime<Utc>,       // when evidgap ran it
    pub published_at: Option<DateTime<Utc>>, // when source last updated
    pub source_version: Option<String>,    // e.g., ICD-10 revision
}
```

Bare facts do not cross port boundaries; all results are `Sourced<T>`.

### `evidgap-graph::Coverage<T>` — per-cell coverage

```rust
pub enum Coverage<T> {
    Empty,                    // queried, nothing returned
    Partial(Vec<Sourced<T>>), // some results, more available
    Full(Vec<Sourced<T>>),    // exhaustively retrieved within scope
}
```

Captured in `GapMatrix` per `(EntityType, Layer)` cell. A `KnowledgeState` may have `Full` mechanism coverage and `Empty` commercial coverage simultaneously.

## Testing

### Unit and integration tests

```bash
cargo nextest run -p evidgap-id

# With output capture
cargo nextest run -p evidgap-ports -- --nocapture
```

### Snapshot testing

[`insta`](https://insta.rs/) is used for snapshot assertions. Update snapshots with:
```bash
cargo insta test --workspace -- --unreleased
```

### Property testing

[`proptest`](https://docs.rs/proptest/) for property-based fuzz testing.

### Mock fixtures

[`mockito`](https://docs.rs/mockito/) for HTTP mocking in adapter tests. Fixture adapters in `evidgap-ports` (behind `features = ["fixtures"]`) provide in-memory port implementations for integration tests.

## Architectural decisions and constraints

**See `ARCHITECTURE.md` for:**
- [Open design questions](ARCHITECTURE.md#open-design-questions) (to resolve before v0.1.0 freeze)
- [Relationship to TRAPI / BioLink Model](ARCHITECTURE.md#relationship-to-trapi--biolink-model) (asymmetry and bridge strategy)
- [Port cardinality discipline](ARCHITECTURE.md#cardinality-discipline--vec-vs-stream) (`Vec` vs `Stream`)
- [Cycle detection in Phase 3](ARCHITECTURE.md#phase-3-join) (union-find prevents double-counting)
- [Scoring weights](ARCHITECTURE.md#phase-4-score) (provenance, recency, study design)

**See `TODO.md` for:**
- Roadmap with SemVer milestones (Phase 0 → v0.1.0 → v0.2.0 → v0.3.0 → v1.0.0)
- Deliverables per milestone
- Future crates (post-1.0 adoption surfaces like TRAPI export)

## Scope boundaries

`evidgap-rs` handles federation and orchestration of biomedical evidence metadata only.

**Out of scope:**
- Parsing biomedical data files → `clinical-rs`, `multiomics-rs`, `biomedref-rs`
- Clinical-coding semantics (ICD-10 hierarchy, ATC, LOINC) → `clinical-rs/medcodes`
- Raw sequencing formats (BAM, VCF, FASTQ) → `oxbow`, `noodles`
- Model training, inference, or NLP
- Knowledge-graph persistence (consumers may use Neo4j, RDF, Datalog, Arrow; workspace ships no backend)

**Sibling workspaces** (Arrow producers, consumed in v0.3.0+):
- [`clinical-rs`](https://github.com/SHA888/clinical-rs) — clinical records, code ontologies, task windowing
- [`multiomics-rs`](https://github.com/SHA888/multiomics-rs) — molecular references (UniProt, Reactome, Open Targets, …)
- [`biomedref-rs`](https://github.com/SHA888/biomedref-rs) — reference data outside molecular omics

## Debugging tips

### Tracing output

The workspace uses `tracing` for instrumentation. Enable at runtime:

```bash
RUST_LOG=evidgap_orchestrator=debug cargo run --bin evidgap
```

### Test debugging

```bash
# Run single test with backtrace and nocapture
RUST_BACKTRACE=1 cargo nextest run -p evidgap-orchestrator test_gap_matrix_cell -- --nocapture

# Insta review mode (interactive snapshot updates)
cargo insta review
```

### Port trait debugging

Add a temporary fixture adapter behind `#[cfg(test)]` to simulate a port response without hitting a live endpoint. Example in `evidgap-ports/src/fixtures.rs`.

## Commit messages and history

**No `Co-Authored-By:` trailers.** Commit attribution stays with the human author. Boilerplate trailers add noise without meaningful authorship value and are retroactively stripped. This applies to all AI-generated commits, including those produced by Claude Code or any other AI tool.

**English-only requirement** for tracked files:
- All Plans.md content must be in English (headers, table columns, task descriptions, status markers)
- No Japanese characters in Plans.md status markers (use `cc:done` instead of `cc:完了`, `cc:wip` instead of `cc:WIP`, etc.)
- All harness output and documentation must be in English

## License and compliance

- License: **Apache-2.0** only (no MIT/BSD; patent grant is load-bearing for federation touching drug discovery)
- Sibling workspaces dual-licensed MIT OR Apache-2.0; `evidgap-rs` consumes under Apache-2.0 leg
- Pre-commit hook: `cargo deny check advisories` gates on RustSec advisories
- Release gate: `cargo semver-checks` enforces SemVer compliance

## References

- **Main design:** `ARCHITECTURE.md` (load-bearing surface, open design questions, relationship to TRAPI)
- **Roadmap:** `TODO.md` (milestones, deliverables, phase gates)
- **Contributing:** `CONTRIBUTING.md` (dev setup, Conventional Commits, PR process)
- **README:** `README.md` (quick start, workspace layout, sibling workspace integrations)
