---
id: convergent-architecture
term: Convergent architecture
type: framework
status: canonical
gloss: A simulation architecture in which world state, perception, belief, needs, emotion, decision, motor intent, animation, visible behavior, and social reaction form one causal chain rather than disconnected subsystems that secretly share hidden state.
domains:
  - simulation
  - game-ai
  - generative-systems
  - architecture
aliases:
  - causal convergence
origin:
  date: 2026
  authorship: joint
  certainty: reconstructed
relations:
  - type: contains
    target: motor-intent-boundary
  - type: requires
    target: non-telepathic-social-inference
  - type: supports
    target: generative-village
  - type: complements
    target: declarative-procedural-modeling
---

# Convergent architecture

## Definition

**Convergent architecture** is a design principle for simulated worlds in which the major systems that produce behavior converge into one explicit causal chain.

The canonical sequence is:

```text
world
 ↓
perception
 ↓
belief / memory
 ↓
need / emotion
 ↓
decision
 ↓
motor intent
 ↓
animation / physical action
 ↓
visible evidence
 ↓
other agents' inference
```

The principle rejects architectures where “AI,” “emotion,” “animation,” and “social reaction” are separate theatrical modules that quietly teleport truth between one another.

If an agent is afraid, that fear should matter because:

1. the agent perceived something;
2. the perception altered belief or internal state;
3. fear affected decision;
4. decision produced motor intent;
5. motor intent changed visible behavior;
6. other agents perceived that behavior and inferred something from it.

The point is **causal continuity**.

## Problem pressure

The framework emerged from dissatisfaction with simulations that contain many impressive subsystems but do not actually compose.

A game may claim to model:

- needs;
- emotions;
- memory;
- relationships;
- animation;
- perception.

But if every system can directly read every other system's hidden variables, the simulation becomes narratively telepathic.

The world looks alive because scripts coordinate the performance, not because the world has a coherent causal substrate.

The frustration was:

> Why model internal state at all if visible behavior does not actually carry it into the world?

Convergent architecture was the answer.

## Core principle

Every important internal distinction should eventually have **external consequences**.

Likewise, every social inference should ultimately depend on something observable or communicable.

This creates two directions:

### Inside-out

```text
internal state → action → visible evidence
```

### Outside-in

```text
visible evidence → perception → inference → updated belief
```

Social worlds emerge when these loops interlock among many agents.

## Why convergence matters

### Debuggability

A causal chain gives every surprising behavior a route backward.

Why did the character run?

Because motor intent said flee.

Why did intent say flee?

Because the decision system selected escape.

Why?

Because fear crossed a threshold after a perception updated belief.

That is inspectable.

### Emergence

When systems compose causally, unplanned interactions become possible.

### Consistency

Animation and decision cannot silently disagree without exposing a boundary failure.

### Epistemic integrity

Other agents do not receive privileged access to hidden state.

## Architecture versus monolith

Convergent architecture does **not** mean one giant subsystem.

The implementation may remain modular:

- perception;
- memory;
- emotion;
- planning;
- locomotion;
- animation;
- dialogue.

The requirement is that modules exchange information through **semantically honest interfaces** that preserve causal order.

## Motor-intent boundary

The most important interface is between high-level decision and physical realization.

An agent should decide something like:

- approach X;
- flee Y;
- look at Z;
- give object A to person B;
- sit near the fire.

It should not decide “play animation clip 47.”

That latter form collapses intention and rendering.

The **motor-intent boundary** lets high-level cognition remain independent of how the body physically fulfills the action.

## Non-telepathic social inference

Convergent architecture also requires that agents learn about one another through:

- observation;
- speech;
- history;
- environmental traces;
- institutional records where appropriate.

They should not directly inspect another agent's fear variable because the designer wants social reactions to look intelligent.

## Non-examples

Convergent architecture is not:

- putting every system in one update loop;
- a behavior tree;
- an entity-component system;
- a requirement for perfect physical realism;
- a ban on authored events.

It is a causal architecture principle.

## Failure modes

### Hidden-state leakage

A social system reads another agent's private state directly.

### Animation teleportation

A decision instantly becomes a visual pose with no motor interface.

### Cosmetic emotion

Emotion exists only to select facial expressions and never affects choice.

### Decorative memory

Agents store history but decisions do not depend on it.

### Reverse causality by convenience

The system changes belief merely because a designer needs a later animation to make sense.

## Operationalization

For any visible action, ask:

> Can I trace this backward through motor intent, decision, internal state, and perception?

For any social belief, ask:

> Can I trace it backward to evidence available to that agent?

If not, the architecture contains a hidden causal shortcut.

## Provenance

### Recovered wording

The v0.1 lexicon preserved:

> “World systems, perception, beliefs, needs, emotions, decisions, motor intent, animation, visible behavior, and social reaction form one causal chain rather than disconnected subsystems.”

### Problem being solved

How can a generative village feel like one world rather than several synchronized fake worlds?

## Open questions

- Which internal states should be explicit versus emergent?
- How much causal compression is acceptable for performance?
- How should dialogue function inside the same architecture?
- Can institutional knowledge be modeled without violating non-telepathic inference?
- What debugging visualization best exposes broken causal chains?

## Revision history

- **2026:** Formulated as causal convergence across agent/world systems.
- **2026-09:** Promoted as the master architectural principle for generative-agent simulation.
