---
id: primitive-vocabulary
term: Primitive vocabulary
type: concept
status: canonical
gloss: A deliberately small reusable set of geometric solids, operations, curves, sweeps, deformations, and compositional rules from which a much larger family of forms can be constructed.
domains:
  - graphics
  - procedural-generation
  - design
aliases:
  - geometric vocabulary
origin:
  date: 2026
  authorship: joint
  certainty: reconstructed
relations:
  - type: used-by
    target: declarative-procedural-modeling
  - type: generates
    target: procedural-visual-language
  - type: enables
    target: geometric-caricature
  - type: enables
    target: constructive-puppet
---

# Primitive vocabulary

## Definition

A **primitive vocabulary** is the compact set of constructive elements from which a procedural visual system builds more complex forms.

Possible primitives include:

- spheres and ellipsoids;
- superellipsoids;
- tapered cylinders;
- boxes;
- planes;
- spline sweeps;
- profile extrusions;
- SDF unions and subtractions;
- bends;
- tapers;
- twists;
- offsets;
- repeated arrays.

The key idea is linguistic:

> A small alphabet can generate a large language.

The goal is not to possess a primitive for every object.

The goal is to possess primitives that compose well.

## Problem pressure

The concept emerged from a preference for **obvious constructive geometry** over seamless sculptural imitation.

If a character is assembled from clearly legible geometric decisions, then variation becomes easier to reason about.

You can ask:

- which primitive creates the torso?
- which deformation creates age?
- what controls the nose silhouette?
- how does posture alter the assembly?

The form remains intellectually inspectable.

## Vocabulary versus asset library

An asset library says:

> Here are 300 finished noses.

A primitive vocabulary says:

> Here are a few operations from which many noses can be constructed.

The second creates a compositional space rather than a catalog.

## Expressive power

A good primitive vocabulary balances:

- compactness;
- composability;
- parameter continuity;
- stylistic coherence;
- semantic usefulness.

Too small and every result looks the same.

Too large and the system becomes a disguised asset library.

## Relation to geometric caricature

Deliberately visible primitives can become the aesthetic.

A forearm does not need to hide that it is fundamentally a tapered sweep.

A face can be an assembly of exaggerated constructive volumes.

That turns implementation constraints into visual language.

## Non-examples

Primitive vocabulary is not:

- a random list of Blender primitives;
- every geometry function in a library;
- a kitbash asset folder;
- low-poly modeling in general.

The vocabulary is intentionally curated for **generative composition**.

## Failure modes

### Special-case growth

Every new design requires a new primitive.

### Non-composability

Primitives work alone but intersect badly.

### Parameter dead zones

Most parameter combinations produce unusable forms.

### Stylistic dilution

Primitives are so general that generated objects lose recognizable visual grammar.

## Operationalization

Evaluate a vocabulary by asking:

- How many distinct coherent forms can it generate?
- How many primitives are actually necessary?
- Can semantics map cleanly onto parameters?
- Do combinations remain visually interpretable?
- Can the system be extended without invalidating old compositions?

## Provenance

The recovered lexicon preserved:

> “Small reusable set of geometric operations/solids from which a much larger visual language can be composed.”

It developed from experiments in visibly geometric, procedural character construction.

## Open questions

- What is the minimal useful human-character vocabulary?
- How should primitive vocabularies differ across architectural, anatomical, and environmental domains?
- Can the vocabulary itself be versioned like a programming language?

## Revision history

- **2026:** Stabilized as the compositional alphabet beneath procedural visual language.
