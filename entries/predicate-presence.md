---
id: predicate-presence
term: Predicate presence
type: concept
status: canonical
gloss: >-
  The condition in which a conceptual object has enough durable registered identity
  to be named, referenced, related, queued, indexed, or projected even though its
  definition and ontology may remain undeveloped.
domains:
  - knowledge-representation
  - knowledge-management
  - research-infrastructure
  - ontology
aliases: []
origin:
  date: 2026-09-05
  authorship: joint
  certainty: exact
  note: >-
    Introduced as a load-bearing Conceptarium architecture concept when the canonical
    Concept Registry was added and documented on 2026-09-05.
relations:
  - type: contrasts-with
    target: ontological-materialization
  - type: enables
    target: ontological-materialization
---

# Predicate presence

## Definition

**Predicate presence** is the minimum durable condition under which Conceptarium can truthfully answer:

> “Does this conceptual object exist in our research memory?”

with:

> “Yes.”

Predicate presence does **not** require Conceptarium to know everything else about the object.

A concept with predicate presence may still lack:

- a full definition;
- a canonical entry;
- domains;
- evidence;
- boundaries;
- provenance detail;
- ontology placement;
- mature relations.

What it does possess is enough stable identity to survive, be addressed, and participate in repository operations.

~~~text
concept noticed
      ↓
stable identity registered
      ↓
PREDICATE PRESENCE
      ↓
can be named / queued / related / searched / projected
      ↓
semantic development may happen later
~~~

## Problem pressure

Conceptarium was producing vocabulary faster than it could responsibly formalize it.

The old architecture created a destructive choice:

~~~text
fully develop concept now
        OR
risk losing the concept
~~~

That is unacceptable for a research-memory system.

A conversation may produce twenty useful terms while only two deserve immediate encyclopedic treatment. Requiring definitions, ontology, evidence, examples, and relations at capture time either interrupts thinking or encourages shallow, fabricated formalization.

Predicate presence was introduced to create a much cheaper threshold:

> **Thinking of a concept should be enough to preserve its existence. Understanding it can happen later.**

The repository can therefore be lossless about **identity** without pretending to be complete about **meaning**.

## Canonical representation

Predicate presence is represented in the Concept Registry:

~~~yaml
- id: stable-concept-id
  term: "Concept name"
  presence: registered
  materialization: registry-only
  ontology_state: unplaced
~~~

The exact metadata can be richer, but the essential fact is that the object now has durable identity.

The registry is canonical for this presence claim.

## What predicate presence enables

A registered concept can be:

- referenced by stable ID;
- targeted by typed relations;
- placed in the generated promotion queue;
- counted in corpus statistics;
- returned by registry-aware queries;
- represented as a placeholder graph node;
- carried into machine-readable projections;
- enriched later without changing identity.

This matters because ontology can now **lag behind discovery** without erasing discovery.

## Predicate presence versus entry existence

A Markdown entry is a stronger state.

~~~text
predicate presence
    "the conceptual object exists here"

full entry
    "the conceptual object has developed semantic content here"
~~~

Every materialized entry should have predicate presence.

Not every concept with predicate presence should have an entry.

That asymmetry is deliberate.

## Predicate presence versus truth

Registration is not endorsement.

A registered object may be:

- speculative;
- false;
- poorly named;
- later deprecated;
- merely a research question;
- a relation target awaiting clarification.

Predicate presence says:

> “This is a conceptual object worth not losing.”

It does not say:

> “This concept is correct.”

## Predicate presence versus ontology placement

These are independent axes.

A concept can be registered while its ontology state remains unplaced, unassessed, roughly-classified, domain-placed, related, or deeply-integrated.

The system therefore avoids inventing classification simply because a term has been captured.

## Examples

A conversation produces **misandric permission structure**, but its standalone definition is not yet stable.

Registering the term gives it predicate presence without forcing premature materialization.

A full entry relates to **totalization**, but totalization has not yet been independently developed.

Registering the relation target preserves graph integrity without inventing a stub.

A new research phrase appears during an intense discussion. The phrase is captured in seconds and later receives a full entry during a batch promotion pass.

## Non-examples

Predicate presence is not:

- a complete ontology;
- a canonical definition;
- scientific validation;
- a placeholder Markdown article;
- a requirement that every registered object eventually be promoted;
- a guarantee that the current display term will remain preferred forever.

## Boundaries and failure modes

### Registry dumping without curation

Cheap capture can produce a large backlog.

That is acceptable only if registry identity remains meaningful and queue tooling makes unfinished material visible.

### Fake ontology

The architecture specifically forbids inventing domains or relations merely to make a captured record look complete.

### Identity instability

Stable IDs matter.

Renaming a display term should not casually destroy the registered identity that relations and projections depend on.

### Registration inflation

Not every noun phrase deserves predicate presence.

The threshold is whether losing the conceptual identity would matter to future reasoning.

## Operationalization

A concept has predicate presence when:

1. it has a unique stable ID in registry/concepts.yml;
2. its term is registered;
3. it can be resolved by repository tooling;
4. relations may legally target it;
5. projections treat it as an existing node regardless of entry materialization.

The Rust validator enforces the structural consequences of this state.

## Relations

### Ontological materialization

Predicate presence **contrasts with** ontological materialization because presence is cheap and minimally committal while materialization progressively develops semantic and theoretical content.

Predicate presence also **enables** materialization by supplying a stable identity that later enrichment can preserve.

## Provenance

### First known appearance

September 5, 2026.

### Repository archaeology

The architecture was introduced in a tightly clustered series of commits:

- 0e680b8 — **Add canonical concept registry**
- 167ed1a — **Document predicate presence and concept registry**
- 7291112 — **Validate concept registry and predicate presence**
- 993a39d — **Derive promotion queue from concept registry**
- 3e360d3 — **Separate registry presence from entry schema**

This sequence shows that predicate presence was not merely documentation vocabulary. It was implemented as a repository invariant, validator rule, and projection architecture.

### Problem being solved

How can Conceptarium preserve every important conceptual discovery immediately without forcing full semantic development at the moment of discovery?

## Open questions

- What is the right threshold for granting predicate presence to a phrase?
- Should registry-only concepts acquire lightweight automated similarity or duplicate checks?
- Which ontology operations should be legal before semantic materialization?
- How should abandoned but historically important registry-only concepts be represented?

## Revision history

- **2026-09-05:** Predicate-presence architecture introduced with the canonical Concept Registry and registry-aware validation/projections.
- **2026-09-07:** First standalone canonical entry materialized.
