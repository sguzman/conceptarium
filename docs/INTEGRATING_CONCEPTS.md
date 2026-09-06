# Integrating Concepts into Conceptarium

This document defines the **end-to-end contract** for capturing, promoting, integrating, or revising a concept in Conceptarium.

Conceptarium distinguishes **predicate presence** from **ontological materialization**. A concept may be captured in the registry with almost no semantic commitment and developed later.

It is written for both humans and AI agents.

When someone says:

> **Integrate this concept into Conceptarium.**

the expected result is **not** “create one Markdown file.”

Integration means placing the concept into the repository as a durable part of the intellectual system: preserving its exact insight, reconstructing why it was needed, locating it among neighboring concepts, updating the graph and indexes, and proving that the corpus still validates and projects correctly.

The short version is:

> **Preserve the insight, reconstruct the pressure, formalize the concept, connect it to the corpus, update the surrounding maps, and validate the whole repository.**

---

## 1. What “integrate” means

A rigorous concept integration normally includes all of the following:

1. ensure the concept has predicate presence in `registry/concepts.yml`;
2. determine whether the idea is actually new;
3. preserve the exact wording that carries conceptual value;
4. decide whether to keep it registry-only, create a new entry, or revise an existing one;
5. assign type, status, domains, aliases, and provenance when materializing;
6. write a dictionary-quality gloss when materializing;
7. write an encyclopedic treatment when materializing;
8. reconstruct **problem pressure**;
9. state the core model, mechanism, distinction, or procedure;
10. identify claims, examples, non-examples, and failure modes;
11. explain how the concept could be tested, observed, or operationalized where appropriate;
12. add typed relations to existing concepts;
13. place the concept in an existing cluster or create a justified new cluster;
14. update the promoted-term index and corpus count;
15. update roadmap/cluster documentation when the corpus structure changed;
16. run validation;
17. regenerate disposable projections;
18. verify CI when available;
19. report what changed and what remains deliberately unresolved.

A concept is not fully integrated merely because `entries/<slug>.md` exists.

---

# 2. First question: new concept or existing concept?

Before creating anything, search the canonical corpus and relevant archive material.

Check:

- canonical term names;
- aliases;
- neighboring concepts;
- previous or superseded wording;
- archived recovered vocabulary;
- cluster maps.

Ask:

> **Is this actually a new conceptual object, or a refinement / alias / application of something already present?**

## Create a new entry when

- the term makes a distinction that existing entries cannot express;
- the mechanism has independent explanatory use;
- the phrase repeatedly compresses an insight worth preserving;
- the concept can plausibly acquire its own examples, boundaries, relations, or research questions;
- removing the term would make some recurring thought significantly harder to express.

## Revise an existing entry when

- the new discussion merely clarifies an existing definition;
- the supposed new term is a synonym rather than a distinct idea;
- the main value is a new example, counterexample, boundary, or implication;
- the new idea is clearly subordinate and has not yet demonstrated independent analytical use.

## Preserve a latent sub-concept when

A useful named distinction appears but does not yet deserve its own full entry.

Give it **registry presence** and preserve richer contextual prose inside the parent entry or cluster.

Examples of this pattern include:

- `intentional signifier`;
- `imposed signifier`;
- `content bypass`;
- `tone lock`;
- `open object`.

Do not explode every useful noun phrase into a Markdown article.

Conceptarium values **conceptual density**, not file count. Registry-only presence is specifically designed to preserve a name without forcing premature semantic materialization.

## Lazy capture requires almost nothing

When the session should not be interrupted, a concept can be captured with only:

~~~yaml
id: stable-slug
term: Exact term
presence: registered
materialization: registry-only
ontology_state: unplaced
~~~

A short note or context is useful but optional.

Do **not** invent domains, definitions, relations, or ontology merely to complete the record.

See [REGISTRY.md](REGISTRY.md).

---

# 3. Preserve exact language before normalizing

Some terms are labels.

Some terms are discoveries.

Some formulations preserve the insight with unusual precision.

If wording is doing real conceptual work, preserve it exactly.

Examples:

- “Knowledge needs a return address.”
- “Bring your own model.”
- “There is still more game to absorb.”
- “Modelability is not ownership.”

Do not casually rename a user-coined or jointly coined term merely because another phrase sounds more academic.

If a better term later appears:

1. preserve the old term;
2. add an alias or successor relation;
3. explain the refinement;
4. never silently rewrite conceptual history.

See [EDITORIAL.md](EDITORIAL.md).

---

# 4. Reconstruct the problem pressure

This is one of the most important requirements in the repository.

A concept should not read as though it appeared fully formed.

The **Problem pressure** section records the frustration, anomaly, repeated observation, failed vocabulary, contradiction, or intellectual need that forced the term into existence.

Recover as much of the following as possible:

- What kept happening?
- What was irritating or confusing?
- What existing words failed?
- Which examples made the distinction necessary?
- What was the speaker trying to explain?
- What conceptual mistake kept recurring?
- What would become harder to think if the term vanished?

For example, `closure pressure` is not adequately preserved by writing:

> “A desire to finish unfinished things.”

The birth pressure included the much more specific experience:

> “You know that game X. You didn’t finish X. There is still more game to absorb. You should finish that game.”

That phenomenology is part of the concept.

## Do not sanitize the origin into abstraction

The polished definition and the messy source pressure serve different purposes.

Keep both.

---

# 5. Establish provenance honestly

Use the best evidence actually available.

Frontmatter origin fields are defined in [SCHEMA.md](SCHEMA.md).

Record:

- first known date;
- authorship;
- certainty;
- optional note.

Possible authorship values:

- `user`
- `assistant`
- `joint`
- `pre-existing`
- `unknown`

Possible certainty values:

- `exact`
- `approximate`
- `reconstructed`
- `unknown`

## Authorship should describe what actually happened

A common pattern is:

- assistant proposes the label;
- user recognizes it as important and supplies the defining distinction;
- concept is therefore historically mixed even if the literal phrase has one proposer.

In that case, the body should explain the actual division of labor rather than flattening it.

## Never fabricate provenance

If an AI agent does not have the originating conversation:

- inspect the repository;
- inspect archive material if relevant;
- use Git history if useful;
- mark provenance as reconstructed or unknown.

Do not invent exact dates, quotations, priority, or authorship.

---

# 6. Choose maturity deliberately

Statuses describe **vocabulary maturity**, not truth.

## `seed`

Use when the idea is worth preserving but still difficult to define.

## `provisional`

Use when the concept is recognizable but boundaries or wording remain unstable.

## `canonical`

Use when the term is stable enough that Conceptarium intends to reuse it deliberately.

Canonical does **not** mean scientifically proven.

A canonical term may organize speculative hypotheses.

## `contested`

Use when the term remains useful but its meaning or attached theory is actively disputed.

## `deprecated`

Use when a better term supersedes it.

Preserve it for genealogy.

## `archived`

Use when an entry is historical rather than active vocabulary.

---

# 7. Choose the primary type

Allowed primary types are documented in [SCHEMA.md](SCHEMA.md):

- concept
- distinction
- mechanism
- framework
- phrase
- failure-mode
- question
- method
- principle
- metaphor

Choose the type that best describes what the entry **does**.

Do not create multiple entries merely because a concept participates in several roles.

---

# 8. Materialize the registry concept

Before writing a full entry, confirm that the stable ID already exists in `registry/concepts.yml`.

Promotion changes the registry record from:

~~~yaml
materialization: registry-only
~~~

to:

~~~yaml
materialization: entry
entry: entries/stable-slug.md
~~~

The stable identity should survive promotion unchanged.

# 9. Write the canonical frontmatter

Every promoted entry should normally contain:

```yaml
---
id: stable-slug
term: Display term
type: concept
status: canonical
gloss: >-
  A compact dictionary-quality definition.
domains:
  - domain-one
aliases: []
origin:
  date: 2026-09-05
  authorship: joint
  certainty: exact
  note: >-
    Optional provenance summary.
relations:
  - type: contrasts-with
    target: neighboring-concept
---
```

Requirements:

- filename should normally equal `id + ".md"`;
- IDs are stable machine identifiers;
- `gloss` must make sense without the full article;
- domains should be broad and reusable;
- aliases should preserve meaningful old or alternate wording;
- relations must use the sentence test.

Do not edit generated `build/` files.

They are projections, not source.

---

# 10. Write the encyclopedic body

A mature canonical entry should attempt most of the following sections.

The exact order can vary when the concept demands it.

## Definition

Explain the concept precisely and at greater depth than the gloss.

State the distinctive structure.

## Dictionary projection

Optional explicit compact definition if useful.

The frontmatter gloss remains authoritative for generation.

## Problem pressure

Required for important local concepts whenever origin can be reconstructed.

Preserve why the concept had to exist.

## Core model / mechanism

Show the structure.

ASCII diagrams are encouraged when they reveal causal or conceptual organization.

Example:

```text
unfinished valued object
        ↓
represented remainder
        ↓
object stays mentally open
        ↓
closure pressure
        ↓
return / completion / abandonment
```

## Internal distinctions

If the concept contains separable forms, state them explicitly.

Example:

```text
consumer signifier
  ├── intentional
  └── imposed
```

Do not immediately promote every branch into its own entry.

## Claims and implications

Separate:

- definition;
- mechanism;
- empirical hypothesis;
- normative claim;
- prediction;
- metaphorical extension.

Do not let precise prose create false evidential certainty.

## Examples

Use examples as tests.

Prefer multiple kinds when possible:

- clean;
- borderline;
- surprising;
- cross-domain;
- originating example.

## Non-examples

State what superficially resembles the concept but should not count.

This is essential for preventing conceptual inflation.

## Boundaries and failure modes

Ask:

- Where does the term overreach?
- What would make it unfalsifiable?
- What neighboring concept could swallow it?
- What common misuse should be prevented?
- Can the term become a moral accusation when it is only descriptive?

## Operationalization / evidence

Where appropriate, explain what evidence would support, weaken, or identify the concept.

For design concepts, explain what an implementation would expose.

For interpretive concepts, explain what observations distinguish it from alternatives.

## Relations

Explain major graph edges in prose.

Metadata carries navigation.

The body carries theory.

## Provenance

Recommended subsections:

- First known appearance
- Immediate context
- Authorship
- Problem being solved
- Conceptual ancestors
- External antecedents
- Later refinements

## Open questions

Do not prematurely seal the theory.

Preserve the next research pressure.

## Revision history

Record **semantic changes**, not every textual edit.

Git already stores textual history.

---

# 11. Distinguish local concepts from external literature

A Conceptarium term may resemble an existing scholarly concept.

Do not erase local discovery.

Default sequence:

1. preserve the local concept;
2. preserve local provenance;
3. identify possible external neighbors;
4. compare them carefully;
5. only then decide whether the relationship is:
   - alias;
   - overlap;
   - refinement;
   - specialization;
   - independent rediscovery;
   - genuine distinction.

Do not perform external-literature normalization merely to make the vocabulary sound standard.

If external research has not been performed, say so.

A useful placeholder is:

> “Potential external neighbors to investigate later include…”

This is better than pretending equivalence.

---

# 12. Add typed relations

Relations are theoretical claims.

Read [RELATIONS.md](RELATIONS.md) before inventing new verbs.

Every edge should pass:

> **SOURCE relation TARGET.**

For example:

> Genre lock **is associated with** closure pressure.

Avoid relation fragments.

Prefer existing verbs.

New verbs are allowed only when they preserve a distinction the current ontology cannot express.

## Relations should not be decorative

Do not add five edges merely because five concepts are vaguely nearby.

Add an edge when the relationship itself is worth remembering.

## Registry-only targets

A relation target does **not** need a full entry, but it must have predicate presence.

~~~text
full entry target     → valid
registry-only target  → valid unfinished ontology
unregistered target   → broken reference / validation error
~~~

If a relation needs a concept that is not yet developed, capture the target in the registry instead of inventing a stub article.

---

# 13. Update the conceptual neighborhood

A new entry should be placed in the intellectual map.

Check existing files in `clusters/`.

## Add to an existing cluster when

- the concept clearly belongs to an established research program;
- it sharpens a branch already represented there;
- it creates an important bridge between existing entries.

## Create a new cluster when

- at least two or more concepts form a coherent research neighborhood;
- the new grouping exposes a useful shared mechanism or question;
- the cluster is likely to grow;
- no existing cluster can represent the relationship without distortion.

Do **not** create one cluster per concept.

Clusters are curated pedagogical maps, not folders.

## Cluster documents may preserve latent vocabulary

A cluster is a good place to record sub-concepts that are not yet mature enough for individual entries.

---

# 14. Update the human index

Every newly promoted canonical entry must be added to:

`indexes/terms.md`

Requirements:

- alphabetical placement;
- one-line useful gloss;
- update **Current promoted corpus** count.

The **promotion queue is not maintained here**. It is generated from registry records whose `materialization` is `registry-only`.

Do not leave a materialized entry invisible to the human index, and do not leave an unmaterialized named concept outside the registry.

---

# 15. Update roadmap or structural docs when warranted

Not every concept requires a roadmap edit.

Update [ROADMAP.md](ROADMAP.md) when the integration changes things such as:

- promoted corpus count;
- active major cluster list;
- phase status;
- tooling status;
- architectural direction.

Update schema/editorial/relation docs when the integration reveals a new repository-wide rule.

Do not bury new policy inside a single concept entry.

---

# 16. Validation is part of integration

A concept is not considered fully integrated until the corpus validates.

From the repository root:

```bash
python -m pip install -r requirements-dev.txt
python tools/validate.py
```

The validator checks structural properties including:

- registry parsing and uniqueness;
- predicate presence for every materialized entry;
- registry/entry path and term agreement;
- relation targets resolving to registered concepts;
- frontmatter parsing;
- stable IDs;
- filename/ID agreement;
- allowed types;
- allowed statuses;
- domains;
- aliases;
- origin fields;
- relation structure;
- relation vocabulary;
- alias collisions;
- maturity expectations.

Unknown relation verbs may remain warnings. Unregistered relation targets are errors; registry-only targets are valid.

Use:

```bash
python tools/validate.py --strict
```

when explicitly auditing all warnings.

## Do not “fix” warnings by deleting useful theory

A dangling relation or new relation verb may expose real unfinished ontology work.

Investigate first.

---

# 17. Regenerate disposable projections

Run:

```bash
python tools/project.py
```

Current projections include:

- dictionary;
- registry-aware graph;
- research frontier;
- problem-pressure index;
- promotion queue;
- machine-readable registry;
- machine-readable catalog.

The generated `build/` directory is disposable and ignored.

Never manually patch generated files to make output look correct.

Fix the canonical source or generator.

---

# 18. Verify CI

GitHub Actions should run validation and projection generation.

When direct repository access exists, verify that the final relevant workflow run succeeds.

A rigorous integration report should distinguish:

- Git accepted the commit;
- validator passed;
- projections built;
- CI completed successfully.

These are not the same claim.

If CI is unavailable, state that local validation/projection succeeded instead.

---

# 19. Integration should preserve uncertainty

Do not convert an exploratory conversation into dogma.

Useful patterns:

> “candidate mechanism”

> “working hypothesis”

> “possible external neighbor”

> “this distinction may deserve its own entry if it recurs”

> “the stronger claim is not yet established”

Conceptarium exists to preserve thought **without laundering uncertainty**.

---

# 20. Do not over-promote sub-concepts

A strong entry often creates useful internal vocabulary.

Examples:

```text
Consumer semiotics
├── intentional signifier
├── imposed signifier
├── allegiance inference
└── content bypass
```

or:

```text
Genre lock
├── tone lock
├── latent feature bundle
└── motivational admissibility
```

These are not automatically separate concepts.

Promote them when they:

- recur independently;
- acquire examples of their own;
- connect to multiple other entries;
- become necessary to state later arguments;
- develop their own boundaries or research questions.

Until then, preserve them inside the parent or cluster.

---

# 21. Revise existing concepts when new discussion changes them

“Integrate today’s concepts” does not mean “only add new files.”

The integration pass should also ask:

- Did today’s conversation sharpen an old distinction?
- Did it falsify part of an entry?
- Did a new example reveal overreach?
- Did an old concept acquire a better name?
- Did two concepts turn out to be linked?
- Did an amber phrase emerge?
- Did provenance become more precise?

Update existing entries when necessary.

Record semantic changes in revision history.

---

# 22. Session-level integration commands

The repository is designed to support several natural workflows.

## “Mark that for Conceptarium”

Meaning:

> Give this concept predicate presence **now**, but do not derail the current thinking session by materializing its ontology.

The expected action is a registry-only capture, usually no more than stable ID + term + date/context if available. A later integration pass can develop it.

## “Integrate this concept”

Meaning:

> Perform the complete workflow in this document for the named concept.

## “Integrate today’s concepts”

Meaning:

> Review the relevant session/day, identify new concepts, distinctions, phrases, and refinements, then integrate everything that meets the threshold.

This command should not require the user to remember every coined term.

The agent is expected to notice conceptual work.

## “Do a Conceptarium pass”

Meaning:

> Perform a broader archaeology and maintenance pass over recent material: new entries, refinements, missed provenance, relation cleanup, and latent concepts.

---

# 23. Agent behavior expectations

An AI agent integrating Conceptarium material should behave as an **editor and research-memory maintainer**, not a transcription bot.

The agent should:

- notice concepts the user did not explicitly enumerate;
- preserve the user’s specialized vocabulary;
- reconstruct intellectual pressure;
- distinguish a new concept from a renamed old one;
- resist premature academic normalization;
- preserve speculation as speculation;
- add boundaries rather than flattering every idea into universality;
- connect concepts through typed theory-bearing relations;
- prefer deep entries over mass stub generation;
- preserve named sub-concepts in the registry without automatically materializing them;
- prefer registry-only capture whenever deeper integration would interrupt or exhaust the session;
- validate the repository after editing;
- report exactly what was changed.

The agent should **not**:

- silently rename meaningful terms;
- invent provenance;
- treat “canonical” as “proven true”;
- turn every phrase into an entry;
- create duplicate dictionary/encyclopedia sources;
- edit generated projections as canonical data;
- add vague graph edges for decoration;
- omit the originating frustration because the polished definition sounds cleaner;
- claim integration is complete before validation.

---

# 24. Full integration checklist

Use this checklist before declaring a concept integrated.

## Discovery

- [ ] Searched canonical entries for same or neighboring concept.
- [ ] Checked aliases and relevant archive material.
- [ ] Decided new entry vs. revision vs. latent sub-concept.

## Predicate presence

- [ ] Stable concept ID exists in `registry/concepts.yml`.
- [ ] Registry term preserves meaningful wording.
- [ ] Materialization state is accurate.
- [ ] Ontology state is honest; unknown/unplaced is allowed.

## Preservation

- [ ] Preserved exact term and valuable original wording.
- [ ] Reconstructed problem pressure.
- [ ] Recorded provenance without fabricating certainty.

## Canonical entry

- [ ] Stable ID and filename.
- [ ] Appropriate type.
- [ ] Appropriate maturity status.
- [ ] Standalone gloss.
- [ ] Domains.
- [ ] Aliases where useful.
- [ ] Origin metadata.
- [ ] Typed relations.
- [ ] Long-form definition.
- [ ] Core model / mechanism / distinction.
- [ ] Claims and implications where relevant.
- [ ] Examples.
- [ ] Non-examples.
- [ ] Boundaries and failure modes.
- [ ] Operationalization / evidence where relevant.
- [ ] Relations explained in prose.
- [ ] Provenance.
- [ ] Open questions.
- [ ] Semantic revision history.

## Corpus integration

- [ ] Registry record points to the canonical entry after promotion.
- [ ] Added to appropriate cluster.
- [ ] Created a new cluster only if justified.
- [ ] Added to `indexes/terms.md`.
- [ ] Updated promoted corpus count.
- [ ] Updated roadmap/docs if architecture changed.
- [ ] Preserved promising sub-concepts without unnecessary promotion.

## Verification

- [ ] `python tools/validate.py` passes.
- [ ] `python tools/project.py` succeeds.
- [ ] Investigated warnings rather than deleting useful theory.
- [ ] CI verified when available.

## Final report

- [ ] State which files were created/updated.
- [ ] State the new promoted corpus count if it changed.
- [ ] Summarize the concept’s central model.
- [ ] Mention important latent sub-concepts.
- [ ] State validation/projection/CI status accurately.

---

# 25. The governing principle

Conceptarium exists because conceptual work is easy to lose.

Integration therefore has three simultaneous obligations:

> **Do not lose conceptual identity while waiting for energy or clarity.**

> **Do not lose the messy origin.**

and

> **Do not leave the concept messy forever when further materialization becomes useful.**

The repository should preserve enough of the original intellectual event to recover why the term mattered, while giving the concept enough structure that another human or AI agent can reuse, challenge, extend, and connect it later.

That is what “integrate this concept” means.
