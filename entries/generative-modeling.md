---
id: generative-modeling
term: "generative modeling"
type: method
status: provisional
gloss: >-
  Building models through reusable rules, parameters, compositions, and constraints that can produce families of coherent instances rather than authoring each finished instance manually.
domains:
  - "procedural-generation"
  - "graphics"
  - "generative-systems"
  - "modeling"
aliases: []
origin:
  date: 2026-08
  authorship: pre-existing
  certainty: reconstructed
  note: >-
    A broad pre-existing term adopted as the general method above the project's more specific declarative procedural modeling architecture.
relations:
  - type: generalizes
    target: declarative-procedural-modeling
  - type: supports
    target: declarative-character
  - type: supports
    target: procedural-visual-language
  - type: uses
    target: primitive-vocabulary
  - type: used-by
    target: generative-village
---

# generative modeling

## Definition

**Generative modeling** builds artifacts by specifying a system capable of producing many coherent instances.

The authored object is therefore not only the final artifact.

It is also:

- the rule set;
- parameter space;
- primitive vocabulary;
- composition grammar;
- constraints;
- variation logic.

The method asks how to construct **a family of possible objects** rather than one object at a time.

## Problem pressure

The broader generative-world project wanted characters, buildings, props, and social worlds to remain editable and compositional.

Manual one-off authoring scales poorly when every new instance requires direct intervention.

Generative modeling shifts effort toward reusable structure.

The goal is not maximal randomness. It is **controlled expressive possibility**.

## Core model

~~~text
rules + parameters + primitives + constraints
        ↓
generative model
        ↓
many coherent instances
        ↓
observed failures / new requirements
        ↓
model revision
~~~

## Claims and implications

Generativity is valuable when variation remains coherent and inspectable.

A good generative model purchases:

- reuse;
- combinatorial range;
- semantic editing;
- consistency;
- faster iteration;
- explicit design-space structure.

The method becomes weak when most outputs require bespoke cleanup.

## Examples

### Character family

One semantic body system produces many ages, builds, professions, and temperaments.

### Architecture

A building grammar produces structurally related houses from parameterized bays, roofs, materials, and use patterns.

### Vegetation

Reusable branching and growth rules produce coherent plant families rather than manually authored copies.

## Non-examples

Random noise is not generative modeling merely because it produces many outputs.

A folder of manually created variants is not a generative model.

An opaque generator may be generative in a broad sense but does not satisfy the project's preference for explicit inspectable structure.

## Boundaries and failure modes

The broad phrase should not swallow its more precise descendants.

**Declarative procedural modeling** adds a semantic source-of-truth requirement.

**Procedural visual language** adds a coherent reusable grammar.

The general term should remain the umbrella method.

## Operationalization / evidence

Ask:

> Is the primary authored artifact a reusable rule/parameter system that can generate materially different coherent instances without instance-by-instance reconstruction?

Measure how often manual patches are required and how much of the output space remains semantically controllable.

## Relations

Generative modeling generalizes **Declarative procedural modeling**, supports **Declarative character** and **Procedural visual language**, uses the **Primitive vocabulary**, and is used by the world-scale **Generative village** project.

## Provenance

### First known appearance

August 2026 in the Atmos roadmap, with broader pre-existing usage inherited from procedural and generative modeling traditions.

### Immediate context

The master lexicon preserved the definition as model-building through rules and parameters that produce many coherent instances rather than authoring each instance manually.

### Problem being solved

To give the procedural-world project a general method for building reusable possibility spaces rather than isolated assets.

### Conceptual ancestors

- [Declarative procedural modeling](./declarative-procedural-modeling.md)
- [Primitive vocabulary](./primitive-vocabulary.md)
- [Procedural visual language](./procedural-visual-language.md)

### External antecedents

Generative and procedural modeling are established fields in graphics, CAD, architecture, and content generation. Conceptarium uses the term broadly and reserves more specific local terms for semantic and stylistic constraints.

## Open questions

- Which parts of a model should remain hand-authored rather than generalized?
- How should expressive range be measured?
- When does a general generator become less useful than several specialized ones?

## Revision history

- 2026-09-07 — Promoted from the Generative systems queue using the recovered August 2026 Atmos architecture and Conceptarium master lexicon.
