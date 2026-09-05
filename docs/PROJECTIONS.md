# Projection Architecture

Conceptarium stores canonical knowledge. Other forms are derived views.

A **projection** selects and transforms part of an entry for a particular cognitive task without becoming an independent source of truth.

## 1. Dictionary

**Question:** What does this word mean?

Input:

- term
- gloss
- aliases
- type
- status
- domains
- a few high-value relations

Possible output:

```text
Epistemic chain of custody
The traceable path by which information moves from an originating
event or source to a present claimant or reader.

Domains: epistemology, narration
Related: epistemic debt, diegetization
```

Dictionary pages should be fast to scan and aggressively linked.

## 2. Encyclopedia

**Question:** What is the full idea?

Input: complete entry.

The encyclopedic view can expose:

- definition;
- problem pressure;
- mechanism;
- examples and non-examples;
- empirical status;
- boundaries;
- relations;
- provenance;
- revisions;
- open questions.

This is the default deep-reading surface.

## 3. Concept graph

**Question:** How does this idea connect to the rest of the system?

Input: typed relations.

Useful graph modes:

- local neighborhood around one concept;
- all descendants/ancestors;
- only causal edges;
- only refinement/supersession edges;
- framework membership;
- cross-domain bridges.

Typed edges make the graph explanatory instead of decorative.

## 4. Genealogy

**Question:** Where did this idea come from?

Input:

- origin metadata;
- provenance;
- `descends-from`, `refines`, `motivates`, and supersession relations;
- semantic revision history.

This could eventually render an intellectual family tree.

A concept genealogy is distinct from a Git history: Git tells us when text changed; genealogy tells us when **meaning changed**.

## 5. Chronology

**Question:** What was the sequence of discovery?

A timeline across all entries can reveal research epochs, conceptual bursts, and when previously separate domains converged.

Approximate dates should remain approximate.

## 6. Atlas

**Question:** What territory have we explored?

An atlas groups concepts by:

- domain;
- research program;
- framework;
- cluster;
- status;
- maturity.

Unlike a graph, the atlas is curated and pedagogical.

## 7. Amber-phrase index

**Question:** What wording must not be lost?

Phrase entries can render as a quotable index linked to the larger concepts they compress.

This is intentionally different from a conventional quotations page: the phrases are preserved because they are **research-bearing compression**.

## 8. Problem-pressure index

**Question:** What problems generated our vocabulary?

This is a potentially unusual and valuable projection.

Extract the “Problem pressure” sections and group concepts by the frustration or puzzle that birthed them.

This allows reverse lookup:

> “I remember the problem I was thinking about, but not the word we invented.”

That is exactly the kind of failure a research memory system should prevent.

## 9. Concept diffs

**Question:** How did our understanding change?

A future renderer could compare semantic revision records between versions:

- original definition;
- added distinction;
- narrowed scope;
- empirical downgrade/upgrade;
- renamed concept;
- newly discovered external antecedent.

This projection treats conceptual revision as first-class history.

## 10. Research frontier

**Question:** Where is the theory unfinished?

Aggregate:

- open questions;
- contested entries;
- provenance checks;
- unresolved boundaries;
- candidate mechanisms awaiting evidence.

This turns Conceptarium into an agenda generator rather than only an archive.

## Blog relationship

The preferred architecture is:

```text
                       -> dictionary
                       -> encyclopedia
Conceptarium Markdown  -> graph
and YAML               -> genealogy
                       -> atlas
                       -> search
                       -> blog embeds
```

The blog may provide navigation, styling, essays, commentary, and interactive visualization. It should not silently fork canonical definitions.

A normal blog essay can cite or embed Conceptarium entries while remaining a separate authored artifact.

## Implementation principle

Do not choose the static-site framework yet unless the content model requires it.

The data layer should be stable enough that Astro, Next.js, Eleventy, Hugo, a custom Rust generator, or some future system could all render it.

Content architecture comes first.
