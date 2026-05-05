# Architecture

This document describes the load-bearing design of `evidgap-rs`. It is a working architecture, not a finished one. Sections marked **[OPEN]** require resolution before v0.1.0 freeze.

---

## Goals

1. **Federate** N live biomedical evidence sources behind one typed contract.
2. **Resolve identity** across vocabularies (MONDO ↔ MeSH ↔ HGNC ↔ UniProt ↔ ChEMBL ↔ NCT ↔ DOI) tri-state, never silently lossy. ICD-10 joins the resolve vocabulary in v0.2.0 alongside `evidgap-adapter-icd10`.
3. **Preserve provenance** on every fact crossing a port boundary.
4. **Account for gaps** as first-class output (`GapMatrix`), not as silence.
5. **Support both transports** — MCP (remote) and Apache Arrow (local) — through a single port surface, with no leakage of transport into the domain.
6. **Stay narrow.** Federation only. Parsing, modelling, and storage live elsewhere.

## Non-goals

- Parsing biomedical file formats. That lives in sibling workspaces.
- Knowledge-graph persistence. `KnowledgeState` is the wire type; consumers may persist however they wish.
- Model training, inference, or NLP. The orchestrator returns structured metadata; downstream consumers may apply LLMs, embeddings, or classical models.
- Real-time streaming or websocket protocols. Each `Orchestrator::run` is a bounded query.
- Authoritative truth claims. `evidgap-rs` reports what the connectors and reference databases said, with provenance and timestamp. It does not adjudicate.

---

## Type system

Three orthogonal concerns. Each lives in its own crate to keep cohesion high and coupling low.

### 1. Identity — `evidgap-id`

Every entity has a canonical newtype ID, parsed-not-validated at construction:

```rust
pub struct Mondo(String);   // "MONDO:0005113"
pub struct Hgnc(u32);       // 6018 (numeric)
pub struct Chembl(String);  // "CHEMBL25"
pub struct UniProt(String); // "P0DTC2"
pub struct Nct(String);     // "NCT12345678"
pub struct Doi(String);     // "10.1038/s41586-024-07000-0"
```

**Deferred to v0.2.0:** `Icd10<V: Icd10Version>` newtype with type-level version markers (`Icd10_2024`, `Icd10_2025`, …) ships with `evidgap-adapter-icd10` in v0.2.0. The design is recorded verbatim in `TODO.md` under "v0.2.0 — `evidgap-id` extension" so it lands without redesign. Shipping it earlier in v0.1.0 would be a YAGNI violation: with no v0.1.0 adapter consuming the type, the version-mixing guard has nothing to guard against. Structural types (`Sourced<T>`, `Xref<T>`, `Coverage<T>`) ship in v0.1.0 because every adapter touches them; leaf types (`Icd10`, `Rxnorm`, `Atc`, …) ship with their first consumer.

Cross-vocabulary identity uses `Xref<T>` — tri-state, never bivalent:

```rust
pub enum Xref<T> {
    Resolved(T),
    NotFound,    // queried, no mapping exists in source vocabulary
    Unattempted, // never queried for this layer
}
```

The tri-state distinction is load-bearing. Most federation systems collapse `NotFound` and `Unattempted` into `Option::None` and lose half the gap signal. `GapMatrix` cannot be produced without it.

`Xref<T>` is **not** the same concept as `medcodes::CrossMap` from `clinical-rs`. `CrossMap` is a code-system trait operating on opaque code strings (e.g., ICD-10-CM → CCS); `Xref<T>` is a typed result wrapping any canonical entity ID after a lookup attempt. They live at different layers and evidgap delegates code-system mapping to `medcodes` rather than re-implementing it. This distinction is repeated here because the naming proximity has caused confusion in design review.

### 2. Provenance — `evidgap-prov`

Facts wrap, never bare:

```rust
pub struct Sourced<T> {
    pub value: T,
    pub connector: Connector,           // enum: Pubmed, Chembl, ClinicalTrials, …
    pub query: String,                  // verbatim query that produced this fact
    pub retrieved_at: DateTime<Utc>,    // when evidgap ran the query
    pub published_at: Option<DateTime<Utc>>, // when the source last updated, if known
    pub source_version: Option<String>, // e.g., ICD-10 revision, ChEMBL release
}
```

Port trait signatures take and return `Sourced<T>` for any non-trivial result. Bare `T` does not cross a port boundary. This is enforced syntactically, not by convention.

`Connector` is an enum, not a string. Adding a connector requires a code change to `evidgap-prov`. This is a deliberate friction: every connector entering the system gets a code review of its scoring weight, rate-limit profile, and trust class before it ships.

### 3. Coverage — `evidgap-graph`

```rust
pub enum Coverage<T> {
    Empty,                       // queried, nothing returned
    Partial(Vec<Sourced<T>>),   // some results, more available
    Full(Vec<Sourced<T>>),      // exhaustively retrieved within scope
}

pub struct GapMatrix {
    cells: HashMap<(EntityType, Layer), Coverage<()>>,
    suggested_close_queries: HashMap<(EntityType, Layer), Vec<SuggestedQuery>>,
}

pub struct KnowledgeState {
    anchor: Anchor,
    diseases: Vec<Sourced<Disease>>,
    genes: Vec<Sourced<Gene>>,
    compounds: Vec<Sourced<Compound>>,
    targets: Vec<Sourced<Target>>,
    trials: Vec<Sourced<Trial>>,
    publications: Vec<Sourced<Publication>>,
    codes: Vec<Sourced<Code>>,
    relations: Vec<Sourced<Relation>>,
    gap_matrix: GapMatrix,
}
```

`Coverage<T>` is per-cell, not global. A `KnowledgeState` may have `Full` mechanism coverage and `Empty` commercial coverage simultaneously. Reports must surface this, not average it away.

---

## The 5-phase pipeline

```
Anchor
  │
  ▼
[1] Resolve         ── normalize anchor to canonical IDs across vocabularies
  │
  ▼
[2] Fan out         ── parallel queries to N layers via ports
  │
  ▼
[3] Join            ── cross-link results via Xref<T>
  │
  ▼
[4] Score           ── provenance + recency + study-design weighting
  │
  ▼
[5] Synthesize      ── emit KnowledgeState + GapMatrix
```

### Phase 1: Resolve

The hardest phase and the one most federation systems get wrong. Given an anchor (a free-text disease name, a partial gene symbol, an NCT ID, a compound name; ICD-10 codes added in v0.2.0 with `Anchor::Code`), produce canonical IDs across all relevant vocabularies. Without this, every downstream join leaks.

v0.1.0 implementation: synchronous lookup against an embedded `phf::Map` of common synonyms for the 7 canonical IDs, plus delegation to live MCP connectors for ambiguous cases. v0.2.0 may delegate to a dedicated ontology-resolver port if the embedded map proves insufficient.

**[OPEN]** Should resolve always succeed (returning `Xref::NotFound` where it can't), or fail fast (returning `Err`)? Current position: never fail. A failed resolution is a `GapMatrix` cell with `Unattempted`, not an `Err`. Failed resolution should not abort the pipeline.

### Phase 2: Fan out

Parallel queries across all configured ports. Each port returns `Sourced<T>` results within its layer. No cross-port communication during fan-out. Failures in one port do not abort others — a failed port produces a `GapMatrix` cell of `Empty` with the error preserved as a `SuggestedQuery::Retry`.

Concurrency: tokio + bounded `JoinSet`. Per-port rate limits live inside the adapter, not the orchestrator.

### Phase 3: Join

Results from different ports are cross-linked via `Xref<T>`. Example join chain (full, post-v0.2.0): Compound from ChEMBL → mechanism → target → UniProt → publications mentioning UniProt → trials testing compound → ICD-10 codes for trial conditions → AdisInsight pipeline entry. v0.1.0 supports the chain up through `trials testing compound`; the ICD-10 and commercial-pipeline tail enters with v0.2.0 adapters.

**Cycle detection is required.** AdisInsight cites trials, ChEMBL cites targets, PubMed cites everything. Naive aggregation double-counts evidence and inflates apparent consensus. Implementation: union-find on `(EntityType, CanonicalId)` pairs; duplicate edges are dropped before scoring.

**Prior art for triangle reconciliation:** Jesús Barrasa's "QuickGraph #19: Taxonomy reconciliation" (2021) demonstrates the canonical patterns for reconciling cross-references across MeSH, Wikidata, and DiseaseOntology in Neo4j with `n10s`/RDF — including triangle detection (perfect three-way matches), incomplete-triangle prediction (missing-leg detection for cross-reference inference), and granularity-mismatch detection (path-length disagreements between taxonomies). evidgap's Phase 3 implements the same patterns over the typed `Xref<T>` surface in pure Rust, no graph database required. Cited as design ancestry, not as a dependency.

### Phase 4: Score

Each `Sourced<T>` gets a quality score combining:

- **Provenance weight** — connector trust class (e.g., RCT in NEJM > preprint > forum)
- **Recency** — exponential decay with per-layer half-life (preprints decay fast; ICD-10 revisions do not)
- **Study design** — RCT > observational > in-vitro > computational, where extractable

**[OPEN]** Should scoring be a port (pluggable) or hardcoded? Current position: hardcoded for v0.1.0 with explicit weights documented in this file; promoted to a `ScoringPort` in v0.2.0 if a second consumer appears.

### Phase 5: Synthesize

Emit `KnowledgeState` and `GapMatrix`. No prose. Consumers may pipe to an LLM, a static report generator, or a UI. The orchestrator's contract ends here.

---

## Hexagonal — ports and adapters

Ports model **domain operations**, not transports. A port saying "find compounds active against this target" knows nothing about whether the implementation calls an MCP server or scans an Arrow batch. Adapters handle transport.

### Port traits — v0.1.0

```rust
#[async_trait]
pub trait MechanismPort: Send + Sync {
    async fn compounds_for_target(&self, target: &UniProt) -> PortResult<Vec<Sourced<Compound>>>;
    async fn targets_for_compound(&self, compound: &Chembl) -> PortResult<Vec<Sourced<Target>>>;
    async fn moa_for_compound(&self, compound: &Chembl) -> PortResult<Sourced<Option<MechanismOfAction>>>;
}

#[async_trait]
pub trait TrialPort: Send + Sync {
    async fn trials_for_disease(&self, disease: &Mondo) -> PortResult<Vec<Sourced<Trial>>>;
    async fn trials_for_compound(&self, compound: &Chembl) -> PortResult<Vec<Sourced<Trial>>>;
    async fn trial_details(&self, nct: &Nct) -> PortResult<Sourced<TrialDetail>>;
}

#[async_trait]
pub trait EvidencePort: Send + Sync {
    async fn publications_for_query(&self, query: &Query) -> PortResult<Vec<Sourced<Publication>>>;
    async fn publication_metadata(&self, doi: &Doi) -> PortResult<Sourced<PublicationMetadata>>;
}
```

### Cardinality discipline — `Vec` vs `Stream`

For each port method, expected result cardinality is decided at port-definition time:

- **`Vec<Sourced<T>>`** when the domain operation naturally returns a small bounded set (compounds for a single target, trials for one compound, MoA for one compound).
- **`impl Stream<Item = Sourced<T>>`** when the domain operation naturally returns an unbounded or large set (publications matching a free-text query, evidence rows for a disease in Open Targets).

Wrong choices here become breaking changes. Each port method's cardinality is documented in rustdoc and in this file before implementation. v0.1.0 ports use `Vec` exclusively because the live MCP transports return small bounded sets per call. `Stream` ports enter in v0.3.0 with Arrow adapters.

### CI gate — dual adapter requirement

**Every port trait must have at least two adapter implementations:**

1. The real adapter (MCP, or Arrow scan in v0.3.0+)
2. An in-memory `FixtureAdapter` shipped under `evidgap-ports`'s `fixtures` feature flag

The CI script enumerates every port trait via `syn` parsing of `evidgap-ports/src/**/*.rs`, scans the workspace for `impl PortTrait for ...` blocks, and fails the build if any port has fewer than two implementations. Cultural conventions are not conventions — they are wishes. This rule is encoded as `xtask check-ports` in CI.

The fixture adapter is the falsifiability gate for the "ports model the domain, not the transport" claim. If a port can be satisfied by an in-memory `HashMap`, an Arrow scan can satisfy it later. If it cannot, the port has leaked transport.

### Adapter responsibilities

Each adapter handles, internally, without leaking upward:

- Authentication (API keys, OAuth, MCP credentials)
- Rate limiting (token bucket per adapter; configurable per construction)
- Retry / backoff (exponential with jitter; bounded retries)
- Pagination cursors (collapsed into `Stream` or `Vec` per port cardinality)
- Schema mapping (from connector-native shape to `Sourced<T>`)
- Error classification — transient vs permanent — surfacing only the latter as `PortError::Permanent`

`PortError` is a bounded enum:

```rust
pub enum PortError {
    Permanent(anyhow::Error),  // unrecoverable; gap matrix records Empty
    Quota,                     // rate limit hit; orchestrator backs off
    Unauthorized,              // credential failure; orchestrator surfaces to caller
}
```

---

## Workspace structure

```
evidgap-rs/
├── crates/
│   ├── evidgap-id/                    # canonical IDs, Xref<T>, parse-don't-validate
│   ├── evidgap-prov/                  # Sourced<T>, Connector enum, temporal types
│   ├── evidgap-graph/                 # entities, relations, GapMatrix, KnowledgeState
│   ├── evidgap-ports/                 # port traits + fixture impls (feature: `fixtures`)
│   ├── evidgap-orchestrator/          # 5-phase pipeline engine
│   ├── evidgap-adapter-pubmed/        # v0.1.0
│   ├── evidgap-adapter-chembl/        # v0.1.0
│   └── evidgap-adapter-clinicaltrials/# v0.1.0
├── apps/
│   └── evidgap-cli/                   # binary, last
├── xtask/                             # workspace tasks (CI gate enforcement)
├── ARCHITECTURE.md
├── TODO.md
├── LICENSE-APACHE
└── Cargo.toml
```

Each adapter is its own crate. Per-adapter dependency surfaces (HTTP clients, MCP client libraries, ChEMBL SDKs) differ enough that a single feature-flagged adapters crate would balloon the dependency closure for any consumer using one connector.

`xtask` is the home of CI gate scripts (port enumeration, dual-adapter check, semver-checks orchestration).

---

## Versioning policy

- All crates SemVer-versioned **independently** following the `clinical-rs` and `multiomics-rs` precedent.
- A breaking change in `evidgap-adapter-chembl` does not force a major bump in `evidgap-adapter-pubmed`.
- The four core crates (`evidgap-id`, `evidgap-prov`, `evidgap-graph`, `evidgap-ports`) move in lockstep through 0.x because they form one cohesive type-system surface; they may diverge after v1.0.
- `cargo-semver-checks` runs in CI on every release tag. SemVer violations fail the release pipeline.
- Pre-1.0 (0.x.y) breaking changes are permitted in minor versions per SemVer §4, with `CHANGELOG.md` entries marking them clearly.

---

## Sibling workspace integration — v0.2.0+

`evidgap-rs` consumes parsed Apache Arrow data from sibling workspaces via dedicated adapters in v0.3.0+. None of this is in v0.1.0.

| evidgap layer    | sibling source                                               | adapter (v0.3.0)                  |
|------------------|--------------------------------------------------------------|-----------------------------------|
| Mechanism        | `multiomics-rs/open-targets-rs` evidence Parquet             | `evidgap-adapter-opentargets`     |
| Druggability     | `multiomics-rs/dgidb-rs` drug-gene Arrow                     | `evidgap-adapter-dgidb`           |
| Target           | `multiomics-rs/uniprot-rs` Arrow                             | `evidgap-adapter-uniprot`         |
| Pathway          | `multiomics-rs/reactome-rs` Arrow                            | `evidgap-adapter-reactome`        |
| Coding           | `clinical-rs/medcodes` `phf` lookup                          | `evidgap-adapter-medcodes`        |
| Literature aux   | `biomedref-rs/jensenlab-textmining-rs` (when published)      | `evidgap-adapter-jensenlab`       |

Critical principle: when both a live and a local adapter exist for the same layer (e.g., ChEMBL via MCP and Open Targets via Arrow for mechanism), they implement the **same port trait**. The orchestrator may use either, both (with provenance distinguishing them), or fall back from one to the other. The user-facing API does not know which transport satisfied the query.

### External Arrow sources

In addition to internal sibling workspaces (`clinical-rs`, `multiomics-rs`, `biomedref-rs`, all under `SHA888`), evidgap may consume Apache Parquet from external biomedical KG projects that distribute pre-built graphs in compatible formats. These are tracked separately because their schema, license, and update cadence are not under our control.

| External source                                                                                            | Format                                       | Adapter (post-1.0)              | Notes                                                                                                                                                                                          |
|------------------------------------------------------------------------------------------------------------|----------------------------------------------|---------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| [OptimusKG](https://github.com/mims-harvard/OptimusKG) (Zitnik Lab, *Nature Scientific Data* 2026)         | Apache Parquet (gold layer), Polars/NetworkX | `evidgap-adapter-optimuskg`     | 65 harmonized sources, 10 entity types, 26 relations, BioCypher + BioLink Model alignment. Their schema doubles as a worked-example reference for our TRAPI export mapping (see below). |

The OptimusKG integration is doubly load-bearing:

1. **As a data source**, it provides a pre-harmonized cross-section of biomedical knowledge that would otherwise take years to reproduce — a strong v0.3.0+ adapter target for breadth coverage.
2. **As a mapping reference**, their BioLink Model alignment for 10 entity types and 26 relation predicates is essentially a worked example of what `evidgap-export-trapi` will need. We borrow the validated mappings rather than redesigning them. This drops the v0.1.0 TRAPI export cost estimate from 6–8 weeks to roughly 4–6 weeks of focused work.

License caveat: OptimusKG's code is MIT but the integrated data carries source-specific licenses (some academic-only, some restricting commercial use). evidgap's adapter must surface these license constraints at runtime for each consumed subset; the adapter's `Sourced<T>` provenance fields will carry per-source-license metadata to make this enforceable downstream.

---

## Relationship to TRAPI / BioLink Model

NCATS Biomedical Data Translator hit Initial Public Release in 2025 with TRAPI (Translator Reasoner API) as its OpenAPI-based query/response standard and BioLink Model as its typed domain ontology. evidgap-rs is positioned as a complement to that ecosystem, not a competitor or a subset.

**The asymmetry is the point.** TRAPI and BioLink Model encode *positive evidence assertions* — subject + predicate + object triples representing what is known. evidgap encodes both what is known **and what isn't**: `Xref::Unattempted`, `Xref::NotFound`, and `Coverage::Empty` represent the absence of inquiry and the absence of evidence respectively. These states have no representation in TRAPI or BioLink, by their design and ours. The two ecosystems make different epistemic commitments and produce different outputs.

**Bridge, not subset.** A post-1.0 `evidgap-export-trapi` crate will serialize the *positive-evidence subset* of `KnowledgeState` into TRAPI-compliant JSON for Translator KP-mode integration. Native serialization remains canonical for full-fidelity gap-aware consumption.

**v0.1.0 stance:** The internal type system is intentionally narrower than BioLink (7 canonical IDs, named entity types, named relations) for ergonomic and compile-time-safety reasons. BioLink alignment is an export concern, not a domain-modelling concern.

**What TRAPI export costs (post-1.0):** roughly 4–6 weeks of focused work — TRAPI envelope serialization, BioLink Model 4.x category and predicate mapping for the 6 cleanly-mappable entities (Disease/Gene/Compound/Target/Trial/Publication), `Sourced<T>` → BioLink association attribute mapping, `reasoner-validator` integration in CI, and BioLink Model version tracking infrastructure. (Down from an initial 6–8 week estimate; OptimusKG's Vittor et al. 2026 BioLink alignment provides a worked-example mapping reference — see "External Arrow sources" above.) The `Code` entity is a CURIE-shaped pass-through with no clinical-coding semantics owned by evidgap, so its TRAPI mapping is trivial; rich code-system semantics live in `clinical-rs/medcodes`, which will own its own TRAPI-or-not decision separately.

**KP-mode, not ARA-mode.** evidgap exports its knowledge for ARAs to query; it does not aim to *be* an ARA (federated query planning across multiple KPs is an order-of-magnitude larger scope, well-served by existing Translator components like BioThings Explorer).

---

## Naming and concept disambiguation

A short reference for terms that are easy to confuse:

- **`evidgap_id::Xref<T>`** — typed tri-state result of an entity-identity lookup across vocabularies. Lives in evidgap.
- **`medcodes::CrossMap`** — trait in `clinical-rs/medcodes` for code-system cross-mapping (e.g., ICD-10-CM → CCS). String-typed, code-domain-only.
- **Crossref.org** — DOI registration agency; a future adapter may consume DOI metadata from it. Will be named `evidgap-adapter-crossref` if added. Distinct from `Xref<T>`.
- **BioLink Model** — NCATS Translator's typed domain ontology. evidgap-rs's internal type system is intentionally narrower than BioLink for v0.1.0 ergonomics and compile-time safety. BioLink alignment is an export concern (post-1.0 `evidgap-export-trapi`), not a domain-modelling concern. See "Relationship to TRAPI / BioLink Model" above.

---

## Open design questions

Each must be resolved before v0.1.0 freeze. Tracked in TODO.md against specific deliverables.

1. **Resolve phase failure mode** — never fail vs. fail fast? Current position: never fail.
2. **Scoring as port vs. hardcoded** — pluggable in v0.1.0 or v0.2.0? Current position: hardcoded for v0.1.0.
3. **Anchor as enum vs. trait** — `Anchor::Disease(Mondo)` vs. trait-object `Box<dyn Anchorable>`? Current position: enum, with `Custom(Box<dyn Anchorable>)` reserved as escape hatch.
4. **Async runtime commitment** — tokio-only vs. runtime-agnostic via `async_trait`? Current position: tokio-only for v0.1.0; revisit at v1.0 if a second runtime appears in the ecosystem with material adoption.
5. **Caching** — orchestrator-level or adapter-level? Current position: adapter-level only; orchestrator stays stateless across runs.
6. **MSRV** — track `clinical-rs` (1.94) or set independently? Current position: track `clinical-rs` since evidgap v0.3.0 will depend on `medcodes`.

---

## What this is not, restated

- Not a knowledge graph database. `KnowledgeState` is the wire type; persistence is the consumer's choice.
- Not a clinical decision support system. Outputs are evidence summaries with provenance, not recommendations.
- Not a research-grade ontology aligner. v0.1.0 covers seven canonical IDs; richer ontology alignment is downstream.
- Not a substitute for primary-source review. The `GapMatrix` is intended to direct human attention, not replace it.
