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
- `supports`
- `supported-by`
- `threatens`
- `prioritizes`
- `shapes`

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
| exemplifies | exemplified-by |
| counteracts | counteracted-by |

A future graph generator should synthesize inverse navigation instead of requiring redundant metadata everywhere.

## Experimental relations

The schema intentionally allows new verbs.

The validator should **warn**, not fail, when an unrecognized relation appears.

That warning means:

> “This relation expands the ontology. Is the distinction worth preserving?”

If yes, add it to this document.

## Dangling targets

A relation may temporarily point to a concept still preserved only in the migration archive.

During Phase 1 this is a warning, not an error.

Once the recovered lexicon is substantially promoted, strict relation validation can become the default.

## The sentence test

Before adding an edge, literally read it:

> **[source term] [relation] [target term].**

If the result is misleading, fragmentary, or semantically empty, rewrite the edge.

The graph is part of the theory.
