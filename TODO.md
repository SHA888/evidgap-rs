# Roadmap

Comprehensive checklist organized by phase and [SemVer](https://semver.org/) release milestone. Each item is a concrete deliverable.

> **Versioning policy:** Each crate versions independently. A breaking change in `evidgap-adapter-pubmed` does not force a major bump in `evidgap-adapter-chembl`. The four core crates (`evidgap-id`, `evidgap-prov`, `evidgap-graph`, `evidgap-ports`) move in lockstep through 0.x because they form one cohesive type-system surface. Pre-1.0 (0.x.y) releases may contain breaking changes in minor versions per SemVer §4.

---

## Table of contents

- [ ] [Phase 0 — Project Bootstrap](#phase-0--project-bootstrap)
- [ ] [v0.1.0 — Type system + 3-layer MVP](#v010--type-system--3-layer-mvp)
- [ ] [v0.2.0 — 6-layer expansion](#v020--6-layer-expansion)
- [ ] [v0.3.0 — Local Arrow adapters](#v030--local-arrow-adapters)
- [ ] [v1.0.0 — Stable API](#v100--stable-api)
- [ ] [Post-1.0 adoption surfaces](#post-10-adoption-surfaces)
- [ ] [Future crates (post-1.0)](#future-crates-post-10)

---

## Phase 0 — Project Bootstrap

Everything needed before writing the first line of library code. This phase produces a fully configured, CI-protected, release-ready Cargo workspace with zero library functionality.

### 0.0.1 — Toolchain & environment

- [x] **Rust toolchain**
  - [x] `rust-toolchain.toml` pinning latest stable channel (≥ 1.94.0, current MSRV)
  - [x] Components: `rustfmt`, `clippy`, `rust-src`, `rust-analyzer`
  - [x] Edition: 2024
  - [x] Verify: `rustup show` matches on a fresh clone
- [x] **Dev tools** (documented in `CONTRIBUTING.md`)
  - [x] `cargo-skill` — layered AI agent skill deployment (mandated for every Rust workspace per project policy)
  - [x] `cargo-nextest` — parallel test runner with better output than `cargo test`
  - [x] `cargo-deny` — license audit, advisory DB, dependency policy
  - [x] `cargo-release` — workspace-aware SemVer release flow
  - [x] `cargo-audit` — security advisory checking against RustSec
  - [x] `cargo-semver-checks` — SemVer violation detection on every release
  - [x] `cargo-machete` — detect unused dependencies
  - [x] `cargo-udeps` (nightly only, optional) — compile-time unused-dep detection
  - [x] `git-cliff` — Conventional Commits changelog generation
  - [x] `typos-cli` — spelling check across docs and code
- [x] **Git configuration**
  - [x] `.gitignore`: `target/`, `data/`, `*.parquet`, `*.arrow`, `*.ipc`, `.env`, `*.csv.gz`, `coverage/`
  - [x] `.gitattributes`: `*.rs diff=rust`, LF line endings enforced
  - [x] Conventional Commits enforced via `CONTRIBUTING.md` (feat/fix/docs/chore/refactor/test/ci/perf)
  - [x] `.pre-commit-config.yaml`: `cargo fmt --check`, `cargo clippy -D warnings`, `typos`, `cargo deny check advisories`

### 0.0.2 — Workspace manifest

- [x] **Root `Cargo.toml`**
  - [x] `[workspace]` with `resolver = "2"`, `members = ["crates/*", "apps/*", "xtask"]`
  - [x] `[workspace.package]` with `edition`, `rust-version`, `license = "Apache-2.0"`, `repository`, `homepage`, `authors`, `categories`, `keywords`
  - [x] `[workspace.dependencies]` pinning every shared dep to the latest stable minor version (tokio, async-trait, thiserror, anyhow, serde, serde_json, chrono, uuid, phf, reqwest, futures, tracing, tracing-subscriber, criterion, proptest, insta, tempfile, mockito)
  - [x] `[workspace.lints.rust]` and `[workspace.lints.clippy]` (see 0.0.4)
- [x] **License declaration** — `Apache-2.0` only across the workspace; no dual-license combinations
- [x] Verify: `cargo check --workspace` passes with empty `lib.rs` stubs
- [x] Verify: `cargo fmt --all -- --check` passes
- [x] Verify: `cargo clippy --workspace -- -D warnings` passes

### 0.0.3 — Crate scaffolding (empty shells)

Each crate gets a publishable-but-empty skeleton with `version = "0.0.0"` (placeholder, unpublished).

- [ ] **`crates/evidgap-id/`** — newtypes, `Xref<T>`, parse-don't-validate
  - [ ] `Cargo.toml` (no_std capable; default features off; `serde` feature flag)
  - [ ] `src/lib.rs` with module stubs for each canonical ID
  - [ ] `README.md`, `CHANGELOG.md` (initialized with `## [Unreleased]`)
- [ ] **`crates/evidgap-prov/`** — `Sourced<T>`, `Connector` enum, temporal types
  - [ ] `Cargo.toml` (depends on `chrono`, `thiserror`)
  - [ ] `src/lib.rs` with stub types
  - [ ] `README.md`, `CHANGELOG.md`
- [ ] **`crates/evidgap-graph/`** — entities, relations, `GapMatrix`, `KnowledgeState`
  - [ ] `Cargo.toml` (depends on `evidgap-id`, `evidgap-prov`)
  - [ ] `src/lib.rs` with stub types
  - [ ] `README.md`, `CHANGELOG.md`
- [ ] **`crates/evidgap-ports/`** — port traits + fixture impls
  - [ ] `Cargo.toml` (depends on `evidgap-id`, `evidgap-prov`, `evidgap-graph`, `async-trait`; `fixtures` feature flag)
  - [ ] `src/lib.rs` with stub traits
  - [ ] `src/fixtures.rs` (gated behind `fixtures` feature)
  - [ ] `README.md`, `CHANGELOG.md`
- [ ] **`crates/evidgap-orchestrator/`** — 5-phase engine
  - [ ] `Cargo.toml` (depends on `evidgap-{id,prov,graph,ports}`, `tokio`, `futures`, `tracing`)
  - [ ] `src/lib.rs` with stub `Orchestrator`
  - [ ] `README.md`, `CHANGELOG.md`
- [ ] **`crates/evidgap-adapter-pubmed/`**, **`crates/evidgap-adapter-chembl/`**, **`crates/evidgap-adapter-clinicaltrials/`**
  - [ ] One `Cargo.toml` each (depends on `evidgap-ports`, `reqwest` or MCP client library, `tokio`, `serde`)
  - [ ] One `src/lib.rs` each
  - [ ] One `README.md`, `CHANGELOG.md` each
- [ ] **`apps/evidgap-cli/`** — binary
  - [ ] `Cargo.toml` with `[[bin]] name = "evidgap"`
  - [ ] `src/main.rs` with empty CLI scaffold
- [ ] **`xtask/`** — workspace utility crate
  - [ ] `Cargo.toml` (binary; depends on `syn`, `glob`, `anyhow`)
  - [ ] `src/main.rs` with subcommand stubs (see 0.0.5)
- [ ] **Final check:** `cargo check --workspace` compiles all crates

### 0.0.4 — Code quality configuration

- [ ] **`rustfmt.toml`**
  - [ ] `edition = "2024"`, `max_width = 100`, `use_field_init_shorthand = true`, `use_try_shorthand = true`, `imports_granularity = "Crate"`, `group_imports = "StdExternalCrate"`
- [ ] **Workspace-level lints** in root `Cargo.toml`
  - [ ] `[workspace.lints.rust]`: `unsafe_code = "forbid"`, `missing_docs = "warn"`, `unused_results = "warn"`, `rust_2018_idioms = "warn"`, `rust_2024_compatibility = "warn"`
  - [ ] `[workspace.lints.clippy]`: `all = { level = "warn", priority = -1 }`, `pedantic = { level = "warn", priority = -1 }`, `nursery = { level = "warn", priority = -1 }`, `unwrap_used = "warn"`, `expect_used = "warn"`, `panic = "warn"`, `todo = "warn"`, `dbg_macro = "warn"`
  - [ ] Each crate inherits via `[lints] workspace = true`
- [ ] **`deny.toml`** (cargo-deny config)
  - [ ] `[advisories]`: `vulnerability = "deny"`, `unmaintained = "warn"`, `yanked = "warn"`
  - [ ] `[licenses]`: `allow = ["Apache-2.0", "MIT", "BSD-2-Clause", "BSD-3-Clause", "ISC", "Unicode-3.0", "Zlib", "CC0-1.0"]`, `confidence-threshold = 0.93`
  - [ ] `[bans]`: `multiple-versions = "warn"`, `wildcards = "deny"`
  - [ ] `[sources]`: `unknown-registry = "deny"`, `unknown-git = "deny"`, `allow-registry = ["https://github.com/rust-lang/crates.io-index"]`
- [ ] Verify: `cargo deny check` passes on empty workspace

### 0.0.5 — `xtask` — workspace governance scripts

The `xtask` crate is the home of every CI gate that cannot be expressed in stock cargo subcommands. Per project policy, principles without CI enforcement are decoration.

- [ ] **`xtask check-ports`** — enumerate port traits and verify dual-adapter rule
  - [ ] Parse `crates/evidgap-ports/src/**/*.rs` via `syn`
  - [ ] Collect every `#[async_trait] pub trait *Port` definition
  - [ ] Scan workspace for `impl <Trait> for <Type>` blocks
  - [ ] For each port trait, require `>= 2` impls (one fixture, one real)
  - [ ] Exit non-zero if any port has fewer
- [ ] **`xtask check-cardinality`** — verify port method cardinality is documented
  - [ ] Each port method must have `/// # Cardinality` rustdoc section noting `Vec` or `Stream` and rationale
  - [ ] Parse rustdoc, fail if missing
- [ ] **`xtask check-provenance`** — verify port methods return `Sourced<_>` not bare `T`
  - [ ] Parse return types; fail on bare entity types crossing port boundary
- [ ] **`xtask check-msrv`** — install MSRV toolchain, run `cargo check --workspace`
- [ ] **`xtask audit`** — wraps `cargo audit` and `cargo deny check`
- [ ] **`xtask coverage`** — wraps `cargo llvm-cov` for unified workspace coverage
- [ ] **`xtask release-dry-run`** — invokes `cargo release --dry-run` workspace-wide

### 0.0.6 — CI / CD pipeline (GitHub Actions)

- [ ] **`.github/workflows/ci.yml`** — runs on every push and PR
  - [ ] Matrix: `ubuntu-latest` (primary), `macos-latest`, `windows-latest`; fail-fast: false
  - [ ] Steps:
    - [ ] `rustup` install with `rust-toolchain.toml`
    - [ ] `cargo fmt --all -- --check`
    - [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings`
    - [ ] `cargo nextest run --workspace --all-features`
    - [ ] `cargo doc --workspace --no-deps --all-features` (verify docs build cleanly with `RUSTDOCFLAGS=-D warnings`)
    - [ ] `cargo deny check`
    - [ ] `cargo run -p xtask -- check-ports`
    - [ ] `cargo run -p xtask -- check-cardinality`
    - [ ] `cargo run -p xtask -- check-provenance`
  - [ ] Rust cache via `Swatinem/rust-cache@v2`
- [ ] **`.github/workflows/release.yml`** — runs on `v*` tag push
  - [ ] Determine which crate(s) changed via `cargo release` heuristics
  - [ ] Run `cargo semver-checks check-release` against the previous published version
  - [ ] `cargo publish -p <crate>` with `CARGO_REGISTRY_TOKEN` secret
  - [ ] Create GitHub Release with changelog generated by `git-cliff`
- [ ] **`.github/workflows/audit.yml`** — scheduled nightly
  - [ ] `cargo audit` against RustSec advisory DB
  - [ ] Open issue on vulnerability found
- [ ] **`.github/workflows/msrv.yml`** — weekly
  - [ ] Install MSRV toolchain, run `cargo run -p xtask -- check-msrv`
- [ ] **`.github/workflows/coverage.yml`** — runs on `main` push
  - [ ] `cargo run -p xtask -- coverage`, upload to Codecov or equivalent
- [ ] **Branch protection rules** (manual, in GitHub settings)
  - [ ] `main`: require PR, require CI green, require 1 approval (when collaborators exist), no force push

### 0.0.7 — Repository files

- [ ] **`LICENSE-APACHE`** — Apache 2.0 full text with current year and `Kresna Sucandra` as copyright holder
- [ ] **`CONTRIBUTING.md`**
  - [ ] Development setup: clone, `rustup`, tool installs from 0.0.1 list
  - [ ] Conventional Commits format
  - [ ] PR process: fork → branch → PR → CI green → 1 approval → merge
  - [ ] How to add a new port trait (must include fixture impl)
  - [ ] How to add a new adapter (must include rate-limit profile, error classification, two passing tests)
  - [ ] How to add a new canonical ID newtype
  - [ ] Code style: `rustfmt.toml`, satisfy clippy pedantic
- [ ] **`SECURITY.md`**
  - [ ] Responsible disclosure policy
  - [ ] Contact: GitHub security advisory
  - [ ] Scope: identity-resolution correctness bugs treated as security-severity (wrong cross-references in clinical context can mislead research)
- [ ] **`CODE_OF_CONDUCT.md`** — Contributor Covenant v2.1
- [ ] **`.github/ISSUE_TEMPLATE/`**
  - [ ] `bug_report.md` — repro, expected vs. actual, crate + version
  - [ ] `feature_request.md` — use case, proposed API, which crate
  - [ ] `port_proposal.md` — new port trait proposal with cardinality, provenance shape, fixture sketch
- [ ] **`.github/PULL_REQUEST_TEMPLATE.md`**
  - [ ] Checklist: tests added, docs updated, CHANGELOG entry, `cargo fmt`, `cargo clippy`, `xtask` checks pass, port additions include fixture impl

### 0.0.8 — Release infrastructure

- [ ] **`cliff.toml`** (git-cliff)
  - [ ] Conventional Commits parsing
  - [ ] Group by: feat, fix, perf, refactor, docs, test, ci, chore
  - [ ] Per-crate changelog generation (filter commits by path `crates/<name>`)
  - [ ] Template: Keep a Changelog format
- [ ] **`release.toml`** (cargo-release)
  - [ ] Per-crate tags: `evidgap-id-v0.1.0`, `evidgap-adapter-pubmed-v0.1.0`, etc.
  - [ ] Pre-release hook: `cargo run -p xtask -- release-dry-run`
- [ ] **Dry-run test:** `cargo release patch --workspace --dry-run` completes without error

### 0.0.9 — Verification checkpoint

All of the following must pass on a fresh `git clone`:

- [ ] `cargo check --workspace --all-features` — compiles
- [ ] `cargo fmt --all -- --check` — formatted
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings` — no warnings
- [ ] `cargo nextest run --workspace --all-features` — tests pass (trivially, since none yet)
- [ ] `cargo doc --workspace --no-deps --all-features` — docs build with `-D warnings`
- [ ] `cargo deny check` — license + advisory clean
- [ ] `cargo run -p xtask -- check-ports` — passes (trivially, no ports yet)
- [ ] `cargo run -p xtask -- check-cardinality` — passes (trivially)
- [ ] `cargo run -p xtask -- check-provenance` — passes (trivially)
- [ ] `cargo release patch --workspace --dry-run` — release flow works
- [ ] GitHub Actions CI green on `main`
- [ ] README, ARCHITECTURE, TODO render correctly on GitHub

**Phase 0 is complete when a contributor can clone, build, test, lint, and dry-run a release with zero manual setup beyond `rustup` and the documented tool installs.**

---

## v0.1.0 — Type system + 3-layer MVP

The first published release. Establishes the type-system surface that every subsequent version preserves through SemVer. Three live MCP adapters: PubMed, ChEMBL, ClinicalTrials.gov.

### `evidgap-id` — v0.1.0

- [ ] **Canonical ID newtypes** — parse-don't-validate constructors
  - [ ] `Mondo` — MONDO disease ontology, validates `MONDO:\d{7}` format
  - [ ] `Hgnc` — HGNC gene ID, numeric u32 with optional `HGNC:` prefix on parse
  - [ ] `Chembl` — ChEMBL compound ID, validates `CHEMBL\d+` format
  - [ ] `UniProt` — UniProt accession, validates `[OPQ][0-9][A-Z0-9]{3}[0-9]` or `[A-NR-Z][0-9]([A-Z][A-Z0-9]{2}[0-9]){1,2}`
  - [ ] `Nct` — ClinicalTrials.gov NCT ID, validates `NCT\d{8}` format
  - [ ] `Doi` — DOI, validates `10\.\d{4,9}/.+` format with permissive suffix
  - [ ] **(Deferred to v0.2.0)** `Icd10<V: Icd10Version>` — version-parameterized ICD-10 code; ships with `evidgap-adapter-icd10`. Leaf types ship with their first consumer.
- [ ] **`Xref<T>` enum** — tri-state cross-reference
  - [ ] `Resolved(T)`, `NotFound`, `Unattempted` variants
  - [ ] `is_resolved()`, `is_attempted()`, `as_option()` accessors
  - [ ] `From<Option<T>>` deliberately **not** implemented — forces explicit choice between `NotFound` and `Unattempted` at every conversion site
- [ ] **Trait `CanonicalId`** — common surface across newtypes
  - [ ] `fn parse(s: &str) -> Result<Self, IdError>`
  - [ ] `fn as_str(&self) -> &str`
  - [ ] `fn id_type() -> EntityType` (associated constant)
- [ ] **Tests**
  - [ ] Unit tests for every parse method (valid, invalid, edge cases)
  - [ ] Property tests (`proptest`): round-trip parse/display, parse idempotence
  - [ ] Snapshot tests (`insta`): error messages stable across versions
- [ ] **`serde` feature flag** — opt-in `Serialize`/`Deserialize` for every newtype
- [ ] **`no_std` compatibility** — verified, no allocator-required dependencies in default features
- [ ] **Documentation**
  - [ ] Rustdoc for every public item with at least one example
  - [ ] Crate-level doc explaining `Xref<T>` tri-state design
  - [ ] `README.md` with usage example
- [ ] **Release**
  - [ ] Bump `0.0.0` → `0.1.0`, publish to crates.io

### `evidgap-prov` — v0.1.0

- [ ] **`Sourced<T>` struct** with fields: `value`, `connector`, `query`, `retrieved_at`, `published_at`, `source_version`
  - [ ] Constructor takes all fields explicitly; no defaults
  - [ ] Accessors return references where appropriate
  - [ ] `map<U>(self, f: FnOnce(T) -> U) -> Sourced<U>` for value transformation preserving provenance
- [ ] **`Connector` enum** — closed enum of supported connectors
  - [ ] v0.1.0 variants: `Pubmed`, `Chembl`, `ClinicalTrials`
  - [ ] Adding a connector is a breaking change in `evidgap-prov` (deliberate friction)
- [ ] **Trust class** — every `Connector` variant maps to a `TrustClass` (e.g., `PeerReviewedIndex`, `RegulatoryRegistry`, `BioactivityDatabase`)
- [ ] **`Temporal` newtype** wrapping `chrono::DateTime<Utc>` with helpers for half-life calculations
- [ ] **Tests** — provenance preservation across `map`, serialization round-trip with `serde` feature
- [ ] **Documentation + release** — rustdoc, README, CHANGELOG, publish

### `evidgap-graph` — v0.1.0

- [ ] **Entity types**
  - [ ] `Disease`, `Gene`, `Compound`, `Target`, `Trial`, `Publication` structs
  - [ ] Each carries its canonical ID + a list of `Xref<OtherId>` for cross-vocabulary linkage
  - [ ] Each implements `Entity` trait with `id_type()` and `canonical_id()` accessors
  - [ ] **(Deferred to v0.2.0)** `Code` struct ships with `evidgap-adapter-icd10`; `EntityType::Code` variant added then
- [ ] **Relations**
  - [ ] `Relation` enum: `CompoundTarget`, `TrialCompound`, `TrialDisease`, `PublicationEntity`, etc.
  - [ ] Each relation wraps `Sourced<RelationDetail>` to preserve provenance
  - [ ] **(Deferred to v0.2.0)** `CodeDisease` and other code-bearing relation variants ship alongside `Code` entity
- [ ] **`Coverage<T>` enum** — `Empty`, `Partial(Vec<Sourced<T>>)`, `Full(Vec<Sourced<T>>)`
- [ ] **`GapMatrix`**
  - [ ] `cells: HashMap<(EntityType, Layer), Coverage<()>>`
  - [ ] `suggested_close_queries: HashMap<(EntityType, Layer), Vec<SuggestedQuery>>`
  - [ ] `Display` impl producing a human-readable table
- [ ] **`KnowledgeState`**
  - [ ] All entity collections + `relations` + `gap_matrix`
  - [ ] Builder pattern for orchestrator construction
  - [ ] `merge` method for combining states from multiple anchors (returns `Result` on conflict)
- [ ] **Tests** — known-shape construction, gap-matrix display snapshot, merge conflict handling
- [ ] **Documentation + release**

### `evidgap-ports` — v0.1.0

- [ ] **`MechanismPort` trait**
  - [ ] `compounds_for_target(target: &UniProt) -> Vec<Sourced<Compound>>`
  - [ ] `targets_for_compound(compound: &Chembl) -> Vec<Sourced<Target>>`
  - [ ] `moa_for_compound(compound: &Chembl) -> Sourced<Option<MechanismOfAction>>`
  - [ ] All methods documented with `# Cardinality` and `# Errors` rustdoc sections
- [ ] **`TrialPort` trait**
  - [ ] `trials_for_disease(disease: &Mondo) -> Vec<Sourced<Trial>>`
  - [ ] `trials_for_compound(compound: &Chembl) -> Vec<Sourced<Trial>>`
  - [ ] `trial_details(nct: &Nct) -> Sourced<TrialDetail>`
- [ ] **`EvidencePort` trait**
  - [ ] `publications_for_query(query: &Query) -> Vec<Sourced<Publication>>`
  - [ ] `publication_metadata(doi: &Doi) -> Sourced<PublicationMetadata>`
- [ ] **`PortError` enum** — `Permanent`, `Quota`, `Unauthorized`
- [ ] **`PortResult<T>` type alias** — `Result<T, PortError>`
- [ ] **Fixture adapters** (behind `fixtures` feature flag)
  - [ ] `MechanismFixture`, `TrialFixture`, `EvidenceFixture` — each backed by `HashMap`
  - [ ] Builder pattern: `MechanismFixture::builder().add_compound(target, compound).build()`
  - [ ] Used by every adapter crate's integration tests
- [ ] **Tests** — fixture round-trip, every fixture satisfies its port trait
- [ ] **Documentation + release**

### `evidgap-adapter-pubmed` — v0.1.0

- [ ] **MCP client integration** — connect to PubMed MCP endpoint
- [ ] **`PubmedAdapter` struct implementing `EvidencePort`**
- [ ] **Rate limiting** — token bucket (configurable; default conservative)
- [ ] **Retry logic** — exponential backoff with jitter, max 3 retries on `Quota`
- [ ] **Schema mapping** — PubMed response → `Sourced<Publication>` with `Connector::Pubmed`, captured query, retrieval timestamp, PubMed last-updated as `published_at`
- [ ] **Tests**
  - [ ] Unit: schema mapping with frozen response fixtures
  - [ ] Integration: against `EvidenceFixture` to verify trait conformance
  - [ ] Live integration test (gated behind env var) — single sanity query
- [ ] **Documentation + release**

### `evidgap-adapter-chembl` — v0.1.0

- [ ] **MCP client integration** for ChEMBL connector
- [ ] **`ChemblAdapter` struct implementing `MechanismPort`**
- [ ] **Rate limiting**, **retry logic**, **schema mapping** as in PubMed adapter
- [ ] **Tests** — unit, integration against fixture, gated live test
- [ ] **Documentation + release**

### `evidgap-adapter-clinicaltrials` — v0.1.0

- [ ] **MCP client integration** for ClinicalTrials.gov connector
- [ ] **`ClinicalTrialsAdapter` struct implementing `TrialPort`**
- [ ] **Rate limiting**, **retry logic**, **schema mapping**
- [ ] **Tests** — unit, integration against fixture, gated live test
- [ ] **Documentation + release**

### `evidgap-orchestrator` — v0.1.0

- [ ] **`Orchestrator` struct + `OrchestratorBuilder`**
  - [ ] Builder accepts one adapter per port trait
  - [ ] `build()` requires all v0.1.0 ports to be set; missing ports fail at build time, not run time
- [ ] **`Anchor` enum** — `Disease(Mondo)`, `Gene(Hgnc)`, `Compound(Chembl)`, `Target(UniProt)`, `Trial(Nct)`, `Publication(Doi)`. **(Deferred to v0.2.0)** `Code(Box<dyn Icd10Erased>)` variant ships with `evidgap-adapter-icd10`. The enum is `#[non_exhaustive]` from v0.1.0 so adding the variant in v0.2.0 is non-breaking for external matchers.
- [ ] **5-phase pipeline**
  - [ ] **Resolve** — embedded `phf::Map` of common synonyms; falls back to `Xref::Unattempted` for unrecognized inputs
  - [ ] **Fan out** — `tokio::JoinSet` over configured ports; per-port `PortError` recorded into `GapMatrix`
  - [ ] **Join** — union-find on `(EntityType, CanonicalId)` for cycle detection; cross-link via `Xref<T>`
  - [ ] **Score** — hardcoded weights documented in `ARCHITECTURE.md` (provenance + recency + study-design)
  - [ ] **Synthesize** — emit `KnowledgeState`
- [ ] **`run(anchor) -> Result<KnowledgeState, OrchestratorError>`**
- [ ] **Tracing** — instrument every phase with `tracing` spans; per-port latency captured
- [ ] **Tests**
  - [ ] End-to-end against fixture adapters: known anchor → known `KnowledgeState`
  - [ ] Cycle detection: synthetic graph with cycles → no double-counting
  - [ ] Partial failure: one port returns `PortError::Permanent` → `GapMatrix` records `Empty`, others succeed
- [ ] **Documentation + release**

### `evidgap-cli` — v0.1.0

- [ ] **CLI binary `evidgap`** built on `clap` v4
- [ ] Subcommands:
  - [ ] `evidgap resolve <input>` — phase 1 only, prints resolved canonical IDs
  - [ ] `evidgap run <input>` — full pipeline, prints `KnowledgeState` as JSON
  - [ ] `evidgap gap <input>` — runs pipeline, prints `GapMatrix` only as a table
  - [ ] `evidgap ports` — list configured ports and their adapter types
- [ ] **Configuration** via `~/.config/evidgap/config.toml` (MCP credentials, rate-limit overrides)
- [ ] **Output formats** — JSON (default), pretty-text, NDJSON (one `Sourced<_>` per line for piping)
- [ ] **Tests** — `assert_cmd` snapshot tests for each subcommand
- [ ] **Documentation** — `evidgap --help` rendered into `apps/evidgap-cli/README.md`
- [ ] **Release**

### v0.1.0 release verification

- [ ] All v0.1.0 crates published to crates.io
- [ ] `cargo install evidgap-cli` works on a fresh machine and runs `evidgap resolve "sepsis"` successfully
- [ ] All `xtask` checks pass on tag commit
- [ ] `cargo semver-checks check-release` returns clean for every crate

---

## v0.2.0 — 6-layer expansion

Five additional MCP adapters. Scoring promoted to a port. Gap-matrix human-readable export. ICD-10 type-system extension lands here, gated by its first consumer (`evidgap-adapter-icd10`).

### `evidgap-id` extension — `Icd10` newtype

Preconditional for `evidgap-adapter-icd10`. Bumps `evidgap-id` to v0.2.0.

- [ ] **`Icd10<V: Icd10Version>`** — version-parameterized newtype
  - [ ] `code: String` (validated against ICD-10 format) + `_v: PhantomData<V>`
  - [ ] Type-level versioning prevents the most common clinical-coding error: silently mixing code revisions across years
- [ ] **`Icd10Version` trait** — `pub trait Icd10Version: 'static {}`
- [ ] **Marker types** — `Icd10_2024`, `Icd10_2025` initial set; one new marker per supported revision year
- [ ] **Version conversions** — explicit `From` or fallible conversion between `Icd10<Icd10_2024>` and `Icd10<Icd10_2025>`; never implicit
- [ ] **`CanonicalId` impl** — same surface as v0.1.0 newtypes
- [ ] **Tests** — parse, version-mixing prevention compile-fail tests, round-trip
- [ ] **`Icd10Erased` trait** — type-erasing wrapper for runtime polymorphism (used by `Anchor::Code`)
- [ ] **Note** — rich code-system semantics (hierarchy, billable status, cross-system mapping to CCS/ATC/LOINC) live in `clinical-rs/medcodes`, not here. evidgap's `Icd10` is a CURIE-shaped pass-through with version safety.

### `evidgap-graph` extension — `Code` entity + `EntityType::Code`

Preconditional for `evidgap-adapter-icd10`. Bumps `evidgap-graph` to v0.2.0.

- [ ] **`Code` struct** — carries `Icd10<V>` (or future code-system newtypes) + `Xref<OtherId>` list
- [ ] **`EntityType::Code` variant** — added to enum (non-breaking because `EntityType` is `#[non_exhaustive]`)
- [ ] **`Relation` variants** — `CodeDisease` and other code-bearing relations as required by `CodingPort`
- [ ] **`KnowledgeState::codes` field** — `Vec<Sourced<Code>>` collection added

### `evidgap-orchestrator` extension — `Anchor::Code` variant

Preconditional for `evidgap-adapter-icd10`. Bumps `evidgap-orchestrator` to v0.2.0.

- [ ] **`Anchor::Code(Box<dyn Icd10Erased>)`** variant added (non-breaking via `#[non_exhaustive]` from v0.1.0)
- [ ] **Resolve phase** extension — accepts free-text ICD-10 codes; canonicalizes through `medcodes` validator if available, otherwise via the live ICD-10 adapter

### Five new adapters

- [ ] `evidgap-adapter-icd10` implementing a new `CodingPort` — depends on the three extensions above (`evidgap-id`, `evidgap-graph`, `evidgap-orchestrator`); the adapter cannot ship before the type-system extensions land
- [ ] `evidgap-adapter-biorxiv` implementing `EvidencePort` (alongside PubMed; orchestrator uses both)
- [ ] `evidgap-adapter-scholargateway` implementing `EvidencePort`
- [ ] `evidgap-adapter-synthesizebio` implementing a new `ExpressionPort`
- [ ] `evidgap-adapter-adisinsight` implementing a new `CommercialPort`

Each follows the v0.1.0 adapter template: MCP client + port impl + rate limit + retry + schema mapping + unit + integration + gated-live tests + docs + CHANGELOG.

### Multi-adapter-per-port semantics

- [ ] `Orchestrator` builder accepts `Vec<Box<dyn EvidencePort>>` for multi-adapter ports
- [ ] Fan-out queries all adapters in parallel; results merged with provenance distinguishing them
- [ ] `GapMatrix::cells` keyed on `(EntityType, Layer, ConnectorOpt)` to surface per-connector coverage

### Scoring port

- [ ] `ScoringPort` trait extracted from hardcoded weights
- [ ] `DefaultScorer` ships in `evidgap-orchestrator` preserving v0.1.0 behavior
- [ ] Custom scorers pluggable; CI gate requires `DefaultScorer` + at least one fixture scorer

### Gap-matrix human export

- [ ] `GapMatrix::to_markdown()` — rendered table for human reports
- [ ] `GapMatrix::to_json()` — structured for downstream tooling
- [ ] `evidgap gap --format markdown <input>` — CLI subcommand

### v0.2.0 verification

- [ ] All adapters publish; orchestrator handles 6 layers + 8 connectors
- [ ] Multi-adapter integration tests for `EvidencePort` (PubMed + bioRxiv + Scholar Gateway)
- [ ] Performance: full 8-connector run for one anchor completes in < 30s (median, against live MCP)

---

## v0.3.0 — Local Arrow adapters

Consume Arrow data emitted by sibling workspaces. Same port traits, different transport.

### Sibling workspace integration

- [ ] **`evidgap-adapter-medcodes`** — consumes `clinical-rs/medcodes` for `CodingPort` offline
- [ ] **`evidgap-adapter-uniprot`** — consumes `multiomics-rs/uniprot-rs` for target lookups
- [ ] **`evidgap-adapter-reactome`** — consumes `multiomics-rs/reactome-rs` for new `PathwayPort`
- [ ] **`evidgap-adapter-opentargets`** — consumes `multiomics-rs/open-targets-rs` Parquet for `MechanismPort` offline (alongside ChEMBL via MCP)
- [ ] **`evidgap-adapter-dgidb`** — consumes `multiomics-rs/dgidb-rs` for druggability layer
- [ ] **`evidgap-adapter-stringdb`** — consumes `multiomics-rs/string-rs` for protein-protein neighborhood expansion (new `InteractionPort`)
- [ ] **`evidgap-adapter-jensenlab`** — consumes `biomedref-rs/jensenlab-textmining-rs` (when published) as `EvidencePort` aux

### External Arrow source — OptimusKG (candidate for v0.3.0 or post-1.0)

- [ ] **`evidgap-adapter-optimuskg`** — consumes [OptimusKG](https://github.com/mims-harvard/OptimusKG) gold-layer Parquet (Zitnik Lab, Nature Scientific Data 2026)
  - [ ] Fetch via OptimusKG Python client (`pip install optimuskg`) invoked from a build script, or direct Harvard Dataverse fetch (DOI: `10.7910/DVN/IYNGEV`)
  - [ ] Map their 10 entity types to evidgap's 7 canonical IDs (gene, drug, disease, protein, etc.) — mapping table maintained against OptimusKG release
  - [ ] Map their 26 relation types to evidgap's `Relation` enum
  - [ ] Per-cell `Sourced<T>::source_version` carries the OptimusKG release tag (e.g., `optimuskg-0.70.9`)
  - [ ] **License-aware provenance**: OptimusKG's code is MIT but the integrated data carries source-specific licenses (some academic-only, some restricting commercial use). The adapter's `Sourced<T>` extends with a `license_constraint: Option<LicenseConstraint>` field per consumed record so downstream consumers can enforce license filtering. New `LicenseConstraint` enum lives in `evidgap-prov`
  - [ ] **Implementation choice — v0.3.0 vs post-1.0**: ship in v0.3.0 if license-aware provenance design lands cleanly; otherwise defer to post-1.0 where it ships alongside the TRAPI export adapter (the BioLink mappings are shared work)

### Sibling-workspace vs external-source distinction

Sibling workspaces (`clinical-rs`, `multiomics-rs`, `biomedref-rs`) are under our governance — same author, same conventions, coordinated SemVer. External sources (OptimusKG and any future external Parquet exporters) are not. The architectural rule: **external-source adapters ship behind feature flags, default off**, so consumers must opt in explicitly. License-constraint provenance is mandatory on external-source adapters, optional on sibling-workspace adapters.

### Arrow integration discipline

- [ ] Each Arrow adapter takes `RecordBatchReader` or `&Path` to Parquet at construction
- [ ] Streaming where cardinality warrants — adds `Stream`-returning port methods if the live transport's `Vec` shape proves too restrictive
- [ ] Same `Sourced<T>` provenance: `Connector::OpenTargets`, `source_version` derived from Open Targets release file
- [ ] Cardinality re-evaluation: any port method changing from `Vec` to `Stream` is a breaking change → bumps the port crate to v0.2.0

### Two-transport orchestration

- [ ] Orchestrator detects when both live and local adapters exist for a port; default policy: prefer local, fall back to live on `Empty`; configurable per port
- [ ] `Sourced<T>` carries enough metadata that consumers can distinguish transports if needed

### v0.3.0 verification

- [ ] All sibling workspaces declared as optional `path` deps initially, switched to `version` deps once published
- [ ] Full test coverage with synthetic Arrow fixtures (no real data in repo)
- [ ] Any required port-trait changes documented as breaking and bumped per SemVer

---

## v1.0.0 — Stable API

API freeze. Every public surface reviewed and committed-to.

- [ ] **Public API review**
  - [ ] Every public type, trait, and method audited
  - [ ] Sealed traits applied to types not intended for external impl
  - [ ] `#[non_exhaustive]` applied to enums likely to grow (`Connector`, `EntityType`, `Layer`, `PortError`)
- [ ] **MSRV policy** — documented commitment (e.g., "MSRV bumps allowed in minor versions with one prior release of warning")
- [ ] **Migration guide** from 0.x → 1.0
- [ ] **Performance budget** — documented latency and memory targets per phase, enforced by criterion benchmarks in CI
- [ ] **Default features finalized** per crate
- [ ] **Deprecations cleared** — anything marked deprecated in 0.x is removed
- [ ] **`cargo-semver-checks`** clean against the last 0.x release as the comparison baseline
- [ ] **Citation** — Zenodo DOI assigned for the v1.0.0 release

---

## Post-1.0 adoption surfaces

These are not feature additions but **bridges into adjacent ecosystems**. Each export adapter takes the canonical `KnowledgeState` produced by the orchestrator and serializes it into another ecosystem's preferred format. The native serialization remains canonical for full-fidelity gap-aware consumption; export adapters are subset views by definition.

Sequencing is demand-driven: an export adapter ships when the first downstream consumer asks for it, not before. The list below is a roadmap of _known_ potential bridges, not a commitment to ship all of them.

### `evidgap-export-trapi` — bridge to NCATS Translator ecosystem

Serializes the **positive-evidence subset** of `KnowledgeState` into TRAPI-compliant JSON for Translator KP-mode integration.

- [ ] **TRAPI envelope** — `message` containing `query_graph`, `knowledge_graph`, and `results`; OpenAPI spec compliance verified by `NCATSTranslator/reasoner-validator`
- [ ] **BioLink Model 4.x mapping** — for the 6 cleanly-mappable entities:
  - [ ] `Disease(Mondo)` → `biolink:Disease`
  - [ ] `Gene(Hgnc)` → `biolink:Gene`
  - [ ] `Compound(Chembl)` → `biolink:ChemicalEntity` / `biolink:Drug`
  - [ ] `Target(UniProt)` → `biolink:Protein`
  - [ ] `Trial(Nct)` → `biolink:ClinicalTrial`
  - [ ] `Publication(Doi)` → `biolink:Publication`
- [ ] **`Code(Icd10)` mapping** — CURIE pass-through; rich semantics owned by `clinical-rs/medcodes` and not in scope here
- [ ] **Predicate hierarchy** — every `Relation` enum variant mapped to a BioLink predicate; mapping table maintained against BioLink Model SemVer
- [ ] **Provenance mapping** — `Sourced<T>` fields → BioLink association attributes (`primary_knowledge_source`, `supporting_publications`, `retrieval_source`, etc.)
- [ ] **`Coverage::Empty` and `Xref::Unattempted`** — emitted to TRAPI `logs` workflow field for diagnostic visibility; they are not first-class in TRAPI by design
- [ ] **CI gate** — `reasoner-validator` runs on every CI build over a representative `KnowledgeState` fixture
- [ ] **BioLink Model version tracking** — embedded reference to a pinned BioLink release; minor version bumps in BioLink land in CHANGELOG with explicit predicate-hierarchy diff
- [ ] **Dependencies** — `evidgap-graph`, `evidgap-orchestrator`
- [ ] **Estimated cost** — 4–6 weeks of focused work for v0.1.0 of this adapter, **down from an initial estimate of 6–8 weeks** because OptimusKG's BioLink Model alignment for 10 entity types and 26 relation predicates (Vittor et al., _Nature Scientific Data_ 2026) provides a worked-example mapping reference; the adapter borrows their validated mappings rather than designing from scratch

### `evidgap-export-eppi` — bridge to EPPI-Mapper

Serializes `KnowledgeState` and `GapMatrix` into EPPI-Reviewer JSON format consumable by EPPI-Mapper for evidence-gap-map visualization.

- [ ] **EPPI-Reviewer JSON schema** — investigate current schema version; emit conformant export
- [ ] **`GapMatrix` cells** → EPPI-Mapper rows/columns/cells; this is the natural home for our gap-aware view since EPPI-Mapper is one of the few tools that understands gap-as-data
- [ ] **Per-cell `SuggestedQuery`** → EPPI cell annotations
- [ ] **Dependencies** — `evidgap-graph`
- [ ] **Estimated cost** — 2–3 weeks; smaller scope than TRAPI because EPPI does not require ontology alignment

### `evidgap-export-eviatlas` — bridge to EviAtlas

CSV/JSON export consumable by EviAtlas (R Shiny app) for evidence-synthesis visualization.

- [ ] **Tabular evidence base format** — emit columns matching EviAtlas's expected schema
- [ ] **Dependencies** — `evidgap-graph`
- [ ] **Estimated cost** — 1–2 weeks

### `evidgap-export-graphdb` — Neo4j / RDF persistence

Persists `KnowledgeState` as a property graph (Neo4j) or RDF triples for consumers who want to query the federated result with Cypher / SPARQL.

- [ ] **Neo4j export** — produce `CREATE`/`MERGE` Cypher script or use `neo4j-rs` driver
- [ ] **RDF export** — Turtle / N-Triples; entities use canonical CURIEs as IRIs
- [ ] **Schema layering** — pluggable schema (BioLink-aligned is one option; native evidgap schema is another)
- [ ] **Dependencies** — `evidgap-graph`
- [ ] **Estimated cost** — 3–4 weeks

### `evidgap-mcp-server` — expose evidgap itself as an MCP server

Wraps `evidgap-orchestrator` as an MCP server so other Claude / LLM tooling can query gap-aware federated evidence as a tool.

- [ ] **MCP protocol implementation** — server-side
- [ ] **Tool surface** — `run_anchor`, `gap_matrix`, `resolve` operations
- [ ] **Dependencies** — `evidgap-orchestrator`
- [ ] **Estimated cost** — 2–3 weeks

### Adoption-surface summary

| Bridge                    | Target ecosystem             | What survives the bridge                                               | Cost (focused work)                      |
| ------------------------- | ---------------------------- | ---------------------------------------------------------------------- | ---------------------------------------- |
| `evidgap-export-trapi`    | NCATS Translator (KP-mode)   | Positive-evidence subset; gap matrix folded into `logs` only           | 4–6 weeks (with OptimusKG mapping reuse) |
| `evidgap-export-eppi`     | EPPI-Mapper                  | Full gap matrix; EPPI is gap-aware                                     | 2–3 weeks                                |
| `evidgap-export-eviatlas` | EviAtlas                     | Tabular evidence base; gap signal partial                              | 1–2 weeks                                |
| `evidgap-export-graphdb`  | Neo4j / RDF stores           | Full `KnowledgeState`; gap matrix as separate sub-graph or named graph | 3–4 weeks                                |
| `evidgap-mcp-server`      | LLM tooling (Claude, others) | Full surface, query-by-query                                           | 2–3 weeks                                |

---

## Future crates (post-1.0)

Tracked for roadmap visibility. Will not block any 1.0 release.

| Crate                       | Purpose                                                | Depends on      |
| --------------------------- | ------------------------------------------------------ | --------------- |
| `evidgap-adapter-crossref`  | DOI metadata enrichment from Crossref.org              | `evidgap-ports` |
| `evidgap-adapter-europepmc` | Europe PMC literature index (alternative to PubMed)    | `evidgap-ports` |
| `evidgap-adapter-rxnorm`    | RxNorm drug nomenclature                               | `evidgap-ports` |
| `evidgap-adapter-mesh`      | MeSH literature indexing terms                         | `evidgap-ports` |
| `evidgap-adapter-hpo`       | Human Phenotype Ontology                               | `evidgap-ports` |
| `evidgap-adapter-omim`      | OMIM Mendelian disease catalog                         | `evidgap-ports` |
| `evidgap-adapter-biorex`    | BioREx-extracted relations as `EvidencePort` aux input | `evidgap-ports` |
| `evidgap-cache`             | Optional caching layer (sled or rocksdb backend)       | `evidgap-ports` |

---

## Release sequence

```
Phase 0 (bootstrap)
    │
    ▼
v0.1.0
    ├─► evidgap-id           (foundation, no internal deps)
    ├─► evidgap-prov         (foundation, no internal deps)
    ├─► evidgap-graph        (depends on id + prov)
    ├─► evidgap-ports        (depends on id + prov + graph)
    ├─► evidgap-adapter-pubmed
    ├─► evidgap-adapter-chembl
    ├─► evidgap-adapter-clinicaltrials
    ├─► evidgap-orchestrator (depends on all the above)
    └─► evidgap-cli          (binary, last)
    │
    ▼
v0.2.0  ── 5 new adapters, scoring port, multi-adapter semantics
    │
    ▼
v0.3.0  ── local Arrow adapters via sibling workspaces
    │
    ▼
API review → v1.0.0 across all crates
    │
    ▼
Post-1.0 ── adoption-surface bridges (TRAPI, EPPI, EviAtlas, graphdb, MCP server)
            shipped demand-driven, not in lockstep
```

Within each release, items are ordered by implementation priority (top = first).
