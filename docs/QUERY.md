# Local Query and Backend Architecture

Conceptarium is **Rust-first infrastructure**.

Canonical knowledge remains plain Markdown/YAML:

~~~text
registry/concepts.yml
entries/*.md
~~~

The `conceptarium` Rust crate parses those sources directly. Databases and indexes are **disposable projections**, never canonical stores.

## Current local query surface

After cloning the repository:

~~~bash
cargo run -- get fallen-constitutionalism
cargo run -- search "humiliation"
cargo run -- list --domain epistemology
cargo run -- relations fallen-constitutionalism
cargo run -- path fallen-constitutionalism anti-domination
cargo run -- queue
cargo run -- validate
cargo run -- project
~~~

The repository also defines a Cargo alias:

~~~bash
cargo conceptarium get fallen-constitutionalism
~~~

Installing the binary with `cargo install --path .` gives the direct form:

~~~bash
conceptarium search "correction"
~~~

## Backend doctrine

~~~text
CANONICAL MARKDOWN / YAML
          ↓
   conceptarium-core
          ↓
  local query semantics
          ↓
disposable indexes / stores
~~~

No backend is allowed to become a second source of semantic truth.

Delete every generated database or index and Conceptarium must be reconstructible from the canonical corpus.

## Development order

### 0. In-memory + petgraph — ACTIVE

The Rust core currently provides:

- corpus parsing;
- stable concept resolution;
- metadata filtering;
- scan search;
- typed incoming/outgoing relation queries;
- shortest directed graph paths;
- registry mutation;
- validation;
- all seven existing projections.

This is intentionally sufficient for a corpus of hundreds or thousands of concepts without a database server.

### 1. Tantivy — ACTIVE

Purpose:

- BM25 ranked full-text search;
- phrase queries;
- field-specific search;
- fast reverse lookup by problem pressure;
- aliases, definitions, examples, provenance, and open questions as separate fields.

The planned index is disposable under `.conceptarium/tantivy/`.

Tantivy is the first persistent backend because search quality creates immediate research value without forcing an ontology or storage model.

Build it with:

~~~bash
cargo run --quiet -- index build
~~~

Then search with BM25 automatically:

~~~bash
cargo run --quiet -- search 'humiliation status restoration'
cargo run --quiet -- search 'correction' --field problem-pressure
~~~

If the Tantivy index does not exist, `search` deliberately falls back to direct scan search over the canonical corpus.

### 2. SQLite — ACTIVE

Purpose:

- universal relational projection;
- ad hoc SQL;
- interoperability with external tools;
- simple analytics over concepts, domains, aliases, relations, and provenance.

Generated artifact:

~~~text
.conceptarium/conceptarium.sqlite
~~~

Build and query it with:

~~~bash
cargo run --quiet -- sqlite build
cargo run --quiet -- sqlite query "SELECT term, type, status FROM concepts WHERE type = 'mechanism' ORDER BY term"
cargo run --quiet -- sqlite query "SELECT source, predicate, target FROM relations WHERE target = 'anti-domination'"
~~~

The CLI opens the projection **read-only** for queries. Mutations belong in canonical Markdown/YAML and are propagated by rebuilding the projection.

### 3. SurrealDB — ACTIVE

Purpose:

- embedded local document + graph querying;
- richer relation traversal without a separate server;
- mixed metadata/body/graph predicates in SurrealQL;
- a future location for vector/document co-location experiments.

Conceptarium uses **SurrealKV**, so the embedded backend remains Rust-native and persistent without requiring RocksDB or a separate SurrealDB server.

Generated store:

~~~text
.conceptarium/surreal/
~~~

Build it from the canonical corpus:

~~~bash
cargo run --quiet -- surreal build
~~~

Query concept documents:

~~~bash
cargo run --quiet -- surreal query "SELECT concept_id, term, kind, status FROM concept WHERE 'epistemology' IN domains ORDER BY term"
~~~

Query graph edges:

~~~bash
cargo run --quiet -- surreal query "SELECT source_id, predicate, target_id FROM relation WHERE predicate = 'supports' ORDER BY source_id"
~~~

Use SurrealDB graph traversal syntax directly:

~~~bash
cargo run --quiet -- surreal query "SELECT concept_id, ->relation[WHERE predicate = 'supports']->concept.term AS supports FROM type::record('concept', 'fallen-constitutionalism')"
~~~

The CLI intentionally accepts only one read-only `SELECT`, `RETURN`, or `INFO` statement. Canonical changes belong in Markdown/YAML and are propagated by rebuilding the store.

SurrealDB is a query laboratory and projection, not Conceptarium's canonical model.

On x86-64 Windows, Conceptarium's `.cargo/config.toml` enables AWS-LC's official prebuilt NASM objects (`AWS_LC_SYS_PREBUILT_NASM=1`), so building the embedded SurrealDB SDK does not require a separate NASM installation.


### 4. Oxigraph — NEXT

Purpose:

- RDF projection;
- SPARQL;
- explicit ontology DAGs;
- broader/narrower domain traversal;
- formal concept/relation interoperability.

Oxigraph should become more important as deeper ontology requirements emerge from real classification pressure.

### 5. Qdrant Edge — PLANNED

Purpose:

- dense semantic embeddings;
- sparse/BM25 vectors where useful;
- problem-pressure retrieval;
- semantic neighborhoods;
- vector-near concepts lacking explicit graph edges;
- ontology-completion suggestions.

Vector geometry remains a projection rather than semantic authority.

## Backend interface direction

The intended long-term architecture is:

~~~text
                    conceptarium
                         │
                 Rust domain model
                         │
          ┌──────────────┼───────────────┐
          │              │               │
       petgraph        Tantivy        projections
          │                              │
          │              ┌───────────────┼──────────────┐
          │              │               │              │
          │            SQLite         SurrealDB      Oxigraph
          │                                              │
          └──────────────────────────────────────── Qdrant Edge
~~~

All backends should consume the same Rust domain objects and stable concept IDs.

## Query philosophy

Different engines answer different questions.

- **petgraph** — What explicit theoretical path connects A to B?
- **Tantivy** — Which concepts literally discuss this idea, especially in a particular semantic field?
- **SQLite** — What structured slice or aggregate satisfies this predicate?
- **SurrealDB** — What mixed document/graph query is convenient to express as one operation?
- **Oxigraph** — What follows from the explicit ontology and RDF relation model?
- **Qdrant Edge** — What is semantically near this thought even when vocabulary differs?

The future hybrid query layer should be able to combine results without collapsing these forms of evidence into one score.
