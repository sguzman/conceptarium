# Canonical Entry Schema

Every canonical Conceptarium entry is a Markdown file with YAML frontmatter followed by a long-form article.

The schema is intentionally designed for both human editing and machine projection.

## Minimal example

```markdown
---
id: epistemic-chain-of-custody
term: Epistemic chain of custody
type: concept
status: canonical
gloss: The traceable path by which information moves from an originating event or source to a present claimant or reader.
domains:
  - epistemology
  - narration
aliases: []
origin:
  date: 2026-09
  authorship: joint
  certainty: approximate
relations:
  - type: pays
    target: epistemic-debt
  - type: required-by
    target: diegetization
---

# Epistemic chain of custody

## Definition

...

## Problem pressure

...

## Core model

...

## Examples

...

## Boundaries and failure modes

...

## Relations

...

## Provenance

...

## Open questions

...

## Revision history

...
```

## Frontmatter fields

### Identity

#### `id`

Stable machine identifier. Normally identical to the filename without `.md`.

Do not change an ID merely because the display term changes. Stable IDs allow links and generated projections to survive renaming.

#### `term`

Current preferred display name.

#### `type`

One of:

- `concept` — a reusable analytical idea.
- `distinction` — a contrast whose value lies in separating nearby things.
- `mechanism` — a named causal or generative process.
- `framework` — a larger organized system containing multiple concepts.
- `phrase` — wording preserved because the expression itself carries insight.
- `failure-mode` — recurring breakdown pattern.
- `question` — a durable research question worth indexing.
- `method` — a procedure for inquiry, design, or verification.
- `principle` — a normative or design constraint intended to govern action or system construction.
- `metaphor` — a metaphor intentionally retained as an explanatory instrument.

A term can participate in several roles, but one primary type keeps projections predictable.

#### `status`

One of:

- `seed`
- `provisional`
- `canonical`
- `contested`
- `deprecated`
- `archived`

Status concerns **our use of the entry**, not whether the underlying claim has been scientifically proven.

### Dictionary projection

#### `gloss`

The dictionary definition. Usually one sentence; at most a short paragraph.

The gloss should be:

- usable without reading the full article;
- specific enough to distinguish the concept from neighbors;
- descriptive rather than promotional;
- careful about uncertainty when the concept is theoretical.

This is the primary source for dictionary-style rendering.

#### `aliases`

Other names, old names, spelling variants, or phrases that should resolve to this entry.

Never destroy an old meaningful term just because a preferred label changes.

### Classification

#### `domains`

Broad areas where the concept is useful, for example:

```yaml
domains:
  - epistemology
  - narration
  - institutions
```

Domains are many-to-many. They are not folder paths.

Optional future fields may include `tags`, `research_programs`, or `clusters`, but domains should remain relatively coarse.

### Origin metadata

#### `origin.date`

Best known first appearance. Precision is honest rather than forced:

```yaml
date: 2026-09-03
```

or:

```yaml
date: 2026-09
```

or:

```yaml
date: unknown
```

#### `origin.authorship`

One of:

- `user`
- `assistant`
- `joint`
- `pre-existing`
- `unknown`

“Pre-existing” means the expression already existed in outside literature, even if Conceptarium gives it a specialized use.

#### `origin.certainty`

One of:

- `exact`
- `approximate`
- `reconstructed`
- `unknown`

This prevents reconstructed provenance from masquerading as transcript-level certainty.

Optional `origin.note` can summarize the first known setting.

### Typed relations

`relations` is a list of directed edges:

```yaml
relations:
  - type: solves
    target: epistemic-debt
  - type: exemplifies
    target: great-subjectification
```

Relation vocabulary is open but should prefer existing verbs where possible.

Useful relation types include:

- `alias-of`
- `contrasts-with`
- `refines`
- `refined-by`
- `supersedes`
- `superseded-by`
- `part-of`
- `contains`
- `causes`
- `caused-by`
- `enables`
- `enabled-by`
- `constrains`
- `constrained-by`
- `solves`
- `solved-by`
- `requires`
- `required-by`
- `produces`
- `produced-by`
- `predicts`
- `explains`
- `exemplifies`
- `generalizes`
- `specializes`
- `operationalizes`
- `motivates`
- `descends-from`

Relations should be interpretable as sentences:

> Diegetization **solves** epistemic debt.

If a relation needs a paragraph of explanation, explain it in the body too.

## Canonical article sections

Not every seed entry needs every section. Mature entries should attempt most of them.

### Definition

The precise long-form definition. Expand the gloss without merely repeating it.

### Problem pressure

This section is especially important to Conceptarium.

Record the **specific frustration, anomaly, contradiction, recurring observation, or unfinished thought that made the concept necessary**.

The goal is to preserve the intellectual pressure that gave birth to the term.

Questions to answer:

- What were we trying to explain?
- What existing vocabulary felt inadequate?
- What kept recurring without a name?
- What distinction was being lost?
- What would become difficult to think if this term disappeared?

### Core model / mechanism

Explain how the concept works.

For causal concepts, state the mechanism. For distinctions, state the axes. For methods, state the procedure. For metaphors, state the mapping.

### Claims and implications

Separate the concept itself from stronger claims made using it.

Where useful, distinguish:

- definitional claim;
- empirical hypothesis;
- normative judgment;
- prediction;
- metaphorical extension.

### Examples

Use several kinds where possible:

- clean example;
- borderline example;
- surprising example;
- historical or fictional example;
- application to another domain.

### Non-examples

Things that superficially resemble the concept but should not count.

### Boundaries and failure modes

Record where the concept is likely to overreach, collapse distinctions, become unfalsifiable, or encourage false precision.

### Operationalization / evidence

For empirical concepts, explain what observable evidence might support, weaken, or test the idea.

For design concepts, explain what an implementation would need to expose.

### Relations

Explain important conceptual neighbors in prose even when the edges are already in frontmatter.

### Provenance

Preserve the intellectual genealogy.

Recommended subsections:

- **First known appearance**
- **Immediate context**
- **Problem being solved**
- **Conceptual ancestors**
- **Later refinements**
- **External antecedents** if a similar term exists elsewhere

Do not fabricate exact dates, quotes, or priority. Mark reconstructed history as reconstructed.

### Open questions

Keep unresolved pressure visible instead of prematurely sealing the theory.

### Revision history

A semantic history, not a Git changelog.

Example:

```markdown
- **2026-09:** Initial formulation around narration and source-access.
- **2026-09:** Generalized from fiction to institutional and historical knowledge.
```

Git records textual edits. This section records **changes in meaning**.

## Source citations

A future citation convention may be added for conversations, files, articles, books, and external research. Until then, provenance should identify sources as specifically as available without inventing identifiers.

## Projection contract

A renderer should be able to derive at least:

- dictionary card from `term + gloss + aliases + status + domains`;
- encyclopedia page from the Markdown body;
- relationship graph from `relations`;
- chronology/genealogy from `origin` and provenance;
- domain and cluster indexes from metadata.

The source entry remains authoritative if a generated projection disagrees with it.
