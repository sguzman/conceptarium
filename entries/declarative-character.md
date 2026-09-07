---
id: declarative-character
term: "declarative character"
type: concept
status: provisional
gloss: >-
  A character represented primarily through layered semantic properties—anatomy, phenotype, age, profession, temperament, clothing, history, and current state—from which concrete geometry and expressive behavior can be generated.
domains:
  - "character-design"
  - "procedural-generation"
  - "generative-systems"
  - "simulation"
aliases: []
origin:
  date: 2026-08
  authorship: joint
  certainty: reconstructed
  note: >-
    Recovered from the Atmos procedural-character design where semantic composition replaced direct vertex authorship as the source of truth.
relations:
  - type: specializes
    target: declarative-procedural-modeling
  - type: uses
    target: primitive-vocabulary
  - type: supports
    target: carry-their-personality-in-their-bodies
  - type: used-by
    target: generative-village
  - type: supported-by
    target: generative-modeling
---

# declarative character

## Definition

A **declarative character** is specified by meaningful properties rather than primarily by manually authored geometry.

A simplified character description might contain:

~~~text
anatomy
+ phenotype
+ age
+ profession
+ personality
+ clothing
+ current emotion
~~~

Procedures translate those properties into concrete form and behavior.

## Problem pressure

Traditional character pipelines tend to make each finished model the main artifact.

That makes systematic semantic variation expensive.

The Atmos design wanted the source of truth to remain editable at the level of meaning:

> **What kind of person is this?**

rather than:

> **Where should this vertex move?**

A declarative character allows the same semantic facts to coordinate geometry, clothing, posture, animation, and later simulation.

## Core model

~~~text
semantic layers
      ↓
constraint / composition rules
      ↓
procedural body + clothing
      ↓
animation affordances
      ↓
visible character instance
~~~

## Claims and implications

Declarative character is a specialization of declarative procedural modeling.

The semantic representation should remain authoritative after generation so that later changes—aging, profession, injury, emotion, equipment—can propagate coherently.

The architecture becomes especially powerful when the same semantic properties affect both body generation and agent behavior.

## Examples

### Semantic blacksmith

The description specifies heavy build, forward posture, profession, apron, soot, and temperament; the generator derives a coherent character.

### Aging

Changing age alters body proportions, posture, materials, and movement ranges without replacing the character with a separately authored mesh.

### Emotional state

Current emotion modulates posture and expression while leaving slower structural traits intact.

## Non-examples

A mesh with a JSON file of labels attached afterward is not declarative if those labels do not generate or constrain the character.

A conventional character creator with independent cosmetic sliders is only partially declarative if the controls lack semantic composition.

## Boundaries and failure modes

Semantic labels can become theatrical if they have evocative names but weak downstream consequences.

Another risk is overcoupling: one property may accidentally determine too many outputs and make characters stereotyped or repetitive.

Conflict resolution between semantic layers must remain explicit.

## Operationalization / evidence

A strong test is:

> Can a new coherent character be produced by changing semantic character properties without hand-editing raw geometry, while multiple visual and behavioral consequences update consistently?

If yes, the representation is genuinely declarative.

## Relations

Declarative character specializes **Declarative procedural modeling**, uses the **Primitive vocabulary**, supports the objective **carry their personality in their bodies**, is used by the **Generative village**, and is supported by the broader **Generative modeling** method.

## Provenance

### First known appearance

August 2026 during the Atmos generative-modeling roadmap.

### Immediate context

The source discussion proposed characters as compositions of anatomy, phenotype, age, profession, personality, clothing, and current emotion.

### Problem being solved

To make character generation semantically editable, compositional, and capable of coordinating many visible consequences from one source of truth.

### Conceptual ancestors

- [Declarative procedural modeling](./declarative-procedural-modeling.md)
- [Primitive vocabulary](./primitive-vocabulary.md)
- [Procedural visual language](./procedural-visual-language.md)

### External antecedents

Declarative character systems overlap with parametric modeling, procedural characters, rig-driven customization, and semantic asset generation. The Conceptarium term names the semantic source-of-truth architecture.

## Open questions

- What is the minimal stable semantic schema for a character?
- How should conflicting traits compose?
- Which traits should be persistent, learned, inherited, or state-dependent?

## Revision history

- 2026-09-07 — Promoted from the Generative systems queue using the recovered August 2026 Atmos architecture and Conceptarium master lexicon.
