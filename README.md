# Conceptarium

**Conceptarium** is the canonical knowledge base for concepts, vocabulary, notes, distinctions, frameworks, mechanisms, metaphors, and “amber phrases” developed during ongoing research and conversation.

The central rule is:

> **Preserve the concept once; derive many views from it.**

A concept has one canonical source entry. Dictionary definitions, encyclopedia articles, relationship graphs, provenance timelines, topic indexes, and future blog pages are **projections** of that same source rather than separately maintained copies.

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
├── entries/          # canonical concept records
├── clusters/         # curated maps of families of concepts
├── indexes/          # human-readable indexes
├── docs/             # schema, editorial rules, projection design
└── archive/          # preserved snapshots/imports that must not be lost
```

Each concept normally lives at:

```text
entries/<slug>.md
```

The file contains two layers:

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

## Entry maturity

Entries can exist before they are finished.

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

See [docs/SCHEMA.md](docs/SCHEMA.md) for the canonical entry format and [docs/EDITORIAL.md](docs/EDITORIAL.md) for editorial rules.
