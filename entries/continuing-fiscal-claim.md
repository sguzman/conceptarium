---
id: continuing-fiscal-claim
term: "Continuing fiscal claim"
type: concept
status: provisional
gloss: >-
  A contributor-specific claim that remains attached to the value of a compulsory contribution as that value moves through treasury transformations rather than terminating at the revenue node.
domains:
  - "public finance"
  - "political theory"
  - "accounting"
aliases:
  - "continuing claim"
origin:
  date: 2026-08-22
  authorship: joint
  certainty: reconstructed
  note: >-
    The source diagram explicitly labeled a continuing claim from taxpayer to final consumption; the fiscal qualifier was added during recovery.
relations:
  - type: part-of
    target: residual-fiscal-rights
  - type: supported-by
    target: fiscal-non-extinguishment
  - type: enabled-by
    target: fiscal-provenance
---

# Continuing fiscal claim

## Definition

A **Continuing fiscal claim** is the persistence of a contributor’s legally meaningful relation to contributed value after collection.

The claim may be informational, allocative, prohibitory, or enforceable depending on constitutional design. What makes it continuing is that the taxpayer remains an entity in the fiscal relation through downstream transformations.

## Problem pressure

The source contrasted two ontologies.

In the simplified conventional schema:

~~~text
taxpayer → REVENUE → STATE → EXPENDITURE
~~~

the taxpayer disappears after revenue.

In the proposed schema, the taxpayer remains connected by a **continuing claim**.

## Core model

~~~text
taxpayer ─────────────────────┐
   ↓                          │
tax contribution              │ continuing claim
   ↓                          │
treasury → appropriation      │
   ↓                          │
program → purchase            │
   ↓                          │
final consumption ←───────────┘
~~~

## Claims and implications

A continuing claim does not necessarily mean continuing ownership. It can be narrower: a right to know, challenge, audit, direct a bounded share, or object to prohibited uses.

The legal content must therefore be specified independently from persistence.

## Examples

### Clean example

A taxpayer's contribution remains linked to downstream expenditure records and the taxpayer can invoke those records when asserting a statutory allocation right.

### Borderline example

The state preserves attribution internally but gives the taxpayer no access or remedy. The informational relation continues; the legal claim may not.

## Non-examples

A generic civic interest in government spending is not contributor-specific. Neither is an informal expectation that “taxpayers deserve value for money.”

## Boundaries and failure modes

The claim becomes bloated if treated as preserving every incident of ownership after taxation. The useful question is always: **what exactly continues?**

## Operationalization / evidence

Represent the contributor as a durable subject in the fiscal data model and define legal rights attached to the persistent edge. Test whether downstream events can be queried from the contributor side and whether that query has enforceable consequence.

## Relations

The continuing claim is part of **Residual fiscal rights**, supported by **Fiscal non-extinguishment**, and enabled by **Fiscal provenance**.

## Provenance

### First known appearance

2026-08-22, reconstructed from the source diagram explicitly labeling a “continuing claim.”

### Problem being solved

To name the persistent relation that fiscal provenance is meant to carry.

## Open questions

- Can continuing claims be transferred, inherited, or waived?
- At what point is a contribution finally consumed and the claim discharged?
- How should claims interact with pooled intergenerational investment?

## Revision history

- 2026-09-06 — Promoted from the diagrammatic continuing claim into a standalone fiscal concept.
