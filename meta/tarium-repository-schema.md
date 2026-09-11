# Tarium Repository Schema

A **Tarium repository** is a repository-scale knowledge system designed to preserve several different kinds of intellectual material without collapsing them into one another.

The family schema is intentionally plural.

A Tarium is not merely:

- a fact database;
- a personal notebook;
- an ontology;
- an argument archive;
- a confessional diary;
- an analytical essay collection.

It is an architecture in which several distinct layers coexist as **first-class, locally sovereign forms of knowledge**.

The minimum conceptual schema contains four layers:

1. **objective ledger**;
2. **ontological structure**;
3. **integration and analysis**;
4. **phenomenological / confessional layer**.

No one layer is permitted to impersonate another.

---

## 1. Objective ledger

The **objective ledger** records externally auditable facts, observations, source claims, measurements, events, dates, identities, textual evidence, or other propositions whose intended standing does not depend upon Salvador's personal interpretation of them.

Its governing question is:

> **What can this repository claim about the world as evidence, record, observation, or source-supported fact?**

The objective ledger should preserve, where relevant:

- source and provenance;
- date and temporal scope;
- confidence or uncertainty;
- whether the record is direct evidence, reported evidence, reconstruction, or inference;
- contradictions between sources;
- unresolved factual disputes;
- distinction between observation and interpretation.

The objective ledger is not required to be complete or infallible.

It is required to make a disciplined attempt at **world-facing auditability**.

### Anti-collapse rule

```text
"I believe X"
    ≠
"X is established as fact"
```

A personal interpretation may motivate investigation of a proposition without upgrading that proposition into the objective ledger.

---

## 2. Ontological structure

The **ontological structure** records what kinds of things the repository recognizes and how those things relate.

Its governing question is:

> **What objects, types, distinctions, relations, mechanisms, states, and structures are required to make this domain legible?**

This layer may encode:

- object types;
- categories;
- taxonomies;
- relations;
- identities;
- states and transitions;
- part/whole structure;
- causal or explanatory mechanisms;
- significance relations;
- inheritance or specialization;
- temporal structure;
- provenance relations;
- constraints and invariants.

Ontology does more than store names.

It makes the domain **structurally queryable**.

A fact can therefore exist in the objective ledger while the ontology determines what sort of fact it is, what it concerns, what it relates to, and why it matters structurally.

### Anti-collapse rule

```text
ontology
    ≠
truth guarantee
```

The existence of a type, relation, or model in the ontology does not prove that every claim instantiated through it is true.

Ontology provides representational structure, not epistemic immunity.

---

## 3. Integration and analysis

The **integration and analysis layer** contains arguments, interpretations, syntheses, audits, comparisons, explanatory models, derived claims, evaluations, and attempts to connect otherwise separate facts and structures.

Its governing question is:

> **Given the available facts and ontology, what follows, what might explain them, how do they fit together, and where do the claims succeed or fail?**

This layer may contain:

- derived claims;
- arguments;
- competing interpretations;
- explanatory hypotheses;
- causal analysis;
- comparative analysis;
- audits;
- criticism;
- synthesis across sources;
- model construction;
- implications;
- predictions;
- unresolved contradictions;
- explicit speculation.

This is where a Tarium becomes more than an archive.

It is allowed to reason.

But reasoning must remain distinguishable from the evidence from which it proceeds.

### Anti-collapse rule

```text
evidence
    ↓ supports / constrains
analysis

analysis
    ≠
evidence itself
```

A strong argument may deserve reuse and canonical status without being rewritten as an objective fact.

---

## 4. Phenomenological / confessional layer

The **phenomenological layer** records the world as experienced, interpreted, desired, feared, remembered, narrated, or confessed by the human principal.

Its governing question is:

> **What is this like from inside Salvador's experience, and what does Salvador currently make of it?**

This layer is first-class.

It is not an embarrassing remainder left over after the repository has extracted the supposedly serious material.

It may contain:

- first-person experience;
- personal interpretation;
- subjective significance;
- emotional valence;
- memory;
- desire;
- fear;
- self-description;
- self-narration;
- confession;
- moral intuition;
- aesthetic reaction;
- private theory about one's own motives;
- contradictory self-understandings;
- unresolved phenomenology.

A confessional artifact may be valuable even when some of its propositions are false.

Its value may lie in accurately preserving:

- what was believed;
- what was felt;
- what appeared salient;
- what interpretation governed action;
- what story the person was telling about themselves or the world at that time.

### Anti-collapse rule

```text
phenomenological truth
"this is how it appeared / felt / was interpreted"
        ≠
objective truth
"this is how the world independently was"
```

The distinction protects both sides.

The objective ledger is protected from autobiographical inflation.

The phenomenological layer is protected from erasure merely because an interpretation later proves mistaken.

---

# Layer sovereignty

The four layers are **distinct, sovereign, and first class**.

This means:

- no layer is merely a staging area for another;
- no layer automatically outranks the others for every purpose;
- each has its own standards of validity;
- each may preserve material that would be illegitimate if silently moved into another layer;
- cross-layer translation must be explicit;
- disagreement between layers is representable rather than forcibly reconciled.

The architecture should permit statements such as:

```text
OBJECTIVE LEDGER
Evidence currently supports A.

ONTOLOGICAL STRUCTURE
A is an instance of type T and relates to B through relation R.

ANALYSIS
A and B together suggest hypothesis H.

PHENOMENOLOGY
Salvador experiences A as evidence for personal narrative N.
```

All four may coexist without one silently swallowing the others.

---

# Cross-layer relations

The layers are sovereign but not isolated.

A mature Tarium should support explicit relations across them.

Examples:

```text
phenomenological artifact
    motivates-investigation -> factual question

objective record
    supports -> analytical claim

objective record
    contradicts -> confessional interpretation

analytical claim
    uses -> ontological distinction

ontology
    structures -> factual record

confessional artifact
    exemplifies -> phenomenological pattern
```

The general rule is:

> **Integration is achieved by typed relations, not by collapsing provenance.**

---

# Claims need layer identity

Whenever practical, a proposition should expose what kind of standing it is claiming.

At minimum, repositories should be able to distinguish:

- observed / sourced fact;
- reported claim;
- inferred claim;
- analytical interpretation;
- speculative hypothesis;
- normative judgment;
- phenomenological report;
- confessional artifact.

These may share vocabulary and refer to the same real-world substrate.

They should not share epistemic status merely because they share a subject.

---

# One substrate, several projections

A single real-world thing may legitimately appear in several layers.

That is not duplication in the pathological sense when the **intentional projection differs**.

For example:

```text
same event
  ├─ objective layer: what happened
  ├─ ontology: what kind of event it is and what it relates to
  ├─ analysis: what the event implies or explains
  └─ phenomenology: what the event meant or felt like to Salvador
```

The repository should prefer explicit cross-linkage over pretending one artifact can exhaust every projection.

---

# Repository instantiation

A new Tarium repository should normally declare, early in its life:

1. **Domain** — what intentional domain or problem-space the repository owns.
2. **Canonical objects** — what kinds of things it expects to represent.
3. **Objective ledger contract** — what counts as evidence or externally auditable record in that domain.
4. **Ontology contract** — how types, relations, identities, states, and significance are represented.
5. **Analysis contract** — where derived claims, arguments, audits, synthesis, and speculation live.
6. **Phenomenology contract** — how first-person interpretation and confession are preserved without being mistaken for external fact.
7. **Cross-layer relation rules** — how material may cite, support, contradict, motivate, or reinterpret material in another layer.
8. **Provenance rules** — how the repository remembers where claims and artifacts came from.
9. **Local sovereignty** — which decisions belong to this repository and are not imposed by Conceptarium.
10. **Projection / tooling rules** — which indexes, databases, rendered views, or search systems are disposable projections rather than canonical state.

Not every young repository must implement all of these mechanically on day one.

But the distinctions should exist conceptually from the beginning so later growth does not force destructive reclassification.

---

# Minimal abstract layout

A Tarium may choose different physical directories, but the logical architecture resembles:

```text
<tarium>/
├── objective/        # evidence, facts, source-backed records
├── ontology/         # types, relations, schemas, identities, structures
├── analysis/         # arguments, synthesis, audits, derived claims
├── phenomenology/    # first-person interpretation and confession
├── registry/         # predicate presence / stable identities where useful
├── meta/             # local constitutional and procedural documents
└── projections/      # generated views; normally non-canonical
```

This is a **logical schema, not a mandatory folder layout**.

A repository may implement the layers in YAML, Markdown, SQL, Rust types, graph data, directories, frontmatter, or another representation appropriate to its domain.

The semantic distinction is mandatory; the filesystem spelling is not.

---

# Anti-collapse constitution

A Tarium should resist at least these conversions:

```text
confession -> fact
fact -> interpretation
interpretation -> ontology
ontology -> proof
feeling -> external claim
source claim -> verified fact
analysis -> evidence
canonical wording -> empirical certainty
repository convenience -> cross-repository sovereignty
```

The point is not to weaken claims.

The point is to know **what kind of claim is being made** so strong claims can be made without semantic smuggling.

---

# Relationship to Tarium Procedure

[Tarium Procedure](./tarium-procedure.md) governs how testimony, evidence, claims, accusations, interpretations, and uncertainty are handled.

This schema governs **where those different epistemic objects live and how they may relate**.

The Procedure says, among other things:

> **No suppression of testimony. No immunity from audit.**

The repository schema gives that principle a durable home:

- testimony has a first-class phenomenological/confessional layer;
- evidence has a first-class objective layer;
- interpretation has a first-class analytical layer;
- structural representation has a first-class ontological layer.

None needs to masquerade as another in order to be preserved.

---

# Relationship to Tarium Register

[Tarium Register](./tarium-register.md) is a prose realization of the Procedure.

The Repository Schema is not a prose style.

A repository may use the Register across all four layers, but the Register should make layer identity clearer rather than blur it.

The same serene institutional voice may therefore say:

- **Objective:** “The available record establishes…”
- **Analytical:** “The present evidence supports, but does not compel, the inference…”
- **Phenomenological:** “The confessional record describes the experience as…”
- **Ontological:** “For repository purposes, this is represented as an instance of…”

Style remains continuous while epistemic standing remains explicit.

---

# Governing synthesis

The general Tarium repository architecture is:

```text
                     TARIUM
                        │
        ┌───────────────┼───────────────┐
        │               │               │
        ↓               ↓               ↓
 OBJECTIVE          ONTOLOGICAL      ANALYTICAL
  LEDGER             STRUCTURE       INTEGRATION
        \               │               /
         \              │              /
          └────── explicit relations ──┘
                        │
                        ↓
              PHENOMENOLOGICAL /
                 CONFESSIONAL
                    LAYER
```

The diagram is not hierarchical.

The four layers are coequal in repository legitimacy while differing in epistemic function.

The deepest rule is:

> **Preserve fact, structure, analysis, and experience as distinct sovereign layers, then integrate them explicitly rather than collapsing them implicitly.**

## Revision history

- **2026-09-11:** Initial family-level repository schema established with objective, ontological, analytical, and phenomenological/confessional layers as distinct first-class sovereign structures.
