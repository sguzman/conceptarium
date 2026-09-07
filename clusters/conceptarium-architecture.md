# Conceptarium Architecture

This cluster tracks the architecture that lets Conceptarium preserve conceptual discovery immediately while delaying expensive semantic and ontological work until it is justified.

The governing distinction is:

~~~text
Predicate presence
"Conceptarium knows this conceptual object exists."
                ≠
Ontological materialization
"Conceptarium has progressively defined, situated, related, and integrated it."
~~~

Canonical entries:

- [Predicate presence](../entries/predicate-presence.md)
- [Ontological materialization](../entries/ontological-materialization.md)

## Lossless discovery pipeline

~~~text
idea / phrase appears
        ↓
stable registry identity
        ↓
predicate presence
        ↓
concept may now be referenced, related,
queued, searched, counted, and projected
        ↓
optional later development
        ↓
ontological materialization
        ↓
full entry / relations / evidence / provenance / placement
~~~

The architectural principle is:

> **Preserve existence cheaply; develop meaning deliberately.**

## Why the distinction matters

Without predicate presence, Conceptarium would have to choose between interrupting active thought to write a full article or risking conceptual loss.

Without ontological materialization, the registry would become a flat inventory of names with no path toward deeper research structure.

The two concepts therefore solve opposite failure modes:

~~~text
no cheap presence
    → discovery loss or premature formalization

no deliberate materialization
    → permanent backlog of semantically thin names
~~~

## Two independent axes

Conceptarium deliberately separates **entry materialization** from **ontology state**.

### Entry materialization

~~~text
registry-only
entry
~~~

This describes whether a full canonical Markdown semantic object exists.

### Ontology state

~~~text
unassessed
unplaced
roughly-classified
domain-placed
related
deeply-integrated
~~~

This describes how deeply situated the object is in the theoretical universe.

The axes should not be collapsed.

A full entry can remain unassessed.

A registry-only concept can still acquire some broad classification.

## Canonical source boundary

~~~text
registry/concepts.yml
    canonical for identity / presence
            +
entries/*.md
    canonical for materialized semantics
            ↓
disposable projections / indexes / databases
~~~

This prevents SQLite, SurrealDB, Tantivy, future Oxigraph, or other backends from becoming accidental competing sources of truth.

## Promotion queue

The promotion queue is not separately authored.

~~~text
registry
  ↓
filter materialization == registry-only
  ↓
build/promotion-queue.md
~~~

This makes unfinished work an executable projection of predicate presence.

Promotion changes the registry state; the queue changes automatically.

## Relation targets

Predicate presence also changes graph integrity.

~~~text
relation target has full entry
    → valid materialized target

relation target is registry-only
    → valid unfinished target

relation target absent from registry
    → structural error
~~~

This lets graph structure get ahead of article writing without creating dangling conceptual references.

## Editorial doctrine

The architecture supports several repository rules:

- preserve before polishing;
- unknown metadata beats invented metadata;
- queue aggressively, materialize deliberately;
- stable identity should survive semantic enrichment;
- vocabulary maturity is not scientific truth;
- a concept does not need a full ontology to deserve preservation;
- full entries should be deep enough to justify their existence.

## Rust implementation

The Rust tooling operationalizes the distinction through:

- registry capture and materialization commands;
- stable concept resolution;
- validation;
- registry-aware graph traversal;
- promotion-queue generation;
- machine-readable registry projections;
- downstream disposable query backends.

The architectural concepts therefore describe executable repository behavior, not only editorial philosophy.

## Research frontier

- Should ontology state remain one ladder or eventually become several independent dimensions?
- What automated signals can identify high-value promotion candidates without rewarding raw frequency?
- How should duplicate or near-duplicate registry concepts be reconciled while preserving provenance?
- Should registry-only concepts support lightweight evidence and relation metadata before entry creation?
- How can future RDF/ontology work preserve the cheap-capture property of predicate presence?

## Revision history

- **2026-09-05:** Concept Registry architecture introduced; predicate presence separated from ontological materialization.
- **2026-09-07:** Governing concepts promoted and architecture cluster materialized.
