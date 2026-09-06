# AGENTS.md

# Conceptarium agent instructions

Conceptarium is a canonical research-memory repository, not a generic notes folder.

Before adding, promoting, renaming, or substantially revising any concept, read:

1. [docs/INTEGRATING_CONCEPTS.md](docs/INTEGRATING_CONCEPTS.md) — **mandatory end-to-end integration contract**
2. [docs/REGISTRY.md](docs/REGISTRY.md) — predicate presence, lazy capture, and promotion queue
3. [docs/SCHEMA.md](docs/SCHEMA.md) — canonical entry schema
4. [docs/EDITORIAL.md](docs/EDITORIAL.md) — preservation and editorial doctrine
5. [docs/RELATIONS.md](docs/RELATIONS.md) — typed relation ontology
6. [docs/PROJECTIONS.md](docs/PROJECTIONS.md) — source/projection boundary
7. [docs/QUERY.md](docs/QUERY.md) — Rust query engine and backend roadmap

## Non-negotiable rules

- Every known concept must have predicate presence in `registry/concepts.yml`.
- Registry presence and full semantic materialization are independent.
- Full entries remain canonical for semantic content; the registry is canonical for identity/presence.
- Many derived projections; no duplicate semantic sources.
- Preserve meaningful coined wording and old vocabulary.
- Reconstruct **problem pressure**: the frustration, anomaly, distinction, or observation that made the concept necessary.
- Never fabricate provenance, exact dates, quotations, or authorship.
- Entry status describes vocabulary maturity, not scientific truth.
- Prefer deep entries to mass-generated stubs.
- When development would interrupt or exhaust the session, create a registry-only concept instead of a shallow entry or losing the idea.
- Never invent ontology merely to complete capture; `ontology_state: unplaced` is valid.
- Search for an existing concept before creating a new one.
- Preserve promising named sub-concepts in the registry even when they remain unmaterialized; parent entries/clusters may still hold their contextual prose.
- Relations are typed theoretical claims and must pass the sentence test.
- Do not manually edit generated `build/` artifacts.
- Every promoted entry must appear in `indexes/terms.md` and resolve to an `entry` record in the Concept Registry.
- Every relation target must resolve to the registry; registry-only targets are valid, unregistered targets are broken references.
- Keep the promoted corpus count accurate.
- Run the Rust validator and projections before declaring integration complete.
- Python is not repository infrastructure; do not reintroduce Python tooling for canonical parsing, validation, projection generation, or query services.

## Required verification

From repository root:

```bash
cargo check --all-targets
cargo test --all-targets
cargo run --quiet -- validate
cargo run --quiet -- project
```

If GitHub Actions is available, verify the final relevant workflow run.

Do not claim “integrated” merely because a file was committed.

The full definition of integration is in [docs/INTEGRATING_CONCEPTS.md](docs/INTEGRATING_CONCEPTS.md).
