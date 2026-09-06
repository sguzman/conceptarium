# Concept Registry

The **Concept Registry** is Conceptarium's canonical presence layer.

Its governing distinction is:

~~~text
PREDICATE PRESENCE
"Conceptarium knows this conceptual object exists."
                ≠
ONTOLOGICAL MATERIALIZATION
"Conceptarium has fully defined, classified, related, and integrated it."
~~~

A concept may therefore be a valid Conceptarium object before it has a full Markdown entry.

## Why the registry exists

Conceptual work is often produced faster than it can be formalized.

A session may produce a useful term when there is no time or energy to write:

- a definition;
- domains;
- relations;
- examples;
- boundaries;
- provenance;
- an ontology placement;
- a full article.

The repository must not force a choice between **fully developing the concept now** and **losing it**.

The registry provides a durable middle state.

## Canonical source

The registry lives at:

`registry/concepts.yml`

It is canonical for **concept identity and presence**.

Full semantic treatment remains canonical in:

`entries/<id>.md`

when such an entry exists.

The registry must not duplicate long-form definitions. Its job is identity, presence, materialization state, and capture metadata.

## Minimal valid concept

The smallest useful registered concept can be:

~~~yaml
- id: misandric-permission-structure
  term: "misandric permission structure"
  presence: registered
  materialization: registry-only
  ontology_state: unplaced
  registered_on: 2026-09-05
~~~

That is enough for the concept to be:

- referred to by stable ID;
- targeted by graph relations;
- shown in the promotion queue;
- indexed by future search or embedding systems;
- enriched later without changing identity.

No definition is required at capture time.

## Materialization states

### `registry-only`

The concept has predicate presence but no full canonical entry yet.

This is the normal state for low-effort capture and the source of the generated promotion queue.

### `entry`

The concept has a full canonical Markdown entry.

The registry record points to it:

~~~yaml
materialization: entry
entry: entries/cultural-refraction.md
~~~

The entry remains authoritative for its definition, type, status, gloss, domains, origin, relations, and article body.

## Ontology state

Ontology state is intentionally independent from materialization.

Allowed states are currently:

- `unassessed` — no formal review under the new ontology model yet;
- `unplaced` — deliberately preserved without ontology placement;
- `roughly-classified` — broad category known, deeper placement unresolved;
- `domain-placed` — placed into one or more explicit knowledge domains;
- `related` — domain placement plus meaningful conceptual relations;
- `deeply-integrated` — substantially situated in the wider theoretical universe.

A full article may still be `unassessed`.

A tiny captured concept may eventually become `roughly-classified` before it has a full definition.

Do not infer semantic maturity from ontology state.

## Predicate presence

**Predicate presence** means a concept has enough durable identity to participate in repository operations.

A concept with predicate presence may be:

- named;
- referenced;
- related;
- queued;
- searched;
- embedded;
- counted;
- promoted later.

This prevents a useful term from disappearing merely because its ontology is unfinished.

## Ontological materialization

**Ontological materialization** is the progressive enrichment of a registered concept.

A possible trajectory is:

~~~text
term noticed
    ↓
registry presence
    ↓
capture note / problem pressure
    ↓
rough classification
    ↓
full entry
    ↓
typed relations
    ↓
domain / subject / research-program placement
    ↓
deeper ontology and evidence
~~~

The stages need not occur in this order.

Unknown metadata is preferable to invented metadata.

## Promotion queue

The promotion queue is **not a second source of truth**.

It is a projection:

~~~text
Concept Registry
      ↓
filter materialization == registry-only
      ↓
Promotion Queue
~~~

`tools/project.py` generates:

`build/promotion-queue.md`

Therefore a concept enters the queue by entering the registry.

When it gains a full entry and its registry record changes to `materialization: entry`, it leaves the promotion queue automatically.

## Relation targets

Relations now distinguish three cases:

~~~text
target has full entry
    → materialized concept

target is registry-only
    → valid unmaterialized concept

target absent from registry
    → broken conceptual reference
~~~

A relation to a registry-only concept is valid.

A relation to an ID with no registry presence is a structural error.

This turns former "dangling concepts" into explicit research inventory.

## Lazy capture workflow

The intended low-energy workflow is:

~~~bash
python tools/registry.py capture "term here" --date 2026-09-05
~~~

Optional context can be added:

~~~bash
python tools/registry.py capture "term here" \
  --date 2026-09-05 \
  --group "Film theory" \
  --note "Came up while thinking about montage and narrative knowledge."
~~~

Then stop.

No ontology work is required.

Later, another session can materialize or classify the concept.

## Promotion workflow

A later integration pass can create the canonical entry and then run:

~~~bash
python tools/registry.py materialize concept-id --entry entries/concept-id.md
~~~

The validator ensures the registry record and entry agree.

## Architectural principle

> **Thinking of a concept should be enough to preserve its existence. Understanding it can happen later.**

This is losslessness applied not merely to prose, but to conceptual identity itself.
