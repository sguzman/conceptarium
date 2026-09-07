---
id: fiscal-provenance
term: "Fiscal provenance"
type: concept
status: provisional
gloss: >-
  The persistent attribution path linking a compulsory contribution to the treasury transformations and expenditures that consume its value, without requiring physical segregation of particular currency units.
domains:
  - "public finance"
  - "political theory"
  - "institutional design"
  - "accounting"
aliases:
  - "tax provenance"
origin:
  date: 2026-08-22
  authorship: joint
  certainty: reconstructed
  note: >-
    The originating discussion explicitly named provenance as a taxpayer right and proposed a taxpayer → tax event → treasury → appropriation → program → contract → vendor → expenditure graph. The prefixed term Fiscal provenance was stabilized during recovery on 2026-09-06.
relations:
  - type: part-of
    target: residual-fiscal-rights
  - type: implemented-by
    target: fiscal-allocation-graph
  - type: enables
    target: fiscal-standing
  - type: supports
    target: taxpayer-sovereignty
---

# Fiscal provenance

## Definition

**Fiscal provenance** is the maintained chain of attribution by which a contributor can trace the value of a compulsory payment through pooling, appropriation, programmatic allocation, contracting, and final consumption.

It does not claim that a particular physical dollar bill remains identifiable. The object preserved is a legally and computationally meaningful attribution relation.

~~~text
physical fungibility
        ≠
loss of accounting identity
~~~

A treasury may pool funds while maintaining an authoritative ledger of which contributors financed which downstream expenditures, fractionally where necessary.

## Problem pressure

The concept arose from the asymmetry between how precisely a state records what an individual owes and how quickly that individual disappears from the accounting model after collection. The question was not merely whether spending is transparent in aggregate. It was: **why should compulsory collection sever the relation between payer, payment, and expenditure?**

Ordinary accounting already individuates claims over fungible pools. The missing vocabulary was therefore not “tracking cash” but preserving provenance of value across treasury transformations.

## Core model

~~~text
taxpayer
  ↓
tax event
  ↓
treasury pool
  ↓
appropriation
  ↓
agency / program
  ↓
contract / purchase
  ↓
final recipient / consumption
~~~

Fiscal provenance preserves an attribution edge from the taxpayer through each transformation. If a contribution represents a fraction of a pooled appropriation, that fraction can be propagated downstream according to explicit rules.

## Claims and implications

The technical claim is that pooled money can remain legally attributable even when physical units are fungible. The stronger normative claim is that compulsory transfer creates a reason to preserve that attribution rather than erase it.

Provenance does not by itself establish taxpayer control over every expenditure. It can exist with weak, moderate, or strong allocation rights and is therefore logically prior to the strongest form of taxpayer sovereignty.

## Examples

### Clean example

A treasury records that a taxpayer supplied a fractional share of a hospital appropriation and propagates that attribution through procurement to an MRI purchase.

### Borderline example

A government issues an annual individualized “tax receipt” based on national averages. This provides information but not full provenance because it does not preserve the actual downstream contribution path.

### Cross-domain example

A mutual fund pools assets while preserving each investor’s legal share.

## Non-examples

A dashboard showing only total government spending is not fiscal provenance. Physically earmarking one banknote for one purchase is also unnecessary; provenance concerns authoritative claims and transformations, not serial-number tracking.

## Boundaries and failure modes

Provenance can become false precision if accounting rules imply a unique causal path where only proportional attribution is defensible. It can also become surveillance infrastructure if contributor identities are unnecessarily public.

Traceability does not settle the legitimacy of taxation, appropriation, or taxpayer veto rights. It only prevents fungibility from being used as an excuse for conceptual disappearance.

## Operationalization / evidence

A real implementation needs stable identifiers for tax events, treasury pools, appropriations, programs, contracts, and expenditures; explicit allocation rules; reproducible fractional propagation; versioned transformations; and independent audit.

A useful test is whether an auditor can start from either a taxpayer contribution or a final expenditure and reconstruct the permitted attribution path.

## Relations

Fiscal provenance is part of **Residual fiscal rights**, implemented by a **Fiscal allocation graph**, enables **Fiscal standing**, and supports **Taxpayer sovereignty** by making contributor-specific claims legible.

## Provenance

### First known appearance

2026-08-22, in the tax-provenance discussion.

### Immediate context

The discussion rejected the idea that pooling money into a general fund makes individualized accounting impossible and proposed “physical pooling + legal individuation.”

### Problem being solved

The taxpayer was visible as a debtor before collection but disappeared as an accounting subject afterward.

### Conceptual ancestors

- accounting claims over fungible pools;
- provenance as a general traceability concept;
- the source rights sequence of provenance, attribution, allocation, and refusal.

### External antecedents

The originating discussion pointed toward tax-choice and public-finance traditions associated with Wicksell, Lindahl, Buchanan, and fiscal-illusion theory. Equivalence has not yet been researched in Conceptarium.

## Open questions

- What attribution rule is most defensible when tax revenues and borrowing are mixed?
- How should provenance treat intertemporal financing, debt service, and money creation?
- Which portions of a provenance ledger should be public, private, or cryptographically auditable?

## Revision history

- 2026-09-06 — Promoted from the fiscal-provenance queue; reconstructed the full ledger model from the 2026-08-22 discussion.
