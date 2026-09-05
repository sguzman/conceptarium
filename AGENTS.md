# AGENTS.md

# Conceptarium agent instructions

Conceptarium is a canonical research-memory repository, not a generic notes folder.

Before adding, promoting, renaming, or substantially revising any concept, read:

1. [docs/INTEGRATING_CONCEPTS.md](docs/INTEGRATING_CONCEPTS.md) — **mandatory end-to-end integration contract**
2. [docs/SCHEMA.md](docs/SCHEMA.md) — canonical entry schema
3. [docs/EDITORIAL.md](docs/EDITORIAL.md) — preservation and editorial doctrine
4. [docs/RELATIONS.md](docs/RELATIONS.md) — typed relation ontology
5. [docs/PROJECTIONS.md](docs/PROJECTIONS.md) — source/projection boundary

## Non-negotiable rules

- One canonical concept entry; many derived projections.
- Preserve meaningful coined wording and old vocabulary.
- Reconstruct **problem pressure**: the frustration, anomaly, distinction, or observation that made the concept necessary.
- Never fabricate provenance, exact dates, quotations, or authorship.
- Entry status describes vocabulary maturity, not scientific truth.
- Prefer deep entries to mass-generated stubs.
- Search for an existing concept before creating a new one.
- Preserve promising sub-concepts inside parent entries/clusters until they demonstrate independent analytical use.
- Relations are typed theoretical claims and must pass the sentence test.
- Do not manually edit generated `build/` artifacts.
- Every promoted entry must appear in `indexes/terms.md`.
- Keep the promoted corpus count accurate.
- Run validation and projections before declaring integration complete.

## Required verification

From repository root:

```bash
python -m pip install -r requirements-dev.txt
python tools/validate.py
python tools/project.py
```

If GitHub Actions is available, verify the final relevant workflow run.

Do not claim “integrated” merely because a file was committed.

The full definition of integration is in [docs/INTEGRATING_CONCEPTS.md](docs/INTEGRATING_CONCEPTS.md).
