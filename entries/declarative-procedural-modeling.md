---
id: declarative-procedural-modeling
term: Declarative procedural modeling
type: method
status: canonical
gloss: A modeling method in which an author specifies what an object or character is in semantic, compositional, and parameterized terms and procedures generate the concrete geometry or representation from that description.
domains:
  - procedural-generation
  - graphics
  - modeling
  - generative-systems
aliases:
  - declarative modeling
origin:
  date: 2026
  authorship: joint
  certainty: reconstructed
relations:
  - type: uses
    target: primitive-vocabulary
  - type: produces
    target: procedural-visual-language
  - type: supports
    target: generative-village
  - type: complements
    target: convergent-architecture
---

# Declarative procedural modeling

## Definition

**Declarative procedural modeling** is the practice of describing **what an object is** and allowing procedures to determine the concrete geometry that realizes that description.

Instead of authoring vertices directly, the author specifies semantic structure.

For a character, that might include:

- body proportions;
- age;
- phenotype;
- posture;
- clothing layers;
- profession;
- mood;
- personality;
- exaggeration style.

For a building:

- structural bays;
- roof type;
- material system;
- civic hierarchy;
- window rhythm;
- ornament density;
- age/weathering.

The generator then composes primitives and procedures into a concrete instance.

## Problem pressure

The method emerged from wanting a modeling system that is both:

- highly generative;
- highly inspectable.

Traditional mesh authoring produces one finished artifact.

Pure generative models can produce variety but often hide the internal compositional logic.

The desired alternative was:

> **Describe the thing as a system of meaningful parts, then let geometry follow.**

This fits the broader preference for declarative systems: parameters should expose the conceptual structure of the artifact.

## Core model

```text
semantic description
      ↓
compositional rules
      ↓
procedural operations
      ↓
geometry
      ↓
rendered instance
```

The semantic description remains editable after generation.

## Why declarative matters

A declarative character can say:

```text
age: elderly
build: narrow
posture: forward
profession: farmer
temperament: severe
clothing: layered-workwear
```

Those values can influence multiple geometric and stylistic decisions consistently.

The alternative is to sculpt all consequences manually.

## Why procedural matters

Declarative labels without procedures are inert.

The procedural layer turns semantics into construction:

- taper limb;
- change spine curve;
- alter shoulder width;
- layer clothing;
- deform face;
- adjust stance;
- change material roughness.

## Primitive vocabulary

The method works best with a small reusable vocabulary of constructive operations.

A compact primitive set acts like an alphabet.

The combinatorics create richness.

## Procedural visual language

When the same primitives and rules are reused across many objects, the system develops a recognizable **visual grammar**.

That is stronger than merely having a consistent shader.

## Non-examples

Declarative procedural modeling is not:

- ordinary manual mesh editing;
- prompting an opaque image generator;
- a collection of random parameters;
- procedural noise alone;
- parametric CAD without semantic composition.

The essential feature is a meaningful mapping from **what the thing is** to **how it is constructed**.

## Failure modes

### Semantic theater

Parameters have evocative names but barely affect output.

### Combinatorial incoherence

Individually valid rules interact badly.

### Primitive explosion

The system grows hundreds of special-case primitives instead of learning to compose.

### Hidden manual patches

Every important output requires bespoke fixes, defeating generativity.

### False declarativity

The author still needs to think in vertex-level details because semantic controls are too weak.

## Operationalization

A good test is:

> Can I create a substantially new coherent instance by editing semantic/compositional properties without touching raw geometry?

If yes, the modeling system is genuinely declarative.

## Provenance

The recovered v0.1 lexicon defined it as:

> “Describe what an object/character is in compositional terms and let procedures generate the concrete geometry/representation.”

This developed alongside constructive puppets, geometric caricature, primitive vocabulary, and procedural visual language.

## Open questions

- What semantic layer is stable across art styles?
- How should conflicting properties resolve?
- Can semantic parameters be learned from examples while keeping the generator inspectable?
- How should live editing expose downstream geometric consequences?

## Revision history

- **2026:** Stabilized as the core authoring method for procedural character/world generation.
