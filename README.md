# Conceptarium

**Conceptarium** is the canonical knowledge base for concepts, vocabulary, notes, distinctions, frameworks, mechanisms, metaphors, and “amber phrases” developed during ongoing research and conversation.

The central rules are:

> **Preserve conceptual identity as soon as it appears. Materialize meaning progressively. Derive many views from canonical sources.**

Conceptarium now separates **predicate presence** from **ontological materialization**. Every known concept has a durable identity in the [Concept Registry](docs/REGISTRY.md), even when it does not yet have a full article, ontology placement, definition, or relation set.

When a concept is materially developed, its Markdown entry becomes authoritative for semantic content. Dictionary definitions, graphs, research-frontier views, promotion queues, and future blog pages are **projections** rather than separately maintained semantic copies.

## What belongs here

Anything that does real conceptual work belongs in Conceptarium, including:

- coined terms and specialized vocabulary;
- existing terms given a specific local meaning;
- distinctions that prevent two ideas from collapsing into one;
- named mechanisms and causal patterns;
- frameworks and theory components;
- recurring failure modes;
- metaphors whose wording preserves an insight;
- questions that repeatedly generate useful theory;
- unfinished concepts worth preserving before they disappear;
- provenance explaining the frustration, puzzle, observation, or conversation that gave birth to an idea.

This is not restricted to AI research. Concepts may concern epistemology, politics, narration, institutions, social theory, generative systems, aesthetics, psychology, architecture, games, or any other domain.

## Repository model

```text
conceptarium/
├── Cargo.toml        # Rust tooling/query crate
├── src/              # parser, validator, query engine, registry, projections
├── registry/         # canonical predicate-presence ledger for every known concept
├── entries/          # full semantic materializations of promoted concepts
├── clusters/         # curated maps of families of concepts
├── indexes/          # human-readable indexes
├── docs/             # integration contract, schema, query/backend architecture
└── archive/          # preserved snapshots/imports that must not be lost
```

Every known concept first has registry presence in:

```text
registry/concepts.yml
```

A fully materialized concept normally also lives at:

```text
entries/<slug>.md
```

The entry contains two layers:

1. **Structured metadata** in YAML frontmatter for machines, indexes, graphs, and dictionary projection.
2. **Long-form Markdown** for the encyclopedic treatment.

## Projection model

The canonical entry can generate several views.

### Dictionary projection

Uses the term, gloss, aliases, status, domains, and a small set of relations.

Designed to answer:

> “What does this term mean?”

### Encyclopedia projection

Uses the full entry body.

Designed to answer:

> “What is the theory, why was the concept needed, how does it work, where does it break, and how has it evolved?”

### Graph projection

Uses typed relations such as:

```text
diegetization
  solves -> epistemic-debt
  requires -> epistemic-chain-of-custody
  exemplifies -> great-subjectification
```

### Genealogy / provenance projection

Uses origin and revision records to show when a concept appeared, what problem generated it, what earlier ideas fed it, and what later concepts descended from it.

### Atlas / cluster projection

Groups concepts by domains, research programs, and curated conceptual families.

The blog, if and when it exists, should ideally **render these projections from this repository** rather than become a second database.

## Preservation doctrine

Conceptual history must not be silently overwritten.

If a term evolves:

- preserve the earlier wording;
- record the later refinement;
- use aliases or successor relations;
- deprecate rather than erase;
- keep uncertainty explicit;
- distinguish “the mechanism is real/useful” from “this exact phrase is canonical.”

A compact phrase may be an intellectual artifact in its own right. For example, an “amber phrase” can preserve an insight even when a longer formal entry exists.

## Presence and entry maturity

Concepts can exist before entries exist at all.

A **registry-only** concept has predicate presence: it can be named, queued, related, searched, or embedded without requiring a definition or ontology placement. Once a full entry exists, entry maturity uses the statuses below.

- **seed** — worth saving; meaning still loose.
- **provisional** — recognizable concept with unresolved boundaries.
- **canonical** — stable enough that we intentionally reuse the term.
- **contested** — useful but internally disputed or under active revision.
- **deprecated** — superseded, but retained for conceptual history.
- **archived** — historical record not intended as current vocabulary.

No useful idea should disappear merely because it is immature.

## Principles

1. **Losslessness over tidiness.** Preserve first; normalize later.
2. **One source, many projections.** Avoid duplicate canonical prose.
3. **Provenance is content.** Record the intellectual pressure that produced the term.
4. **Definitions need boundaries.** Say what a concept is *not*.
5. **Relations should be typed.** “Related” is useful; “solves,” “causes,” “contrasts-with,” and “refines” are better.
6. **Examples are tests.** Good examples clarify; counterexamples expose overreach.
7. **Revision is expected.** A knowledge base should remember how its concepts changed.
8. **Plain text is the durable layer.** The corpus must remain usable without any particular website or framework.
9. **Presence is cheaper than understanding.** Capture a concept before demanding its ontology.
10. **Unknown metadata beats invented metadata.** Ontology can be materialized later.

## Adding or integrating a concept

The complete end-to-end contract is [docs/INTEGRATING_CONCEPTS.md](docs/INTEGRATING_CONCEPTS.md).

When someone says **“integrate this concept”**, the expected work includes discovery, provenance, problem-pressure reconstruction, canonical entry writing, typed relations, cluster/index updates, validation, projection generation, and CI verification—not merely creating a Markdown file.

AI agents should also read the root [AGENTS.md](AGENTS.md) before modifying the corpus.

See [docs/REGISTRY.md](docs/REGISTRY.md) for predicate presence and lazy capture, [docs/SCHEMA.md](docs/SCHEMA.md) for the canonical entry format, [docs/EDITORIAL.md](docs/EDITORIAL.md) for editorial rules, [docs/RELATIONS.md](docs/RELATIONS.md) for graph semantics, and [docs/QUERY.md](docs/QUERY.md) for the Rust local-query/backend architecture.


## Rust tooling and local query

Conceptarium's executable infrastructure is a dedicated Rust crate. Python is not required for parsing, validation, registry mutation, projection generation, or querying.

Use it directly through Cargo:

```bash
cargo run -- get fallen-constitutionalism
cargo run -- index build
cargo run -- search "humiliation"
cargo run -- search "correction" --field problem-pressure
cargo run -- sqlite build
cargo run -- sqlite query "SELECT term, type FROM concepts WHERE type = 'mechanism' ORDER BY term"
cargo run -- surreal build
cargo run -- surreal query "SELECT concept_id, term FROM concept WHERE 'epistemology' IN domains ORDER BY term"
cargo run -- list --domain epistemology
cargo run -- relations fallen-constitutionalism
cargo run -- path fallen-constitutionalism anti-domination
cargo run -- queue
```

Or install the local binary:

```bash
cargo install --path .
conceptarium search "correction"
```

The query engine uses the canonical corpus directly plus `petgraph` for structural traversal. **Tantivy** provides local BM25/full-text search, **SQLite** provides a read-only relational projection for arbitrary SQL, and **SurrealDB/SurrealKV** provides an embedded local document-graph query surface. Oxigraph is next; Qdrant Edge follows as a disposable semantic projection over the same Rust domain model.

## Validation

```bash
cargo run --quiet -- validate
```

It checks the Concept Registry, canonical frontmatter, IDs, entry types/statuses, provenance fields, alias collisions, relation shape, registry/entry consistency, and several maturity expectations. Every materialized entry and every relation target must have registry presence; relations to registry-only concepts are valid.

Use:

```bash
cargo run --quiet -- validate --strict
```

when you want warnings to fail as well.

GitHub Actions compiles and tests the Rust tooling, validates the corpus, and regenerates projections on pushes to `main` and on pull requests.

Relation design is documented in [docs/RELATIONS.md](docs/RELATIONS.md).

## Generate projections

Derived views are generated by Rust from the canonical registry and entries and written to the ignored `build/` directory:

```bash
cargo run --quiet -- project
```

The current generator emits:

- `build/dictionary.md` — compact dictionary projection;
- `build/graph.json` — typed node/edge graph including registry-only targets;
- `build/frontier.md` — open questions and explicitly provisional/contested entries;
- `build/problem-pressure.md` — reverse lookup from the problem that birthed a concept;
- `build/promotion-queue.md` — all registry-only concepts awaiting materialization;
- `build/registry.json` — machine-readable predicate-presence projection;
- `build/catalog.json` — compact machine-readable catalog including materialized and registry-only concepts.

Generated files are disposable. **Never edit them as source.**

## Infrastructure rule

> **Markdown/YAML is canonical; Rust owns executable semantics; databases are rebuildable projections.**

See [docs/QUERY.md](docs/QUERY.md) for backend responsibilities and development order.
