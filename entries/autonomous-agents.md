---
id: autonomous-agents
term: "autonomous agents"
type: concept
status: provisional
gloss: >-
  Persistent simulated entities that maintain internal state across time and select behavior from their own needs, beliefs, goals, memories, emotions, and perceptions rather than existing only as scene-local scripted reactions.
domains:
  - "simulation"
  - "game-ai"
  - "generative-systems"
  - "agents"
aliases:
  - "autonomous simulated agents"
origin:
  date: 2026-08
  authorship: joint
  certainty: reconstructed
  note: >-
    Recovered from the Atmos roadmap as the concrete entity-level target beneath the broader Agentic systems architecture.
relations:
  - type: specializes
    target: agentic-systems
  - type: constrained-by
    target: convergent-architecture
  - type: acts-through
    target: motor-intent-boundary
  - type: shaped-by
    target: emotional-systems
  - type: used-by
    target: generative-village
---

# autonomous agents

## Definition

**Autonomous agents** are simulated entities whose behavior is selected from persistent internal state rather than authored as a sequence of scene-specific reactions.

An autonomous agent can carry forward:

- needs;
- beliefs;
- memories;
- goals;
- emotional state;
- relationships;
- uncertainty;
- previous consequences.

The entity therefore has a history that matters to its next action.

## Problem pressure

The Atmos project needed a concrete target smaller than “build a living village.”

A procedural world is not socially generative if its characters only wait for triggers. The first executable promise became a single creature whose need changes, who perceives something relevant, chooses an action, acts through a body, and updates after the consequence.

That made **autonomous agent** the entity-level unit of the larger Agentic systems architecture.

## Core model

~~~text
persistent internal state
        +
perception of current world
        ↓
action selection
        ↓
motor intent
        ↓
embodied action
        ↓
world consequence
        ↓
state update
        ↓
next decision
~~~

## Claims and implications

Autonomy is graded rather than binary.

A hungry animal that remembers food locations and changes behavior after failed attempts can be meaningfully autonomous without possessing language or long-horizon planning.

The defining requirement is that the agent's current behavior is generated from its own persistent causal state rather than merely from a designer-authored scene script.

## Examples

### Hungry walker

Hunger rises, food is perceived, one food target is selected, the agent approaches and eats, and hunger falls.

### Choice under pressure

An agent chooses between nearby low-value food and distant high-value food based on hunger, memory, risk, and effort.

### Social agent

A character alters approach behavior because previous interactions changed trust or fear.

## Non-examples

An NPC that plays a canned animation when the player crosses a trigger is not autonomous merely because it has state variables.

Random wandering is not autonomy.

An LLM attached to a character is not sufficient if the surrounding world and action system do not preserve persistent causal state.

## Boundaries and failure modes

The term should not imply philosophical free will.

It names architectural autonomy inside a simulation.

The strongest failure mode is **decorative autonomy**: the system stores rich state while actual behavior remains mostly scripted or privileged designer logic bypasses the agent's decision process.

## Operationalization / evidence

A practical test is:

> If the same scene begins with different persistent agent state, does behavior change for reasons inspectable through perception, internal state, decision, and motor intent?

Useful demonstrations should expose the full causal trace rather than merely show varied animation.

## Relations

Autonomous agents specialize **Agentic systems**, are constrained by **Convergent architecture**, act through the **Motor-intent boundary**, are shaped by **Emotional systems**, and populate the world-scale ambitions represented by the **Generative village**.

## Provenance

### First known appearance

August 2026 in the Atmos roadmap and integration planning.

### Immediate context

The single-agent roadmap explicitly prioritized an explainable hungry-agent loop before multi-agent village simulation.

### Problem being solved

To create the smallest complete unit of simulated life that can later scale into households, social inference, and village history.

### Conceptual ancestors

- [Agentic systems](./agentic-systems.md)
- [Convergent architecture](./convergent-architecture.md)
- [Motor-intent boundary](./motor-intent-boundary.md)

### External antecedents

Autonomous-agent architectures are a broad pre-existing field in AI, simulation, robotics, and game AI. Conceptarium's distinctive use is tied to causal convergence and visible embodied evidence.

## Open questions

- What minimum persistent state is enough for convincing autonomy?
- When should planning replace reactive selection?
- How should agent goals be inspected without making the world telepathic?

## Revision history

- 2026-09-07 — Promoted from the Generative systems queue using the recovered August 2026 Atmos architecture and Conceptarium master lexicon.
