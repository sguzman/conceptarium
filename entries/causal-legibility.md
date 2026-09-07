---
id: causal-legibility
term: "causal legibility"
type: concept
status: provisional
gloss: >-
  The degree to which an outcome can be understood through an inspectable causal structure whose relevant mechanisms, dependencies, evidence, and uncertainty are visible enough to test and revise.
domains:
  - "epistemology"
  - "ai"
  - "causal reasoning"
  - "explanation"
aliases: []
origin:
  date: 2026-09-05
  authorship: unknown
  certainty: reconstructed
  note: >-
    The term survived in the former promotion queue and appears conceptually in Paranoid totalization, where totalizing stories are attractive because they convert uncertainty into causal legibility. This entry distinguishes genuine legibility from merely having a coherent causal story.
relations:
  - type: supports
    target: adversarial-verification
  - type: supported-by
    target: epistemic-chain-of-custody
  - type: threatened-by
    target: narrative-coherence-pressure
  - type: contrasts-with
    target: false-precision
  - type: associated-with
    target: diegetic-truth
---

# causal legibility

## Definition

**Causal legibility** is the degree to which the causal structure behind an outcome is available for inspection rather than remaining an opaque assertion.

A causally legible explanation makes visible:

- what variables or events matter;
- how they are connected;
- what evidence supports each link;
- which links are inferred rather than observed;
- where uncertainty remains;
- what observation would weaken the account.

The concept distinguishes **having a causal story** from **having a causally inspectable model**.

## Problem pressure

The canonical **Paranoid totalization** entry says totalization is attractive because it converts painful uncertainty into **causal legibility**.

That observation exposes an ambiguity.

Humans and models genuinely need causal structure. But a totalizing explanation can create **counterfeit legibility**: everything has a cause inside the story, yet nothing can falsify the story.

The queue term therefore needs a mature definition that preserves causal understanding while separating it from narrative closure.

## Core model

~~~text
opaque outcome
     ↓
candidate mechanisms
     ↓
evidence + provenance + dependency structure
     ↓
tests / interventions / counterfactuals
     ↓
CAUSAL LEGIBILITY
~~~

Counterfeit version:

~~~text
opaque outcome
     ↓
smooth explanation
     ↓
no visible uncertainty / no falsification path
     ↓
narrative coherence mistaken for causation
~~~

## Claims and implications

Causal legibility is graded.

A model can explain some local mechanisms while leaving upstream causes opaque.

The concept also does not require complete causal identification. A partially legible model that marks unknown links can be more epistemically mature than a fully specified but invented story.

For AI, the useful standard is not “give me a cause” but **show what makes this causal claim inspectable and defeasible**.

## Examples

### Software debugging

A causally legible diagnosis links logs, timing, subsystem behavior, and a reproducible failure condition.

### Historical explanation

A historian distinguishes documented policy decisions, inferred motives, and uncertain causal contribution rather than presenting one seamless master motive.

### Narrative analysis

A story makes causal relations between events legible even though no narrator possesses all of them; the analyst separates diegetic truth from narrator access.

## Non-examples

A plausible explanation is not automatically causally legible.

Nor is a long explanation. Detail without testable dependency structure can be false precision.

Correlation alone may be evidentially useful while still leaving causal structure opaque.

## Boundaries and failure modes

The term can overpromise in domains where causation is genuinely diffuse or underdetermined.

Some causal systems are only partially observable, and insisting on a clean causal graph can manufacture structure that reality does not supply.

Legibility must therefore include **visible uncertainty**, not just visible arrows.

## Operationalization / evidence

For a causal account, ask:

1. what is observed?
2. what is inferred?
3. what mechanisms connect them?
4. what provenance supports each link?
5. what alternatives remain?
6. what intervention or evidence would distinguish them?

A causally legible account exposes answers rather than hiding them behind rhetorical confidence.

## Relations

Causal legibility supports **Adversarial verification**, is supported by an **Epistemic chain of custody**, is threatened by **Narrative-coherence pressure**, contrasts with **False precision**, and is associated with **Diegetic truth** where causal structure exists independently of what a narrator knows.

## Provenance

### First known appearance

Present in the former promotion queue by early September 2026. The exact originating turn was not recovered.

### Immediate context

The phrase is explicitly implicated in the canonical Paranoid totalization entry: totalizing theories create relief by turning uncertainty into causal legibility.

### Problem being solved

To preserve the legitimate demand for causal understanding without rewarding self-sealing stories that merely feel explanatory.

### Conceptual ancestors

- [Paranoid totalization](./paranoid-totalization.md)
- [Adversarial verification](./adversarial-verification.md)
- [Epistemic chain of custody](./epistemic-chain-of-custody.md)
- [Narrative-coherence pressure](./narrative-coherence-pressure.md)

### External antecedents

Neighbors include causal inference, mechanistic explanation, interpretability, causal graphs, and scientific explanation. The Conceptarium emphasis is on inspectability plus falsification surface.

## Open questions

- Can causal legibility be scored without privileging overly simple models?
- How should diffuse multi-causal systems represent partial legibility?
- What distinguishes genuinely mechanistic explanation from a rhetorically detailed causal story?

## Revision history

- 2026-09-07 — Promoted from the Epistemology / AI queue; local meaning reconstructed from surviving canonical entries and the former promotion queue.
