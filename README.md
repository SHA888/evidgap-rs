# evidgap-rs

**Federation and orchestration layer for biomedical evidence — Apache Arrow as the local contract, MCP as the remote contract.**

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE-APACHE)
[![Rust](https://img.shields.io/badge/rust-1.94%2B-orange.svg)](https://www.rust-lang.org)

[Architecture](ARCHITECTURE.md) · [Roadmap](TODO.md) · [Contributing](CONTRIBUTING.md)

---

## What is this?

`evidgap-rs` is a Cargo workspace that federates live biomedical evidence sources behind a single typed contract.

Given a biomedical anchor (a disease, a gene, a compound, a clinical trial, an outcome measure), `evidgap` fans out queries across N evidence layers — mechanism, druggability, trial, literature, commercial pipeline, clinical coding — joins the results through a typed identity-resolution layer, scores them by provenance and temporal recency, and emits a `KnowledgeState` plus a `GapMatrix` describing what is known, what is unknown, and what query would close each gap.

The output is structured. Prose synthesis is downstream and optional.

`evidgap-rs` is not a parser, not a model, and not a knowledge graph. It is the orchestration layer above all three.

## Why?

Biomedical evidence is fragmented across registries, indices, preprint servers, commercial pipelines, and ontologies. Answering a single research question — *"what is known about JAK1 inhibitors in sepsis?"* — currently requires manual traversal of PubMed, ClinicalTrials.gov, ChEMBL, Open Targets, AdisInsight, ICD-10 indices, and several preprint servers. Each step loses provenance. None of the steps share an identity model. Most of the steps don't talk to each other at all.

Existing federation work — NCATS Biomedical Data Translator (2025 Initial Public Release), Open Targets, PubTator Central, BioThings Explorer — has solved the *positive-evidence* side of this problem. They federate, type, and assert *what is known*. `evidgap-rs` makes a different epistemic commitment: it encodes both what is known **and what isn't**. Identity resolution is tri-state (`Resolved` / `NotFound` / `Unattempted`), coverage is per-cell (`Empty` / `Partial` / `Full`), and the `GapMatrix` surfaces the absence of evidence as first-class output with a suggested query for every gap.

Positive-evidence federation tools cannot represent this — TRAPI and the BioLink Model encode assertions, not their absence — so `evidgap-rs` is **complementary** to those ecosystems, not a replacement. Native serialization carries the full gap-aware view; export adapters bridge the positive-evidence subset into adjacent ecosystems (TRAPI for Translator KP-mode integration, EPPI-Mapper / EviAtlas for systematic-review visualization). The workspace ships as a typed Rust library consumable both online (via MCP connectors) and offline (via Apache Arrow reference databases from `clinical-rs`, `multiomics-rs`, and `biomedref-rs`).

## Status

Pre-release. The type system and port traits are the load-bearing surface; v0.1.0 freezes them and ships three live adapters (PubMed, ChEMBL, ClinicalTrials.gov). Earlier 0.0.x releases are exploratory and may break the surface.

## Quick start

```toml
# Cargo.toml — once published
[dependencies]
evidgap-orchestrator           = "0.1"
evidgap-adapter-pubmed         = "0.1"
evidgap-adapter-chembl         = "0.1"
evidgap-adapter-clinicaltrials = "0.1"
```

```rust
use evidgap_id::Mondo;
use evidgap_orchestrator::{Anchor, Orchestrator};

let orch = Orchestrator::builder()
    .with_pubmed(pubmed_adapter)
    .with_chembl(chembl_adapter)
    .with_clinicaltrials(clinicaltrials_adapter)
    .build();

let anchor = Anchor::Disease(Mondo::parse("MONDO:0005113")?);  // sepsis
let state  = orch.run(anchor).await?;

println!("{}", state.gap_matrix());
for sourced in state.publications() { /* ... */ }
```

## Workspace layout

```
evidgap-rs/
├── crates/
│   ├── evidgap-id                     # canonical ID newtypes + Xref<T>
│   ├── evidgap-prov                   # Sourced<T>, temporal, connector enum
│   ├── evidgap-graph                  # entities, relations, GapMatrix
│   ├── evidgap-ports                  # port traits + in-memory fixture impls (feature: `fixtures`)
│   ├── evidgap-orchestrator           # 5-phase pipeline engine
│   ├── evidgap-adapter-pubmed         # v0.1.0
│   ├── evidgap-adapter-chembl         # v0.1.0
│   └── evidgap-adapter-clinicaltrials # v0.1.0
├── apps/
│   └── evidgap-cli                    # binary, last
├── ARCHITECTURE.md
├── TODO.md
├── LICENSE-APACHE
└── Cargo.toml                         # workspace manifest
```

## Sibling workspaces

`evidgap-rs` is a peer to four reference-data workspaces. None depends on another at the workspace level. All emit Apache Arrow `RecordBatch` as the local contract. `evidgap-rs` consumes from them via Arrow adapters, planned for v0.2.0+.

| Workspace                                                                    | Role                                                                | License           |
|------------------------------------------------------------------------------|---------------------------------------------------------------------|-------------------|
| [`clinical-rs`](https://github.com/SHA888/clinical-rs)                       | Clinical records, code ontologies, task windowing                   | MIT OR Apache-2.0 |
| [`multiomics-rs`](https://github.com/SHA888/multiomics-rs)                   | Molecular reference databases (UniProt, Reactome, Open Targets, …)  | MIT OR Apache-2.0 |
| [`multiomics-rs-licensed`](https://github.com/SHA888/multiomics-rs-licensed) | Molecular references requiring license agreements                   | MIT OR Apache-2.0 |
| [`biomedref-rs`](https://github.com/SHA888/biomedref-rs)                     | Reference data outside strict molecular omics                       | MIT OR Apache-2.0 |
| `evidgap-rs` (this workspace)                                                | Federation + orchestration over MCP and Arrow                       | Apache-2.0        |

## Connectors

| Layer              | Connector              | Adapter crate                            | Status    |
|--------------------|------------------------|------------------------------------------|-----------|
| Mechanism          | ChEMBL                 | `evidgap-adapter-chembl`                 | 🚧 v0.1.0 |
| Trial              | ClinicalTrials.gov     | `evidgap-adapter-clinicaltrials`         | 🚧 v0.1.0 |
| Evidence           | PubMed                 | `evidgap-adapter-pubmed`                 | 🚧 v0.1.0 |
| Coding             | ICD-10                 | `evidgap-adapter-icd10`                  | 📋 v0.2.0 |
| Preprint           | bioRxiv                | `evidgap-adapter-biorxiv`                | 📋 v0.2.0 |
| Semantic           | Scholar Gateway        | `evidgap-adapter-scholargateway`         | 📋 v0.2.0 |
| Expression         | Synthesize Bio         | `evidgap-adapter-synthesizebio`          | 📋 v0.2.0 |
| Commercial         | AdisInsight            | `evidgap-adapter-adisinsight`            | 📋 v0.2.0 |
| Local mechanism    | Open Targets (Arrow)   | `evidgap-adapter-opentargets`            | 📋 v0.3.0 |
| Local druggability | DGIdb (Arrow)          | `evidgap-adapter-dgidb`                  | 📋 v0.3.0 |
| Local target       | UniProt (Arrow)        | `evidgap-adapter-uniprot`                | 📋 v0.3.0 |
| Local pathway      | Reactome (Arrow)       | `evidgap-adapter-reactome`               | 📋 v0.3.0 |
| Local coding       | medcodes (Arrow)       | `evidgap-adapter-medcodes`               | 📋 v0.3.0 |

## Canonical identifiers — v0.1.0

| Entity      | Canonical ID source              | Newtype                |
|-------------|----------------------------------|------------------------|
| Disease     | MONDO                            | `evidgap_id::Mondo`    |
| Gene        | HGNC                             | `evidgap_id::Hgnc`     |
| Compound    | ChEMBL                           | `evidgap_id::Chembl`   |
| Target      | UniProt                          | `evidgap_id::UniProt`  |
| Trial       | ClinicalTrials.gov NCT           | `evidgap_id::Nct`      |
| Publication | DOI                              | `evidgap_id::Doi`      |

Roadmap canonical IDs ship with their first consuming adapter, not before:

- **v0.2.0** — ICD-10 (`evidgap_id::Icd10<V: Icd10Version>`, type-level versioning) shipping alongside `evidgap-adapter-icd10`
- **v0.3.0+** (evaluated when first downstream consumer appears) — RxNorm, ATC, MeSH, HPO, Reactome, KEGG, OMIM, SNOMED-CT (license gated), LOINC, AdisInsight pipeline IDs

## Scope boundary

`evidgap-rs` handles federation and orchestration of biomedical evidence metadata.

It does **not** handle:

- Parsing biomedical data files → use `clinical-rs`, `multiomics-rs`, or `biomedref-rs`
- Clinical record processing → `clinical-rs`
- Clinical-coding semantic depth (ICD-10 hierarchy, ATC, LOINC, SNOMED-CT lookup, cross-system mapping) → `clinical-rs/medcodes`. evidgap's `Code` entity is a CURIE-shaped pass-through that carries provenance only; rich code-system semantics are owned by `medcodes`.
- Raw sequencing formats (BAM, VCF, FASTQ) → `oxbow`, `noodles`
- Model training or inference
- Knowledge-graph storage. Consumers may persist `KnowledgeState` to any backend (Neo4j, RDF, Datalog, columnar Arrow); the workspace ships no graph database.

## Design principles

1. **Ports model the domain, not the transport.** Every port trait is testable in-memory. CI fails if any port lacks at least two adapter implementations: one real, one fixture.
2. **Identity resolution is tri-state.** `Xref<T>` distinguishes `Resolved`, `NotFound`, and `Unattempted`. Conflating the latter two is the most common gap-accounting failure mode in federation.
3. **Provenance is mandatory.** Facts cross port boundaries wrapped in `Sourced<T>` carrying connector, query, and timestamps. Bare facts do not exist on the public surface.
4. **Gap matrix is the deliverable.** Prose summarization is downstream and optional. Every cell of the matrix is falsifiable: it points to a query and a result.
5. **Apache Arrow on the local edge, MCP on the remote edge.** No third format on the contract surface.
6. **Library-first, binary-last.** `evidgap-cli` is a thin orchestration over the public crate API; nothing ships exclusively in the binary.

## Relationship to existing tools

`evidgap-rs` is positioned as a **pipeline-first generator of gap-aware typed evidence**. Adjacent ecosystems either federate positive-evidence (we bridge to them via export), do partial-overlap work in one of our five phases (we differ in scope and discipline), or visualize curated evidence downstream (we feed them).

| Tool                                                                              | Approach                                                                                       | Relationship to evidgap-rs                                                                                                                                                                                              |
|-----------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| [NCATS Translator](https://ncats.nih.gov/research/research-activities/translator) (2025 GA) + [TRAPI](https://github.com/NCATSTranslator/ReasonerAPI) + [BioLink Model](https://biolink.github.io/biolink-model/) | Federated positive-evidence KGs, multi-institution consortium, OpenAPI-based query/response standard | **Adjacent ecosystem.** evidgap exports a positive-evidence TRAPI subset (post-1.0) for Translator KP-mode integration. The gap matrix has no TRAPI analog by design — different epistemic commitment, not a deficiency.                |
| [Open Targets Platform](https://platform.opentargets.org/)                        | Mechanism-to-trial evidence aggregation                                                         | **Consumed.** evidgap consumes Open Targets via `multiomics-rs/open-targets-rs` as a local Arrow adapter (v0.3.0). evidgap adds the federation, gap accounting, and orchestration layer.                                |
| [OptimusKG](https://github.com/mims-harvard/OptimusKG) (Vittor et al., *Nature Scientific Data* 2026) | Pre-built multimodal biomedical KG; 65 sources harmonized via BioCypher + BioLink Model; distributed as Apache Parquet (medallion gold layer) | **Consumed (post-1.0).** evidgap consumes OptimusKG's gold-layer Parquet via a future `evidgap-adapter-optimuskg`. Their BioLink-aligned schema (10 entity types, 26 relations) doubles as a worked example of the mapping our TRAPI export work needs. Different epistemic commitment (positive-evidence batch KG vs. real-time gap-aware federation), structurally complementary. |
| [PubTator Central](https://www.ncbi.nlm.nih.gov/research/pubtator/), [BioREx](https://github.com/ncbi/BioREx) | Entity-tagged literature retrieval, biomedical relation extraction                              | **Adjacent NLP.** Different layer. evidgap may consume relation-extraction outputs via a future `EvidencePort` adapter (post-1.0).                                                                                       |
| [BioThings Explorer](https://github.com/biothings/biothings_explorer)             | TRAPI-compliant federated query API (Python)                                                    | **Sibling federation tool.** Different language, different epistemic stance (positive-evidence-only). Interoperable via TRAPI export.                                                                                    |
| [Biobtree](https://github.com/tamerh/biobtree)                                    | Bioinformatics identifier mapper, single executable, B+tree-backed                              | **Phase-1-only overlap.** Identifier mapping is one of evidgap's five phases. Biobtree appears dormant since 2020; evidgap covers a different scope and continues active development.                                    |
| [dbxref (PyPI)](https://pypi.org/project/dbxref/)                                 | Library for resolving database cross references                                                 | **Phase-1-only overlap.** Smaller scope, low activity. Same niche as Biobtree.                                                                                                                                           |
| [EPPI-Mapper](https://eppi.ioe.ac.uk/cms/Default.aspx?tabid=3790) / [EviAtlas](https://environmentalevidencejournal.biomedcentral.com/articles/10.1186/s13750-019-0167-1) / [MetaReviewer](https://www.air.org/mosaic/tools) | Visualization tools for evidence gap maps and systematic-review databases                       | **Downstream.** They visualize *curated* evidence; evidgap *generates* the typed evidence stream they consume. Export adapters planned post-1.0.                                                                         |

## Requirements

- Rust 1.94+ (2024 edition)
- Network access to MCP connector endpoints for live adapters
- Optional: Apache Arrow reference data from sibling workspaces for offline adapters (v0.2.0+)

## License

Apache-2.0. See [`LICENSE-APACHE`](LICENSE-APACHE).

The Apache-2.0 patent grant is load-bearing for federation work that touches drug discovery and clinical evidence; permissive licenses without explicit patent terms (MIT, BSD) are intentionally not used here. Sibling workspaces are dual-licensed MIT OR Apache-2.0; `evidgap-rs` consumes them under the Apache-2.0 leg.

## Citation

```bibtex
@software{evidgap_rs,
  author  = {Kresna Sucandra},
  title   = {evidgap-rs: Federation and orchestration layer for biomedical evidence},
  url     = {https://github.com/SHA888/evidgap-rs},
  license = {Apache-2.0},
}
```
