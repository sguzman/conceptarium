---
id: non-telepathic-social-inference
term: Non-telepathic social inference
type: principle
status: canonical
gloss: A simulation rule requiring agents to infer other agents’ beliefs, emotions, intentions, relationships, and traits from observable behavior, communication, history, context, and records rather than directly reading hidden internal state.
domains:
  - simulation
  - game-ai
  - social-ai
  - epistemology
aliases:
  - no-telepathy rule
origin:
  date: 2026
  authorship: joint
  certainty: reconstructed
relations:
  - type: required-by
    target: convergent-architecture
  - type: depends-on
    target: expressive-animation
  - type: supports
    target: generative-village
  - type: analogous-to
    target: epistemic-chain-of-custody
---

# Non-telepathic social inference

## Definition

**Non-telepathic social inference** is the rule that simulated agents should not directly read another agent's private internal state merely because the program has access to it.

If agent A believes agent B is angry, there should be some route by which A could have learned that.

Possible evidence includes:

- facial expression;
- posture;
- voice;
- words;
- previous interactions;
- witnessed actions;
- third-party testimony;
- social reputation;
- environmental traces;
- public records.

The simulation should distinguish:

```text
B is angry
```

from:

```text
A believes B is angry
```

Those are different facts.

## Problem pressure

The concept emerged from a common simulation shortcut.

Designers create rich internal variables:

```text
fear = .82
trust(A) = -.4
goal = steal_food
```

Then other agents quietly query those variables in order to react “intelligently.”

The result is social theater without epistemology.

Everyone knows what the designer knows.

This destroys:

- misunderstanding;
- deception;
- reputation;
- inference;
- ambiguity;
- signaling;
- discovery;
- social learning.

The no-telepathy rule restores them.

## Core model

```text
B internal state
      ↓
B decision
      ↓
B visible behavior / communication
      ↓
A perception
      ↓
A inference
      ↓
A belief about B
```

Every social model is therefore **agent-relative**.

## Misunderstanding as a feature

A good social simulation should allow:

- false beliefs;
- ambiguous signals;
- mistaken attribution;
- deception;
- reputation lag;
- private motives;
- accidental revelation.

These are not bugs.

They are consequences of agents possessing different information.

## Relationship to expressive animation

Visible behavior becomes epistemically important.

If posture, gaze, speed, hesitation, distance, and gesture carry no usable evidence, other agents cannot infer much without telepathy.

Thus **expressive animation** is not merely aesthetic polish.

It is part of the social information channel.

## Relationship to epistemic chain of custody

There is a deep analogy with Conceptarium's epistemology cluster.

For a social belief:

```text
hidden state
  ↓
behavior
  ↓
perception
  ↓
inference
  ↓
belief
```

That is a miniature epistemic chain of custody.

The simulation becomes more coherent when every belief has a plausible informational ancestry.

## Institutional knowledge

Non-telepathic does not mean agents must personally witness everything.

Institutions can create legitimate information channels:

- announcements;
- ledgers;
- gossip;
- newspapers;
- records;
- messengers;
- shared norms.

The key is that knowledge still has a route.

## Non-examples

The principle does not forbid:

- omniscient debugging tools;
- designer inspection;
- supernatural telepathy in a world where telepathy is explicitly part of the fiction;
- shared data structures internally.

It forbids **unmodeled epistemic access by agents**.

## Failure modes

### Relationship-variable leakage

A character directly reads “Alice trusts Bob 0.31.”

### Emotion leakage

A character knows another is afraid despite no observable cue.

### Goal leakage

NPCs counter a player's plan they could not know.

### Global-reputation magic

All agents instantly update reputation after an unwitnessed event.

## Operationalization

For every agent belief about another agent, ask:

> What evidence caused this belief?

If the only answer is “the engine already knew,” the design is telepathic.

## Provenance

The recovered v0.1 definition was:

> “Agents infer others from observable behavior, history, context, and communication rather than reading hidden internal state directly.”

It crystallized as part of the generative-village architecture.

## Open questions

- How should gossip propagate uncertainty?
- How much trait inference should agents perform from sparse evidence?
- Should agents reason explicitly about deception?
- How should cultural norms affect interpretation of the same signal?

## Revision history

- **2026:** Stabilized as the epistemic rule for social simulation.
- **2026-09:** Connected explicitly to epistemic chain-of-custody thinking.
