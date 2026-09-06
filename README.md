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
├── registry/         # canonical predicate-presence ledger for every known concept
├── entries/          # full semantic materializations of promoted concepts
├── clusters/         # curated maps of families of concepts
├── indexes/          # human-readable indexes
├── docs/             # integration contract, schema, editorial rules, projections
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

See [docs/REGISTRY.md](docs/REGISTRY.md) for predicate presence and lazy capture, [docs/SCHEMA.md](docs/SCHEMA.md) for the canonical entry format, [docs/EDITORIAL.md](docs/EDITORIAL.md) for editorial rules, and [docs/RELATIONS.md](docs/RELATIONS.md) for graph semantics.


## Validation

Conceptarium includes a lightweight corpus validator:

```bash
python -m pip install -r requirements-dev.txt
python tools/validate.py
```

It checks the Concept Registry, canonical frontmatter, IDs, entry types/statuses, provenance fields, alias collisions, relation shape, registry/entry consistency, and several maturity expectations. Every materialized entry and every relation target must have registry presence; relations to registry-only concepts are valid.

During corpus migration, experimental relation verbs and dangling relation targets are **warnings** rather than hard failures. Use:

```bash
python tools/validate.py --strict
```

when you want warnings to fail as well.

GitHub Actions runs the non-strict validator automatically on pushes to `main` and on pull requests.

Relation design is documented in [docs/RELATIONS.md](docs/RELATIONS.md).


## Generate projections

Derived views are generated from the canonical entries and written to the ignored `build/` directory:

```bash
python tools/project.py
```

The current generator emits:

- `build/dictionary.md` — compact dictionary projection;
- `build/graph.json` — typed node/edge graph with dangling-target markers;
- `build/frontier.md` — open questions and explicitly provisional/contested entries;
- `build/problem-pressure.md` — reverse lookup from the problem that birthed a concept;
- `build/promotion-queue.md` — all registry-only concepts awaiting materialization;
- `build/registry.json` — machine-readable predicate-presence projection;
- `build/catalog.json` — compact machine-readable catalog including materialized and registry-only concepts.

Generated files are disposable. **Never edit them as source.**
