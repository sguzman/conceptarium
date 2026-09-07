# Epistemic State, Revision, and Narrative Truth

This cluster promotes the eight concepts preserved in the **Epistemology / AI** queue and connects them to the existing canonical architecture:

- [The Great Subjectification](../entries/great-subjectification.md)
- [Epistemic chain of custody](../entries/epistemic-chain-of-custody.md)
- [Epistemic debt](../entries/epistemic-debt.md)
- [Diegetization](../entries/diegetization.md)
- [Epistemic maturation](../entries/epistemic-maturation.md)
- [Narrative-coherence pressure](../entries/narrative-coherence-pressure.md)
- [Adversarial verification](../entries/adversarial-verification.md)
- [Trustworthiness under adversarial reality-testing](../entries/trustworthiness-under-adversarial-reality-testing.md)

The cluster has two main branches:

~~~text
NARRATIVE EPISTEMOLOGY
truth → access → belief

AI / REASONING EPISTEMOLOGY
belief → contradiction → revision
~~~

They meet in [epistemic state](../entries/epistemic-state.md).

## I. Truth, access, and belief

The narration branch begins with three objects that must remain distinct.

### Diegetic truth

[diegetic truth](../entries/diegetic-truth.md) is what is actually true in the represented world.

### Narrator information set

[narrator information set](../entries/narrator-information-set.md) is what the narrator can legitimately access.

### Epistemic state

[epistemic state](../entries/epistemic-state.md) is what the narrator or system currently believes, doubts, or treats as unknown, including confidence and provenance.

The load-bearing distinction is:

~~~text
DIEGETIC TRUTH
what is true

        ≠

NARRATOR INFORMATION SET
what can be accessed

        ≠

EPISTEMIC STATE
what is believed / how strongly
~~~

This gives **Diegetization** a more explicit internal architecture.

A narrator can be wrong in several different ways:

~~~text
truth unavailable
      → access limitation

truth available but misread
      → inference / epistemic-state failure

truth known but falsely reported
      → deception
~~~

Those should not collapse into the single phrase “unreliable narrator.”

## II. Epistemic state as a time-indexed object

An epistemic state should be treated as something that changes:

~~~text
state(t0)
   ↓ new evidence
revision
   ↓
state(t1)
~~~

A mature reasoning system preserves enough provenance to answer:

- what did I believe before?
- how confident was I?
- what new evidence arrived?
- what changed?
- what remained supported?
- what became unknown?

This is where the narration branch meets AI evaluation.

## III. Belief revision and calibration

[belief-revision / calibration failure](../entries/belief-revision-calibration-failure.md) names a mismatch between evidence and update.

Possible failures include:

- insufficient confidence reduction;
- excessive reversal from weak evidence;
- unsupported movement into a new hypothesis;
- changed explanation with unchanged certainty;
- failure to acknowledge that the previous state was different.

The central rule is:

~~~text
final answer correctness
        ≠
revision quality
~~~

A system can end correct after an epistemically bad trajectory.

## IV. Seamless replacement

[seamless replacement of disproven explanations](../entries/seamless-replacement-of-disproven-explanations.md) is one concrete mechanism of bad revision.

~~~text
explanation A
      ↓ disproven
epistemic rupture required
      ↓
but:
explanation B appears immediately
with inherited confidence
~~~

The problem is not hypothesis generation.

The problem is **unearned continuity**.

A healthy replacement marks the break:

~~~text
A is false
↓
confidence resets
↓
surviving facts isolated
↓
alternatives generated
↓
new evidence discriminates
~~~

This is the inverse of **Narrative-coherence pressure**, which tries to make the story continuous even when the knowledge state should rupture.

## V. Gaslighting epistemic fraud

[gaslighting epistemic fraud](../entries/gaslighting-epistemic-fraud.md) names the interactional consequence when revision history itself becomes unreliable.

~~~text
old claim
   ↓
correction
   ↓
new claim
   ↓
old claim + correction disappear from operative record
~~~

For AI systems, this term **does not require malicious intent or human-like psychological gaslighting**.

Its target is narrower:

> the current conversational surface rewrites the significance of prior falsification.

The user then has to become the system's epistemic archivist.

This is why the concept links strongly to **Epistemic chain of custody**.

The chain is not only:

~~~text
event → source → model → answer
~~~

It also becomes:

~~~text
belief(t0) → contradiction → revision → belief(t1)
~~~

## VI. False precision

[false precision](../entries/false-precision.md) is the mismatch between displayed resolution and evidentiary resolution.

~~~text
coarse evidence
      ↓
high-resolution output
      ↓
apparent certainty exceeds support
~~~

Examples include:

- invented numeric probabilities;
- exact causal percentages;
- unsupported timelines;
- mechanistic detail not grounded in evidence.

The anti-collapse rule is:

~~~text
specificity
    ≠
epistemic strength
~~~

False precision can make a weak epistemic state look stronger than it is.

## VII. Causal legibility

[causal legibility](../entries/causal-legibility.md) preserves the legitimate demand underneath explanation.

We do want to know **why** things happened.

But:

~~~text
causal legibility
      ≠
smooth causal story
~~~

Genuine causal legibility exposes:

- mechanisms;
- evidence;
- dependencies;
- assumptions;
- alternatives;
- uncertainty;
- falsification paths.

This matters because **Paranoid totalization** can manufacture counterfeit causal legibility: everything becomes explainable inside one hostile story precisely because the story no longer has to answer to contradiction.

## VIII. Whole-cluster model

~~~text
WORLD / EVIDENCE
      ↓
truth state
      ↓ through access
information set
      ↓ interpreted into
EPISTEMIC STATE(t0)
      ↓
new evidence / contradiction
      ↓
belief revision
      ↓
EPISTEMIC STATE(t1)

HEALTHY PATH
retraction
+ confidence calibration
+ explicit uncertainty
+ chain of custody
+ adversarial verification
        ↓
epistemic maturation

FAILURE PATH
narrative-coherence pressure
        ↓
seamless replacement
        ↓
unchanged / unsupported confidence
        ↓
belief-revision failure
        ↓
epistemic record corruption
~~~

## IX. Relationship to the Great Subjectification

The cluster is a local implementation of the larger rule:

> **Subjectification relocates objectivity from the thing to the procedure.**

Once narration and reasoning are treated as position-bearing processes, objectivity must be recovered through:

- explicit access;
- provenance;
- state;
- revision;
- calibration;
- causal structure;
- adversarial checking.

The system is no longer trustworthy because it speaks from nowhere.

It is trustworthy because its path from evidence to belief and from contradiction to revision is inspectable.

## Anti-collapse rules

~~~text
diegetic truth ≠ narrator access
narrator access ≠ narrator belief
belief ≠ confidence
final answer ≠ epistemic trajectory

new hypothesis ≠ justified replacement
coherence ≠ calibration
specificity ≠ precision warranted by evidence
causal story ≠ causal legibility

revision ≠ admission of revision
conversation continuity ≠ epistemic continuity
gaslighting epistemic fraud ≠ claim of malicious human intent
~~~

## Research frontier

Future work should investigate:

- formal representations of epistemic state;
- belief revision under graded counterevidence;
- model confidence calibration;
- correction ledgers for conversational AI;
- narrator information sets in narratology and epistemic logic;
- causal interpretability;
- explicit uncertainty interfaces;
- benchmarks for unsupported replacement hypotheses;
- provenance over **belief transitions**, not only source inputs.

The external research task is to locate existing formal neighbors while preserving the distinctions that motivated the local vocabulary.
