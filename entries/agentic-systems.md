---
id: agentic-systems
term: Agentic systems
type: concept
status: canonical
gloss: Systems organized around entities that maintain persistent state, beliefs, needs, goals, memories, and decision processes and can therefore select behavior rather than only execute scene-local scripts.
domains:
  - simulation
  - ai
  - game-ai
aliases:
  - autonomous agent systems
origin:
  date: 2026
  authorship: pre-existing
  certainty: reconstructed
relations:
  - type: used-by
    target: generative-village
  - type: constrained-by
    target: convergent-architecture
  - type: acts-through
    target: motor-intent-boundary
  - type: communicates-through
    target: expressive-animation
---

# Agentic systems

## Definition

**Agentic systems** are systems whose central entities possess persistent internal state and choose actions in relation to that state.

An agent may maintain:

- beliefs;
- memories;
- needs;
- goals;
- relationships;
- emotions;
- plans;
- uncertainty.

The defining distinction is from a scene-local script.

A scripted character reacts because a trigger fired.

An agentic character reacts because its current state plus perception produced a decision.

## Problem pressure

The concept entered the generative-village project because procedural environments alone do not create a living world.

Characters must be able to carry history forward.

The world needs entities for whom previous events matter.

## Minimal loop

```text
perceive
  ↓
update internal state
  ↓
select goal/action
  ↓
act
  ↓
observe consequences
  ↓
update
```

Persistence across cycles is crucial.

## Agency is graded

An agent does not need human-level planning.

A simple animal with persistent hunger, fear, territory, and memory can be meaningfully agentic.

The concept therefore supports a spectrum rather than a binary.

## Relation to convergence

Agentic systems become believable only when internal state actually reaches the world through action and the world pushes back through perception.

Otherwise “agency” becomes hidden bookkeeping.

## Non-examples

Agentic systems are not:

- any state machine;
- any animated NPC;
- random behavior;
- procedural movement without goals;
- an LLM call attached to a character.

## Failure modes

### Decorative state

The system stores needs that barely affect behavior.

### Instant omniscience

Agents know world state they never perceived.

### Goal teleportation

Plans directly set outcomes rather than intents.

### Memory without consequence

History accumulates but never changes future decisions.

## Provenance

The recovered v0.1 lexicon preserved “agentic systems” as systems organized around decision-making entities with goals, beliefs, needs, and state.

## Open questions

- Which internal variables should be explicit?
- How should agents forget?
- How much planning is necessary before reactive systems become sufficiently agentic?
- How should social institutions constrain individual goals?

## Revision history

- **2026:** Incorporated into the generative-village architecture.
