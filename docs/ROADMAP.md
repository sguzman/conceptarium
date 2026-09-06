# Roadmap

Conceptarium should grow in layers. The immediate priority is **preservation and conceptual depth**, not website polish.

## Phase 0 — Loss prevention

**Status: established**

- repository initialized;
- canonical entry schema established;
- editorial doctrine established;
- projection architecture documented;
- recovered master lexicon archived intact;
- first high-value concepts promoted into deep entries.

The archive is a safety net, not the final information architecture.

## Phase 1 — Corpus migration

**Status: active — 151 concepts/phrases/frameworks promoted as individual entries.**

Promotion means more than copying a dictionary sentence. A promoted entry should acquire, where appropriate:

- structured metadata;
- dictionary gloss;
- long-form definition;
- problem pressure;
- mechanism or model;
- examples and non-examples;
- boundaries/failure modes;
- operationalization;
- typed relations;
- provenance;
- open questions;
- semantic revision history.

Do **not** generate a hundred shallow files merely to claim completion.

Promotion order:

1. concepts repeatedly reused in new reasoning;
2. concepts with strong “problem pressure” worth reconstructing;
3. concepts that resolve important dangling relations;
4. high-value amber phrases;
5. concepts whose provenance is at risk of being forgotten;
6. supporting vocabulary needed by major frameworks.

Major clusters currently receiving deep migration:

- epistemology / narration / AI trust;
- Fallen / Village / elite dynamics;
- institutions / dependency / anti-domination;
- generative simulation / procedural visual systems;
- culture / consumption / signification;
- franchise identity / branding / semantic drift;
- cultural refraction / authorship / estrangement / spatial horror;
- aesthetic governance / erotic legitimacy / audience allocation;
- attention / absorption / completion;
- abundance / pleasure / prestige.

## Phase 2 — Relation cleanup

**Status: active — manual curation plus automated validation are in place.**

As entries accumulate:

- normalize relation verbs;
- create useful inverse edges;
- detect broken targets;
- distinguish aliases from refinements;
- identify conceptual cycles;
- build curated cluster maps.

The validator currently flags:

- duplicate IDs;
- duplicate terms;
- invalid statuses/types;
- missing relation targets;
- malformed frontmatter;
- filenames that disagree with IDs;
- aliases colliding with canonical terms.

## Phase 3 — Projection generator

**Status: initial generator implemented.**

The repository includes a deterministic renderer in `tools/project.py` that reads `entries/*.md` and emits disposable projections into `build/`.

Current generated artifacts:

### Dictionary

Alphabetized cards from frontmatter.

### Encyclopedia

Full articles with backlinks.

### Graph data

JSON containing nodes and typed edges.

### Genealogy

Origin dates, conceptual ancestors, refinements, and semantic revisions.

### Research frontier

Open questions + contested/provisional concepts + provenance TODOs.

### Problem-pressure index

Reverse index from the *puzzle that birthed a concept* to the concepts produced by that puzzle.

This is especially important because it lets us recover a forgotten term from the original frustration that created it.

## Phase 4 — Blog integration

Only after the content schema proves stable should the blog become a renderer.

Preferred architecture:

```text
Conceptarium repo
    │
    ├── canonical Markdown/YAML
    │
    └── generated machine views
            │
            └── blog/site renderer
```

The blog can add:

- search;
- backlinks;
- graph exploration;
- timelines;
- domain pages;
- hover definitions;
- amber-phrase browsing;
- revision diffs;
- essay embeds.

But the blog should not own canonical definitions.

## Phase 5 — Research instrumentation

Once the corpus is large enough, Conceptarium can become an active research tool.

Possible features:

- automatic detection of unlinked concept mentions;
- “concept collision” pages showing entries that make contradictory claims;
- query by problem pressure;
- query by relation path;
- chronological visualization of research programs;
- semantic diff between revisions;
- provenance confidence indicators;
- external-literature comparison;
- export for AI retrieval;
- generated “what are we currently confused about?” pages.

## Website stack

Do not commit yet.

Markdown/YAML should remain portable across Astro, Hugo, Eleventy, Next.js, a Rust static generator, or another future renderer.

Choose the site stack only when projection requirements are concrete.

## Long-term shape

Conceptarium can eventually behave simultaneously as:

1. **dictionary** — quick term lookup;
2. **encyclopedia** — full theoretical treatment;
3. **knowledge graph** — typed conceptual relationships;
4. **genealogy** — where ideas came from;
5. **research notebook** — unresolved questions and seeds;
6. **intellectual atlas** — clusters and domains;
7. **semantic history** — how definitions changed;
8. **AI-readable corpus** — structured retrieval over accumulated conceptual vocabulary.

The repository remains the durable layer beneath all eight.
