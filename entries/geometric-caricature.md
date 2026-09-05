---
id: geometric-caricature
term: Geometric caricature
type: concept
status: canonical
gloss: Character or object design that achieves recognizability, personality, and exaggeration through deliberately legible geometric construction rather than by concealing geometry beneath naturalistic sculptural detail.
domains:
  - character-design
  - graphics
  - aesthetics
aliases: []
origin:
  date: 2026
  authorship: joint
  certainty: reconstructed
relations:
  - type: uses
    target: primitive-vocabulary
  - type: expresses
    target: procedural-visual-language
  - type: overlaps-with
    target: constructive-puppet
  - type: generated-by
    target: declarative-procedural-modeling
---

# Geometric caricature

## Definition

**Geometric caricature** is recognizable and expressive design achieved through deliberately obvious geometry.

Instead of hiding construction, the artist exaggerates meaningful features using shapes such as:

- superellipsoids;
- tapered cylinders;
- spline sweeps;
- planes;
- boxes;
- profile extrusions;
- SDF unions;
- curved plates.

The design question is:

> Which geometric decisions carry the identity?

## Problem pressure

The concept emerged while exploring procedural character generation.

Naturalistic anatomy is difficult to generate robustly because it contains continuous, subtle surface information.

Caricature offers another route.

Recognition can survive large simplification if the right structural relationships remain:

- head/body ratio;
- nose projection;
- shoulder slope;
- torso mass;
- limb taper;
- posture;
- face plane orientation.

This suggested that procedural geometry need not approximate realism poorly.

It could pursue **caricatural truth** instead.

## Caricature as information compression

A caricature discards detail while preserving or amplifying discriminative structure.

That makes it especially compatible with generative systems.

A procedural rule can say:

- longer midface;
- forward head posture;
- heavy upper torso;
- narrow shoulders;
- oversized hands.

These changes can create identity without requiring a bespoke sculpt.

## Personality in geometry

The larger ambition is that generated characters **carry their personality in their bodies**.

Not through crude physiognomic determinism, but through expressive design choices:

- posture;
- tension;
- clothing structure;
- age;
- profession;
- habitual movement;
- mood.

Geometry becomes semantic.

## Non-examples

Geometric caricature is not:

- merely low polygon count;
- random exaggeration;
- flat-shaded realism;
- deforming one base mesh without meaningful construction.

The geometry must be **legible as a deliberate expressive vocabulary**.

## Failure modes

### Symbolic cliché

Personality becomes encoded through simplistic stereotypes.

### Recognition collapse

Exaggeration destroys the underlying structural identity.

### Primitive monotony

Every feature is reduced to the same few forms without enough variation.

### Semantic overclaim

A design implies that deep moral traits are visually knowable from anatomy.

The intended use is expressive character design, not biological diagnosis.

## Operationalization

A useful test:

> Can the character remain recognizable and expressive when fine surface detail, textures, and realistic anatomy are removed?

If yes, the geometry is carrying meaningful information.

## Provenance

The recovered lexicon defined it as:

> “Recognizable/exaggerated character design using deliberately obvious geometry: superellipsoids, tapered cylinders, spline sweeps, planes, SDF unions, etc.”

## Open questions

- What geometric variables carry identity most efficiently?
- How should caricature interact with age and phenotype without collapsing into stereotype?
- Can procedural systems learn exaggeration rules while remaining editable?

## Revision history

- **2026:** Stabilized as the expressive target for procedural primitive-based character design.
