---
id: emotional-systems
term: "emotional systems"
type: framework
status: provisional
gloss: >-
  Simulation architecture in which emotion is a persistent, causally active state produced by appraisal and personality and capable of changing decisions, motor behavior, expression, memory, and social inference rather than acting as a cosmetic animation tag.
domains:
  - "simulation"
  - "game-ai"
  - "emotion"
  - "generative-systems"
aliases:
  - "emotional system"
origin:
  date: 2026-08
  authorship: joint
  certainty: reconstructed
  note: >-
    Recovered from the Atmos roadmap where emotion was explicitly framed as simulation state that must affect action selection and visible behavior.
relations:
  - type: part-of
    target: convergent-architecture
  - type: shapes
    target: autonomous-agents
  - type: expressed-by
    target: expressive-animation
  - type: supports
    target: non-telepathic-social-inference
  - type: constrains
    target: motor-intent-boundary
---

# emotional systems

## Definition

**Emotional systems** treat emotions as stateful causal mechanisms inside an agent.

An emotional state is not merely:

~~~text
fear = play_fear_animation
~~~

It should participate in a chain such as:

~~~text
event / perception
      ↓
appraisal
      ↓
emotional state
      ↓
decision weighting
      ↓
motor intent / expression
      ↓
visible behavior
      ↓
social inference
~~~

## Problem pressure

Many game systems label characters with emotions but allow those labels to affect only presentation.

The Atmos architecture rejected this as cosmetic.

If fear, anger, shame, affection, confidence, or fatigue are modeled, they should change what the agent notices, chooses, remembers, approaches, avoids, and expresses.

Otherwise the simulation contains emotion vocabulary without emotional causality.

## Core model

~~~text
perceived event
      +
agent history / personality / need
      ↓
appraisal
      ↓
emotional state
      ↓
decision bias + action tendency
      ↓
motor / expressive modulation
      ↓
world-visible evidence
~~~

## Claims and implications

Emotion should mediate rather than replace agency.

The same event can produce different emotional responses because appraisal depends on personality, memory, goals, and context.

The system should also permit mixed, decaying, persistent, and context-specific states rather than one global mood enum.

## Examples

### Fear

A threat increases avoidance, interpersonal distance, gaze behavior, speed, and later memory weighting.

### Anger

Perceived obstruction changes goal priority and movement style without forcing a single scripted attack.

### Affection

Relationship history changes attention, approach, willingness to help, and expressive behavior.

## Non-examples

A facial-expression selector is not an emotional system by itself.

Dialogue tags such as “angry” are not sufficient.

A random mood generator is not emotional causality unless state responds to events and alters later behavior.

## Boundaries and failure modes

Overly elaborate emotion taxonomies can become decorative complexity.

A strong system should earn each emotional variable through observable causal effects.

Another failure mode is perfect readability: expression should provide evidence, not direct public access to hidden emotional state.

## Operationalization / evidence

For every emotional variable, trace:

1. what events or appraisals change it;
2. what decisions it modifies;
3. what motor or expressive consequences it produces;
4. what another agent could legitimately infer from those consequences.

If any link is absent, the emotional state may be decorative.

## Relations

Emotional systems are part of **Convergent architecture**, shape **autonomous agents**, are expressed through **Expressive animation**, support **Non-telepathic social inference**, and ultimately act through the **Motor-intent boundary** rather than teleporting directly into animation clips.

## Provenance

### First known appearance

August 2026 in the Atmos emotional-systems roadmap.

### Immediate context

The roadmap explicitly framed emotion as appraisal-driven state affecting action selection, expression, personality-specific response, and non-telepathic social inference.

### Problem being solved

To make emotion an actual simulation mechanism rather than a cosmetic presentation layer.

### Conceptual ancestors

- [Convergent architecture](./convergent-architecture.md)
- [Expressive animation](./expressive-animation.md)
- [Non-telepathic social inference](./non-telepathic-social-inference.md)

### External antecedents

Computational emotion, appraisal theory, affective computing, and game AI provide broad antecedents. Conceptarium's architectural emphasis is causal convergence and visible evidence.

## Open questions

- Which appraisal dimensions are worth explicitly modeling?
- How should personality mediate appraisal without hard-coded stereotypes?
- What timescales should emotional decay and memory interaction use?

## Revision history

- 2026-09-07 — Promoted from the Generative systems queue using the recovered August 2026 Atmos architecture and Conceptarium master lexicon.
