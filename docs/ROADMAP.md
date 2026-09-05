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

Promote recovered vocabulary from the archive into individual entries.

Do **not** generate a hundred one-paragraph files merely to claim completion.

Promotion order should follow:

1. concepts repeatedly reused in new reasoning;
2. concepts with strong “problem pressure” worth reconstructing;
3. concepts that resolve dangling typed relations;
4. high-value amber phrases;
5. concepts whose provenance is at risk of being forgotten;
6. supporting vocabulary needed by major frameworks.

During migration, recover provenance where possible from conversations and research documents.

## Phase 2 — Relation cleanup

As entries accumulate:

- normalize relation verbs;
- create missing inverse edges where useful;
- detect broken targets;
- distinguish aliases from refinements;
- identify conceptual cycles;
- build curated cluster maps.

A future validation tool should flag:

- duplicate IDs;
- duplicate terms;
- invalid statuses/types;
- missing relation targets;
- malformed frontmatter;
- filenames that disagree with IDs;
- aliases colliding with canonical terms.

## Phase 3 — Projection generator

Build a small deterministic renderer that reads `entries/*.md`.

Initial generated artifacts:

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

This is a particularly important projection because it helps recover a forgotten term from the original mental frustration.

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
- a generated “what are we currently confused about?” page.

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
8. **AI-readable corpus** — structured retrieval over the user's accumulated conceptual vocabulary.

The repository remains the durable layer beneath all eight.
