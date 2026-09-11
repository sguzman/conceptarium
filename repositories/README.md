# Repository Entities

`repositories/` contains shallow materializations of repository-scale ontology objects known to Conceptarium.

These are **not concept entries** and do not participate in the ordinary concept promotion queue.

Their predicate presence is canonical in [`registry/repositories.yml`](../registry/repositories.yml).

## Purpose

The repository category exists because a repository such as Politicarium or Salvatarium is not merely a concept about a repository. It is an external entity with its own local authority, state, artifacts, and ontology.

Conceptarium may know that such an entity exists and may maintain a coarse theory of its place in the Tarium ecology.

It must not silently become a mirror of that entity.

## Materialization contract

A repository materialization should normally contain only:

- canonical repository identity;
- broad purpose;
- declared Tarium-family status;
- Conceptarium's coarse external classification;
- the knowledge boundary;
- the authority boundary.

It should not contain:

- exhaustive artifact inventories;
- local conclusions;
- private records;
- detailed object state;
- copied internal schemas unless intentionally published for interoperability.

## Authority rule

> **The source repository is authoritative for itself. Conceptarium is authoritative only for Conceptarium's external description of it.**

A classification here is therefore descriptive and revisable, not legislation imposed on the source repository.

See [`docs/TARIUM_ECOLOGY.md`](../docs/TARIUM_ECOLOGY.md) for the governing architecture.
