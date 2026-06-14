# evidgap-rs Plans.md

Created: 2026-06-15

---

## Phase 0.0.4: Code Quality Configuration

Configuration files that gate all downstream CI/CD and enforce workspace-wide standards.

| Task | Content | DoD | Depends | Status |
|------|---------|-----|---------|--------|
| 0.4.1 | Create `rustfmt.toml` with edition 2024, max_width 100, import granularity | `cargo fmt --all -- --check` passes; formatting consistent across workspace | - | cc:done [65bd148] |
| 0.4.2 | Create `deny.toml` with advisories, licenses, bans, sources policies | `cargo deny check` passes with no errors or warnings | - | cc:done [f49150a] |
| 0.4.3 | Update all 8 crate `Cargo.toml` files to inherit workspace lints via `lints = workspace = true` | `cargo clippy --workspace --all-targets --all-features -- -D warnings` passes | 0.4.1, 0.4.2 | cc:done [ab3d971] |
| 0.4.4 | Verify `cargo check --workspace` and `cargo fmt --check` pass on fresh state | All checks pass; no formatting drift | 0.4.3 | cc:done [HEAD] |

---

## Phase 0.0.5: xtask Governance Scripts

CI gate enforcement scripts that verify design principles (dual-adapter rule, provenance correctness, cardinality documentation).

| Task | Content | DoD | Depends | Status |
|------|---------|-----|---------|--------|
| 0.5.1 | Implement `xtask check-ports` — enumerate port traits via syn, verify >= 2 impls (1 fixture, 1 real) per trait | Command exits 0; rejects workspace if any port has < 2 impls; used in CI | 0.4.4 | cc:done [a1587bf] |
| 0.5.2 | Implement `xtask check-cardinality` — parse rustdoc, verify each port method has `# Cardinality` section with Vec/Stream rationale | Command exits 0 on port traits with cardinality docs; fails if missing | 0.5.1 | cc:todo |
| 0.5.3 | Implement `xtask check-provenance` — parse port method return types, fail if bare entity types cross boundary (require Sourced<T>) | Command exits 0; rejects port methods returning bare T without Sourced wrapper | 0.5.2 | cc:todo |
| 0.5.4 | Implement `xtask check-msrv` — install MSRV toolchain, run `cargo check --workspace` | Command succeeds on MSRV (1.94); gated by CI weekly | 0.5.3 | cc:todo |
| 0.5.5 | Implement `xtask audit` — wrap `cargo audit` and `cargo deny check` with clear error messages | Command aggregates both audits into single exit code | 0.5.4 | cc:todo |
| 0.5.6 | Implement `xtask coverage` — wrap `cargo llvm-cov` for unified workspace coverage (optional for Phase 0; defer if llvm-tools unavailable) | Command produces coverage report or exits with explicit "llvm-tools not available" | 0.5.5 | cc:todo |
| 0.5.7 | Implement `xtask release-dry-run` — invoke `cargo release --dry-run` with workspace-aware tag inference | Command outputs release plan without publishing; used pre-release | 0.5.6 | cc:todo |

---

## Phase 0.0.6: CI/CD Pipeline (GitHub Actions)

Automated workflow gates that run on every push, tag, and schedule. Enforces all prior checks.

| Task | Content | DoD | Depends | Status |
|------|---------|-----|---------|--------|
| 0.6.1 | Create `.github/workflows/ci.yml` — primary CI on `push` and `pull_request`; matrix: ubuntu/macos/windows; fail-fast: false | Workflow runs and passes on commit to branch; includes fmt, clippy, nextest, doc build, deny, xtask checks | 0.5.7 | cc:todo |
| 0.6.2 | Create `.github/workflows/release.yml` — triggered on `v*` tag push; runs semver-checks and publishes | Workflow detects changed crate(s), runs semver-checks, publishes to crates.io with `CARGO_REGISTRY_TOKEN` secret | 0.5.7 | cc:todo |
| 0.6.3 | Create `.github/workflows/audit.yml` — scheduled nightly RustSec scan; opens issue on finding | Workflow runs on schedule; creates GitHub issue if advisories found | 0.6.1 | cc:todo |
| 0.6.4 | Create `.github/workflows/msrv.yml` — scheduled weekly MSRV check via `xtask check-msrv` | Workflow runs on schedule; fails if MSRV build fails | 0.6.1 | cc:todo |
| 0.6.5 | Create `.github/workflows/coverage.yml` — runs on `main` push; uploads coverage report | Workflow produces coverage, uploads to Codecov or artifact storage | 0.6.1 | cc:todo |
| 0.6.6 | Configure branch protection rules on `main` (manual GitHub UI step) — require PR, CI green, 1 approval, no force push | Rules visible in repo settings; enforce on all pushes | 0.6.5 | cc:todo |

---

## Phase 0.0.7: Repository Files

Governance and contributor documentation. Defines security policy, code of conduct, and contribution workflow.

| Task | Content | DoD | Depends | Status |
|------|---------|-----|---------|--------|
| 0.7.1 | Create `LICENSE-APACHE` — full Apache 2.0 text with copyright year 2026, copyright holder "Kresna Sucandra" | File exists; `cargo deny check licenses` passes; REUSE compliance (optional) | - | cc:todo |
| 0.7.2 | Expand `CONTRIBUTING.md` — add dev setup (clone, rustup, tool installs), Conventional Commits, PR process, port trait addition guide, adapter addition guide, canonical ID guide | File documents all required steps; new contributor can follow without external help | 0.7.1 | cc:todo |
| 0.7.3 | Create `SECURITY.md` — responsible disclosure policy, contact method (GitHub security advisory), scope (identity-resolution bugs as security-severity) | File explains how to report security issues; clear scope definition | 0.7.1 | cc:todo |
| 0.7.4 | Create `CODE_OF_CONDUCT.md` — Contributor Covenant v2.1 | File present and enforced as linked from README | 0.7.1 | cc:todo |
| 0.7.5 | Create `.github/ISSUE_TEMPLATE/bug_report.md` — repro, expected vs. actual, crate + version | Template guides issue reporters; makes triage easier | 0.7.1 | cc:todo |
| 0.7.6 | Create `.github/ISSUE_TEMPLATE/feature_request.md` — use case, proposed API, which crate | Template structures feature requests; clarifies scope | 0.7.1 | cc:todo |
| 0.7.7 | Create `.github/ISSUE_TEMPLATE/port_proposal.md` — new port trait proposal with cardinality, provenance shape, fixture sketch | Template guides port trait additions per design principles | 0.7.1 | cc:todo |
| 0.7.8 | Create `.github/PULL_REQUEST_TEMPLATE.md` — checklist: tests, docs, CHANGELOG, fmt, clippy, xtask passes, fixtures for new ports | Template ensures PRs include required artifacts before review | 0.7.1 | cc:todo |

---

## Phase 0.0.8: Release Infrastructure

Per-crate versioning, changelog generation, and publish automation.

| Task | Content | DoD | Depends | Status |
|------|---------|-----|---------|--------|
| 0.8.1 | Create `cliff.toml` — git-cliff config for Conventional Commits, group by (feat, fix, perf, refactor, docs, test, ci, chore), per-crate changelog generation | `git cliff --unreleased` outputs structured changelog; per-crate filtering works | 0.7.8 | cc:todo |
| 0.8.2 | Create `release.toml` — cargo-release config with per-crate tags format (`evidgap-id-v0.1.0`, etc.) and pre-release hook (`xtask release-dry-run`) | Config allows `cargo release patch --workspace --dry-run` to complete | 0.8.1 | cc:todo |
| 0.8.3 | Dry-run release simulation — run `cargo release patch --workspace --dry-run` on a test branch; verify all crates increment, tags are correct, publish would work | Simulation completes without error; shows expected tag list and changelog entries | 0.8.2 | cc:todo |

---

## Phase 0.0.9: Verification Checkpoint

Final validation that Phase 0 is complete and reproducible from scratch.

| Task | Content | DoD | Depends | Status |
|------|---------|-----|---------|--------|
| 0.9.1 | Fresh clone verification — clone repo on clean system, run all Phase 0 checks: `cargo fmt --check`, `cargo clippy`, `cargo nextest`, `cargo doc`, `cargo deny check`, `xtask check-ports`, `xtask check-cardinality`, `xtask check-provenance` | All checks pass on fresh clone; no manual intervention needed | Phase 0.0.4–0.0.8 complete | cc:todo |
| 0.9.2 | Document Phase 0 completion — add entry to TODO.md marking 0.0.4–0.0.9 complete; commit as "chore(phase-0): mark bootstrap complete" | TODO.md reflects completion; commit merged to main | 0.9.1 | cc:todo |

---

## Spec delta

No product-contract updates required. Phase 0 is infrastructure and governance implementation against the already-documented design in ARCHITECTURE.md and CONTRIBUTING.md.

**Spec skip reason:** Code quality configuration, CI/CD gates, repository governance, and release infrastructure are mechanical implementations of documented principles. No new product behavior, API, or design decisions introduced — all gates enforce existing design constraints (dual-adapter rule, provenance correctness, cardinality discipline, Conventional Commits). Changes are tool-level, not domain-level.

---

## Execution notes

**Team validation mode:** `native` (no subagents required; infrastructure tasks are deterministic against documented spec).

**Dependency structure:**
- **0.4.x** (Code quality) is foundation for all downstream tasks
- **0.5.x** (xtask) is foundation for 0.6.x (CI gates call xtask commands)
- **0.6.x** (CI/CD) depends on 0.4–0.5 and can run in parallel with 0.7–0.8
- **0.7–0.8** (Governance & release) depend on 0.6 (release.yml is in workflows)
- **0.9.x** (Verification) is final checkpoint; depends on all prior phases

**Parallelizable tasks:**
- Within Phase 0.4: all tasks can run in parallel after initial file reads
- Within Phase 0.5: xtask tasks are independent after foundation (0.5.1)
- Within Phase 0.6: workflows can be created in parallel; only 0.6.6 (branch protection) requires 0.6.x completion
- Within Phase 0.7–0.8: can work in parallel, but 0.8.1 (cliff.toml) benefits from 0.7.2 (CONTRIBUTING.md) being done first for reference

**Suggested session strategy:**
1. **Session 1:** Phase 0.4 (Code quality) — standalone, foundational, ~30 min
2. **Session 2:** Phase 0.5 (xtask) — medium-complexity code, ~1–1.5 hours
3. **Session 3:** Phase 0.6 (CI/CD) — high volume of YAML, ~45 min–1 hour
4. **Session 4:** Phase 0.7–0.8 (Governance & release) — documentation + config, ~1 hour
5. **Session 5:** Phase 0.9 (Verification) — final validation, ~15 min

Alternatively, combine sessions 1–2 or 2–3 if consecutive execution is preferred.

---

## Next action

新しいセッションの起動コマンド: `claude`

起動後の最初の入力: `/harness-work 0.4.1`

向いている場面: Phase 0.4.1 is the lightest first task; all downstream tasks depend on infrastructure being complete. Starting with rustfmt.toml unblocks parallel work on deny.toml and crate updates.
