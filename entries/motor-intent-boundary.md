---
id: motor-intent-boundary
term: Motor-intent boundary
type: concept
status: canonical
gloss: The interface between an agent’s high-level decision about what it intends to do and the animation, locomotion, inverse-kinematics, navigation, or physical machinery that determines how the body actually performs it.
domains:
  - game-ai
  - animation
  - simulation
  - architecture
aliases:
  - intent-to-motion boundary
origin:
  date: 2026
  authorship: joint
  certainty: reconstructed
relations:
  - type: part-of
    target: convergent-architecture
  - type: used-by
    target: agentic-systems
  - type: realized-by
    target: expressive-animation
---

# Motor-intent boundary

## Definition

The **motor-intent boundary** is the interface between cognition and embodiment in an agent simulation.

Above the boundary, the agent expresses **what it intends to accomplish**.

Below the boundary, movement and animation systems determine **how the body accomplishes it**.

Examples of motor intent:

- walk to the door;
- face Maria;
- reach for the cup;
- flee from the dog;
- sit beside the fire;
- hand the tool to another agent;
- maintain distance from a threat.

These are not animation clips.

They are embodied goals.

## Problem pressure

The concept emerged from the need to prevent high-level AI from becoming entangled with the rendering implementation.

If the cognition system says:

> play RUN_AFRAID_03

then several conceptual layers have collapsed:

- fear;
- decision;
- locomotion;
- body mechanics;
- expression.

The agent is no longer deciding what to do.

It is selecting a performance.

The motor-intent boundary preserves the missing layer.

## Core model

```text
belief / need / emotion
        ↓
decision
        ↓
MOTOR INTENT
================ boundary
        ↓
navigation / locomotion
        ↓
pose / IK / animation
        ↓
physical action
        ↓
observable behavior
```

The lower system can fail or modify the intent.

That matters.

An agent can intend to open a door and discover it is locked.

The world then pushes back against cognition.

## Why this boundary matters

### Embodiment

Intent becomes constrained by the body and world rather than instantly realized.

### Reusability

The same high-level intent can be realized by different bodies.

### Expressiveness

Emotion can modulate motion without replacing the underlying intent.

### Failure

Attempts can fail visibly and feed information back into the agent.

### Architecture

Cognition does not need to know animation implementation details.

## Intent parameters

A motor intent may contain semantics such as:

- target;
- urgency;
- desired speed;
- acceptable distance;
- posture preference;
- emotional modulation;
- interruption policy;
- social constraints.

For example:

```text
intent: approach(person)
urgency: low
desired_distance: conversational
affect: nervous
```

The animation system can then produce nervous approach behavior without the planner specifying frames.

## Expression without collapse

Emotion should affect embodiment.

Fear can change:

- gait;
- acceleration;
- gaze;
- distance;
- posture;
- hand motion.

But fear should not bypass decision-making unless the architecture intentionally models reflex.

That distinction lets the system represent both deliberate and involuntary behavior.

## Non-examples

The motor-intent boundary is not:

- a specific API;
- an animation state machine;
- a movement controller;
- a physics engine.

It is the semantic contract between cognitive and motor layers.

## Failure modes

### Clip-thinking

High-level cognition selects animations directly.

### Omnipotent intent

Every intent succeeds immediately, eliminating embodiment.

### Motor backdoor

Animation code changes social or cognitive state directly without generating observable evidence.

### Semantic poverty

The boundary contains only “move to coordinate,” making meaningful embodied actions impossible to express.

## Operationalization

A good test is:

> Could two agents with different bodies execute the same intent differently while preserving the intention?

If yes, the boundary is probably doing useful work.

## Provenance

The v0.1 lexicon preserved the term as:

> “Interface between an agent’s high-level decision/intention and the animation/physical machinery that realizes visible action.”

It belongs to the larger **convergent architecture** project.

## Open questions

- What is the minimal intent vocabulary?
- How should compound actions be decomposed?
- Where do reflexes live?
- Should dialogue gestures be motor intent or expressive modulation?
- How should failed motor actions update beliefs?

## Revision history

- **2026:** Formulated as the cognition/embodiment interface in generative agents.
