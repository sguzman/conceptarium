# Systems and Generativity

This cluster joins three layers of the generative-world project:

1. **declarative construction** — what things are;
2. **causal simulation** — how internal state becomes world behavior;
3. **social epistemology** — how agents learn about one another without hidden-state leakage.

The concrete world-scale target is the [Generative village](../entries/generative-village.md).

## Master architecture

~~~text
GENERATIVE MODELING
      ↓
Declarative procedural modeling
      ↓
Declarative character
      ↓
body / clothing / posture / affordances
      ↓
"carry their personality in their bodies"

AUTONOMOUS AGENT
      ↓
perception
      ↓
belief / memory
      ↓
need + emotional systems
      ↓
decision
      ↓
Motor-intent boundary
      ↓
Expressive animation / physical action
      ↓
visible evidence
      ↓
Non-telepathic social inference
      ↓
other agents update beliefs
~~~

The project is strongest when these are not parallel feature lists.

They are one causal system.

## Declarative construction branch

The broad method is [generative modeling](../entries/generative-modeling.md):

> author reusable rules, parameters, primitives, and constraints that generate coherent families of artifacts rather than building every finished instance manually.

The more specific authoring architecture is [Declarative procedural modeling](../entries/declarative-procedural-modeling.md):

~~~text
semantic description
      ↓
compositional rules
      ↓
procedural operations
      ↓
concrete geometry
~~~

Supporting entries:

- [Primitive vocabulary](../entries/primitive-vocabulary.md)
- [Procedural visual language](../entries/procedural-visual-language.md)
- [Constructive puppet](../entries/constructive-puppet.md)
- [Geometric caricature](../entries/geometric-caricature.md)
- [declarative character](../entries/declarative-character.md)

The hierarchy is deliberate:

~~~text
Generative modeling
      ↓ general umbrella
Declarative procedural modeling
      ↓ semantic specialization
Declarative character
      ↓ character-specific representation
concrete generated instance
~~~

## Declarative character

A [declarative character](../entries/declarative-character.md) is specified through layered semantic properties such as:

~~~text
anatomy
+ phenotype
+ age
+ profession
+ personality
+ clothing
+ current emotion
~~~

The source of truth is therefore not primarily the final mesh.

It is the semantic composition that can regenerate or modify the body.

That lets one property influence multiple downstream systems coherently.

## Carry their personality in their bodies

[carry their personality in their bodies](../entries/carry-their-personality-in-their-bodies.md) is preserved as an amber design objective.

The phrase connects static construction and dynamic embodiment:

~~~text
history / profession / temperament / age / emotion
        ↓
geometry + clothing + posture + movement
        ↓
visible evidence of character
~~~

The objective is not perfect psychological transparency.

A body should carry evidence of the character, not expose hidden state telepathically.

That means the same architecture must also permit:

- masking;
- ambiguity;
- contradiction;
- culture-specific expression;
- traits that remain invisible.

## Causal simulation branch

The master architectural principle remains [Convergent architecture](../entries/convergent-architecture.md).

Its entity-level unit is now explicitly [autonomous agents](../entries/autonomous-agents.md).

The distinction is:

~~~text
Agentic systems
      =
the architecture for persistent state and decision

Autonomous agents
      =
the concrete persistent entities operating inside it
~~~

A minimal autonomous loop is:

~~~text
persistent internal state
      +
perception
      ↓
decision
      ↓
motor intent
      ↓
embodied action
      ↓
world consequence
      ↓
state update
~~~

Supporting entries:

- [Agentic systems](../entries/agentic-systems.md)
- [autonomous agents](../entries/autonomous-agents.md)
- [Motor-intent boundary](../entries/motor-intent-boundary.md)
- [Expressive animation](../entries/expressive-animation.md)

## Emotional systems

[emotional systems](../entries/emotional-systems.md) closes one of the major causal gaps in the old architecture.

Emotion is not a cosmetic terminal tag:

~~~text
fear = play_fear_animation
~~~

The intended chain is:

~~~text
event / perception
      ↓
appraisal
      ↓
emotional state
      ↓
decision weighting
      ↓
motor / expressive modulation
      ↓
visible behavior
~~~

This makes emotion part of agency rather than presentation.

It also preserves personality-specific response: the same event can produce different emotional state because appraisal depends on memory, need, goals, temperament, and context.

## Social epistemology branch

The governing rule remains [Non-telepathic social inference](../entries/non-telepathic-social-inference.md).

~~~text
another agent's hidden state
        ↓
their decision
        ↓
visible behavior / communication
        ↓
my perception
        ↓
my inference
        ↓
my belief about them
~~~

This is deliberately analogous to an [epistemic chain of custody](../entries/epistemic-chain-of-custody.md): simulated social knowledge should have informational ancestry.

The new entries sharpen this further.

**Emotional systems** produce internal causal state.

**Expressive animation** exposes partial evidence of that state.

**Carry their personality in their bodies** extends that evidence across longer timescales such as profession, habit, age, and character history.

None of those gives another agent automatic access to the truth.

## World-scale target

The [Generative village](../entries/generative-village.md) is where the construction and simulation branches meet.

~~~text
generative modeling
      ↓
declarative people / places / objects
      +
autonomous agents
      ↓
persistent need / emotion / memory / decision
      +
visible embodied behavior
      ↓
social inference
      ↓
accumulated local history
~~~

The village scale remains attractive because it is large enough for households, work, reputation, trade, conflict, and repeated relationships while remaining small enough for causal history to stay inspectable.

## Core anti-collapse rules

- generative modeling ≠ random generation;
- declarative character ≠ labels attached to a finished mesh;
- autonomous agent ≠ scripted NPC with state variables;
- emotional system ≠ emote selector;
- expressive behavior ≠ perfect mind-reading;
- visible personality ≠ deterministic stereotype;
- system complexity ≠ causal convergence.

## Ethical boundary

- [Sovereign system builders](../entries/sovereign-system-builders.md)
- [Anti-domination](../entries/anti-domination.md)
- [Contestability](../entries/contestability.md)

A system can model agents deeply without acquiring moral permission to treat real people as model components.

The generative project and the ethical project therefore still meet at a common rule:

> **Modelability is not ownership.**
