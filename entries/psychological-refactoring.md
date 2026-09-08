---
id: psychological-refactoring
term: Psychological refactoring
type: mechanism
status: canonical
gloss: >-
  Revising legacy psychological rules while preserving valid information, protective
  functions, or hard-won constraints that the older rule was originally built to carry.
domains:
  - psychology
  - self-governance
  - self-knowledge
aliases: []
origin:
  date: 2026-09-07
  authorship: user
  certainty: reconstructed
  note: >-
    Preserves the software metaphor for changing internal implementation without
    discarding valid behavioral or epistemic requirements.
relations:
  - type: changes
    target: legacy-psychological-code
  - type: counteracts
    target: stale-proposition
  - type: enabled-by
    target: epistemic-review
  - type: motivated-by
    target: psychological-archaeology
---

# Psychological refactoring

## Definition

**Psychological refactoring** changes how an old internal rule is implemented while trying to preserve the valid information or protective function it carried.

~~~text
legacy rule
    ↓
identify original purpose
    ↓
separate signal from obsolete implementation
    ↓
rewrite rule for present conditions
~~~

## Why refactor instead of delete?

An old rule may contain genuine information.

Example:

~~~text
OLD:
"Never depend on anyone."

POSSIBLE VALID SIGNAL:
"Dependency without exit can create domination."

REFACTORED:
"Accept interdependence when exit, reciprocity,
and retained agency are preserved."
~~~

Deletion would lose the signal.

Refactoring preserves it while narrowing the rule.

## Relation to epistemic review

[Epistemic review](./epistemic-review.md) determines whether the rule still deserves authority.

Refactoring is one possible action after review.

## Relation to legacy code

[Legacy psychological code](./legacy-psychological-code.md) provides the object being changed.

## Refactoring operations

- narrow scope;
- add exceptions;
- update thresholds;
- distinguish old and current environments;
- separate warning from veto;
- preserve evidence while changing interpretation;
- replace absolute prohibition with conditional rule.

## Failure modes

### Premature deletion

Useful protective information is discarded.

### Cosmetic rewrite

Language changes while behavior remains identical.

### Overengineering

The person builds an elaborate theory instead of changing a simple rule.

### False source confidence

Refactoring depends too heavily on a speculative origin story.

## Provenance

Registered September 7, 2026 as the repair operation in the legacy-psychological-code model.

## Open questions

- Which rules are better retired than refactored?
- How can behavioral evidence confirm that refactoring actually occurred?
- What happens when multiple legacy rules depend on one another?

## Revision history

- **2026-09-08:** First materialization.
