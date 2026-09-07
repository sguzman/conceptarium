---
id: fiscal-allocation-graph
term: "Fiscal allocation graph"
type: method
status: provisional
gloss: >-
  A legally authoritative graph that propagates contributor attribution through tax events, treasury pools, appropriations, programs, contracts, vendors, and final expenditures.
domains:
  - "public finance"
  - "accounting"
  - "data systems"
  - "institutional design"
aliases:
  - "allocation graph"
origin:
  date: 2026-08-22
  authorship: joint
  certainty: reconstructed
  note: >-
    The source explicitly proposed a legally authoritative allocation graph; the prefixed term was stabilized during recovery.
relations:
  - type: operationalizes
    target: fiscal-provenance
  - type: enables
    target: continuing-fiscal-claim
  - type: supports
    target: fiscal-standing
---

# Fiscal allocation graph

## Definition

A **Fiscal allocation graph** is the computational and legal representation that makes fiscal provenance executable.

Nodes can include taxpayers, tax events, revenue pools, appropriations, agencies, programs, contracts, vendors, purchases, and final consumption. Edges encode authorized transformations and fractional attribution.

The graph tracks **value lineage**, not the physical travel of currency tokens.

## Problem pressure

The theory needed an answer to “how could you know where my dollar went once everything is pooled?”

The answer was to stop imagining a dollar bill traveling intact and model the treasury as a graph of allocations. If a contributor supplies a known fraction of a pool, formally specified rules can propagate that fraction downstream.

## Core model

~~~text
taxpayer
  ↓ contributes
tax event
  ↓ enters
treasury pool
  ↓ allocated by
appropriation
  ↓ funds
program
  ↓ purchases through
contract
  ↓ paid to
vendor
  ↓ supplies
final good/service
~~~

## Claims and implications

The method is technically plausible in principle, but large-scale implementation details remain open.

The graph does not settle which attribution convention is morally correct where funds are pooled, borrowed, or refinanced. It makes those conventions explicit and auditable.

## Examples

### Clean example

A $100 tax event receives a stable identifier. Twenty percent of its attributable value flows into a health appropriation, then fractionally into an MRI contract.

### Borderline example

A graph records only agency-level allocations. It provides partial provenance but cannot answer final-recipient questions.

### Cross-domain example

Supply-chain provenance systems propagate identity and transformation information through multi-stage networks.

## Non-examples

A Sankey diagram built from national averages is visualization, not necessarily an authoritative allocation graph. A blockchain is not required.

## Boundaries and failure modes

Graph precision can exceed economic reality. Borrowing, time, fungibility, and policy substitution create attribution ambiguity. The system must distinguish chosen accounting rules from stronger causal claims.

## Operationalization / evidence

Required features include immutable tax-event IDs, typed allocation edges, exact fractional arithmetic, versioned rules, audit logs, reverse traversal, privacy controls, and reconciliation with official accounts.

A core validation test is conservation: attributed value should not be silently created or lost across transformations except at explicitly modeled events.

## Relations

The graph operationalizes **Fiscal provenance**, enables a **Continuing fiscal claim**, and supports **Fiscal standing** by producing an evidentiary path from contribution to disputed expenditure.

## Provenance

### First known appearance

2026-08-22, in the source proposal for a legally authoritative **allocation graph**.

### Immediate context

The chain was taxpayer → tax event → treasury pool → appropriation → agency → program → contract → vendor → expenditure.

### Problem being solved

To turn fiscal provenance from moral intuition into implementable infrastructure.

## Open questions

- What accounting convention should propagate provenance through deficit-financed expenditures?
- Should attribution follow cash flow, budget authority, economic incidence, or a hybrid?
- What guarantees are sufficient for independent verification?

## Revision history

- 2026-09-06 — Promoted as the implementation method for fiscal provenance.
