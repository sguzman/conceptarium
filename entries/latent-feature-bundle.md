---
id: latent-feature-bundle
term: "latent feature bundle"
type: framework
status: provisional
gloss: >-
  The underlying cluster of affective, sensory, formal, narrative, and thematic properties that may actually drive an appetite or lock beneath a coarser conscious label such as genre.
domains:
  - "aesthetics"
  - "media-consumption"
  - "attention"
  - "representation"
aliases: []
origin:
  date: 2026-09-05
  authorship: assistant
  certainty: reconstructed
  note: >-
    Recovered from the canonical Genre lock section 'Genre label versus latent feature bundle,' which gave a brooding + technological + nocturnal + urban + high-contrast + dark-story + small-hope example.
relations:
  - type: explains
    target: genre-lock
  - type: shapes
    target: media-appetite
  - type: explains
    target: tone-lock
  - type: explains
    target: aesthetic-lock
---

# latent feature bundle

## Definition

A **Latent feature bundle** is the hidden or only partly articulated configuration of properties that actually makes a work fit a current appetite.

A conscious label such as “science fiction” may compress a bundle like:

~~~text
brooding
+ technological
+ nocturnal
+ urban
+ high-contrast
+ dark story
+ small hope signals
~~~

The label is useful, but it may not be the true causal unit of selection.

## Problem pressure

The user’s science-fiction lock did not respond equally to all science fiction. Some nominally non-science-fiction works could plausibly fit the desired atmosphere better than nominal genre members.

The Genre lock entry therefore needed a concept separating **taxonomy** from **the feature distribution the mind is actually selecting for**.

## Core model

~~~text
conscious label
"science fiction"
        ↓ approximates
LATENT FEATURE BUNDLE
        ↓
actual match computation
        ↓
motivational admissibility
~~~

## Claims and implications

The bundle can include features at many levels:

- genre;
- tone;
- color and lighting;
- sound;
- pacing;
- world structure;
- narrative tension;
- social texture;
- emotional promise.

The term does not claim the bundle is neurologically literal or easily recoverable. It is an explanatory model for why labels sometimes predict appetite poorly.

## Examples

### Clean example

A player says they want sci-fi but repeatedly selects games that are rainy, neon-lit, urban, melancholic, and technologically dense while rejecting bright space operas.

### Cross-genre example

A gothic detective game and a cyberpunk game both satisfy the same appetite because the actual bundle is nocturnal urban mystery + oppressive atmosphere + small hope.

### Borderline example

A genre label predicts choices perfectly for a period. The latent bundle may simply overlap tightly with that genre.

## Non-examples

A list of every property of a work is not a latent feature bundle. The bundle should contain features doing explanatory work for selection.

A marketing taxonomy is also not automatically the person’s bundle.

## Boundaries and failure modes

The model can become post hoc if analysts keep adding features until every choice is explained.

A useful bundle should generate predictions about **unseen candidates**, not merely redescribe past selections.

## Operationalization / evidence

Infer candidate features from repeated selections and rejections, then test them against new works that vary independently on genre and tone.

Prediction improves if the inferred bundle explains why some nominal genre members fail while cross-genre works succeed.

## Relations

Latent feature bundle explains variation within **Genre lock**, **Tone lock**, and **Aesthetic lock**, and shapes **Media appetite** by specifying what properties currently carry reward.

## Provenance

### First known appearance

2026-09-05, explicitly as a section heading and model inside the canonical Genre lock entry.

### Immediate context

The source contrasted the conscious genre label “science fiction” with a deeper bundle of brooding, technological, nocturnal, urban, high-contrast, dark-but-hopeful properties.

### Problem being solved

Nominal genre labels were too coarse to explain the specificity of the user’s actual appetite.

### Conceptual ancestors

- [Genre lock](./genre-lock.md)
- [media appetite](./media-appetite.md)

### External antecedents

Potential neighbors include latent-factor models, feature-based choice, embedding spaces, and prototype theory. The Conceptarium term is phenomenological and should not be collapsed into any one computational model.

## Open questions

- How stable are latent feature bundles across appetite episodes?
- Can multiple different bundles produce the same conscious genre label?
- How many features are needed before the model becomes overfit?

## Revision history

- 2026-09-07 — Promoted from the Attention / absorption / completion queue.
