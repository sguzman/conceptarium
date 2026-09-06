# Relation Ontology

Conceptarium relations are **typed theoretical claims**, not decorative backlinks.

A relation should let us read:

> **SOURCE _relation_ TARGET**

as an intelligible statement.

For example:

> Diegetization **solves** epistemic debt.

The edge itself preserves information that would be lost by a flat “related concepts” list.

## Design rules

### 1. One edge, one claim

Avoid relation fragments that require another edge to become grammatical.

Bad:

```yaml
- type: converts
  target: rising-fallen
- type: into
  target: established-fallen
```

Better:

```yaml
- type: incorporates
  target: rising-fallen
- type: produces
  target: established-fallen
```

The graph may render edges independently.

### 2. Prefer existing relation types

A new relation verb is allowed when it preserves a distinction that existing verbs would erase.

Do not create synonyms merely for prose variety.

### 3. Direction matters

Choose the direction that expresses the useful claim.

```yaml
diegetization:
  - type: solves
    target: epistemic-debt
```

is usually more informative than starting from epistemic debt.

Inverse edges may be generated later where the inverse is mechanically known.

### 4. Relations do not replace prose

Important relationships should also be explained in the article body.

The metadata edge exists for projection and navigation.

## Preferred relation families

The vocabulary remains extensible, but these families should cover most cases.

### Identity and conceptual evolution

- `alias-of`
- `refines`
- `refined-by`
- `supersedes`
- `superseded-by`
- `descends-from`
- `changes`
- `changed-by`

### Structure and taxonomy

- `part-of`
- `contains`
- `specializes`
- `generalizes`
- `instance-of`
- `contains-instance`

### Contrast and distinction

- `contrasts-with`
- `distinguishes-from`
- `overlaps-with`
- `confused-with`

### Causation and dynamics

- `causes`
- `caused-by`
- `produces`
- `produced-by`
- `enables`
- `enabled-by`
- `constrains`
- `constrained-by`
- `intensifies`
- `intensified-by`
- `relieves`
- `relieved-by`
- `threatens`
- `threatened-by`
- `prevents`
- `prevented-by`
- `stabilizes`
- `stabilized-by`
- `counteracts`
- `counteracted-by`

### Dependency and implementation

- `requires`
- `required-by`
- `uses`
- `used-by`
- `implements`
- `implemented-by`
- `operationalizes`
- `realizes`
- `realized-by`

### Epistemic and explanatory

- `explains`
- `explained-by`
- `predicts`
- `tests`
- `tested-by`
- `evaluates`
- `exemplifies`
- `exemplified-by`
- `motivates`
- `motivated-by`

### Representation and generation

- `translates`
- `expresses`
- `expressed-by`
- `generates`
- `generated-by`
- `built-from`

### Social / institutional

These are allowed where they genuinely preserve theory:

- `incorporates`
- `characterizes`
- `characterized-by`
- `associated-with`
- `obscures`
- `supports`
- `supported-by`
- `threatens`
- `prioritizes`
- `shapes`
- `shaped-by`

If one of these begins to acquire several incompatible meanings, split it into more precise verbs.

## Symmetric relations

Some relations are conceptually symmetric:

- `contrasts-with`
- `overlaps-with`
- `associated-with`
- `confused-with`

A renderer may eventually synthesize backlinks, but canonical entries do not currently need to store both directions.

## Inverse pairs

Some relations have obvious inverses:

| Forward | Inverse |
| --- | --- |
| part-of | contains |
| specializes | generalizes |
| causes | caused-by |
| produces | produced-by |
| enables | enabled-by |
| constrains | constrained-by |
| intensifies | intensified-by |
| relieves | relieved-by |
| threatens | threatened-by |
| requires | required-by |
| uses | used-by |
| implements | implemented-by |
| generates | generated-by |
| realizes | realized-by |
| motivates | motivated-by |
| expresses | expressed-by |
| refines | refined-by |
| supersedes | superseded-by |
| changes | changed-by |
| prevents | prevented-by |
| stabilizes | stabilized-by |
| characterizes | characterized-by |
| supports | supported-by |
| explains | explained-by |
| shapes | shaped-by |
| exemplifies | exemplified-by |
| counteracts | counteracted-by |

A future graph generator should synthesize inverse navigation instead of requiring redundant metadata everywhere.

## Experimental relations

The schema intentionally allows new verbs.

The validator should **warn**, not fail, when an unrecognized relation appears.

That warning means:

> “This relation expands the ontology. Is the distinction worth preserving?”

If yes, add it to this document.

## Registry-only targets

Every relation target must have **predicate presence** in the Concept Registry.

Three states are possible:

~~~text
target → materialized entry
    valid, semantically developed

target → registry-only concept
    valid, semantically unmaterialized

target → no registry record
    broken conceptual reference
~~~

A registry-only target is deliberately allowed. It lets graph structure get ahead of article-writing without losing identity.

An unregistered target is a structural error: capture the concept in `registry/concepts.yml` before using it as an edge target.

## The sentence test

Before adding an edge, literally read it:

> **[source term] [relation] [target term].**

If the result is misleading, fragmentary, or semantically empty, rewrite the edge.

The graph is part of the theory.
