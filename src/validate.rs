use crate::corpus::Corpus;
use crate::project::registry_only_relation_targets;
use anyhow::Result;
use std::collections::{BTreeMap, BTreeSet};

const ALLOWED_PRESENCE: &[&str] = &["registered"];
const ALLOWED_MATERIALIZATION: &[&str] = &["entry", "registry-only"];
const ALLOWED_ONTOLOGY_STATES: &[&str] = &[
    "unassessed",
    "unplaced",
    "roughly-classified",
    "domain-placed",
    "related",
    "deeply-integrated",
];
const ALLOWED_TYPES: &[&str] = &[
    "concept",
    "distinction",
    "mechanism",
    "framework",
    "phrase",
    "failure-mode",
    "question",
    "method",
    "principle",
    "metaphor",
];
const ALLOWED_STATUSES: &[&str] = &[
    "seed",
    "provisional",
    "canonical",
    "contested",
    "deprecated",
    "archived",
];
const ALLOWED_AUTHORSHIP: &[&str] = &[
    "user",
    "assistant",
    "joint",
    "pre-existing",
    "unknown",
];
const ALLOWED_CERTAINTY: &[&str] = &[
    "exact",
    "approximate",
    "reconstructed",
    "unknown",
];
const BAD_FRAGMENT_RELATIONS: &[&str] = &["from", "into", "between", "to", "with"];
const PREFERRED_RELATIONS: &[&str] = &[
    "alias-of","refines","refined-by","supersedes","superseded-by","descends-from",
    "changes","changed-by","part-of","contains","specializes","generalizes","instance-of",
    "contains-instance","contrasts-with","distinguishes-from","overlaps-with","confused-with",
    "causes","caused-by","produces","produced-by","enables","enabled-by","constrains",
    "constrained-by","intensifies","intensified-by","relieves","relieved-by","threatens",
    "threatened-by","prevents","prevented-by","stabilizes","stabilized-by","requires",
    "required-by","uses","used-by","implements","implemented-by","operationalizes","realizes",
    "realized-by","explains","explained-by","predicts","tests","tested-by","evaluates",
    "exemplifies","exemplified-by","motivates","motivated-by","translates","expresses",
    "expressed-by","generates","generated-by","built-from","incorporates","characterizes",
    "characterized-by","associated-with","obscures","supports","supported-by","prioritizes",
    "shapes","shaped-by","solves","solved-by","pays","paid-by","compressed-by",
    "compressed-form-of","counteracts","counteracted-by","counters","moves-from",
    "moves-toward","measured-by","exposed-by","penalizes","populated-by","complements",
    "acts-through","communicates-through","depends-on","analogous-to",
];

pub fn run(corpus: &Corpus, strict: bool) -> Result<bool> {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    let mut registry_ids = BTreeSet::new();
    let mut registry_terms = BTreeMap::new();

    for (index, record) in corpus.registry.concepts.iter().enumerate() {
        let label = format!("registry/concepts.yml: concept[{index}]");
        if record.id.trim().is_empty() {
            errors.push(format!("{label}.id must be a non-empty string"));
        }
        if !registry_ids.insert(record.id.clone()) {
            errors.push(format!("{label}: duplicate registry id {:?}", record.id));
        }

        if record.term.trim().is_empty() {
            errors.push(format!("{label}.term must be a non-empty string"));
        } else {
            let key = record.term.to_lowercase();
            if let Some(owner) = registry_terms.insert(key, record.id.clone()) {
                if owner != record.id {
                    errors.push(format!(
                        "{label}: registry term {:?} duplicates term owned by {:?}",
                        record.term, owner
                    ));
                }
            }
        }

        if !ALLOWED_PRESENCE.contains(&record.presence.as_str()) {
            errors.push(format!("{label}: invalid presence {:?}", record.presence));
        }
        if !ALLOWED_MATERIALIZATION.contains(&record.materialization.as_str()) {
            errors.push(format!(
                "{label}: invalid materialization {:?}",
                record.materialization
            ));
        }
        if !ALLOWED_ONTOLOGY_STATES.contains(&record.ontology_state.as_str()) {
            errors.push(format!(
                "{label}: invalid ontology_state {:?}",
                record.ontology_state
            ));
        }

        match record.materialization.as_str() {
            "entry" if record.entry.as_deref().is_none_or(str::is_empty) => {
                errors.push(format!("{label}: materialized concept requires entry path"));
            }
            "registry-only" if record.entry.is_some() => {
                errors.push(format!("{label}: registry-only concept must not declare entry path"));
            }
            _ => {}
        }
    }

    if corpus.entries.is_empty() {
        errors.push("no canonical entries found".to_string());
    }

    let registry = corpus.registry_by_id();
    let mut entry_ids = BTreeSet::new();
    let mut terms = BTreeMap::new();
    let mut alias_owners: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

    for entry in &corpus.entries {
        let label = entry.path.display().to_string();
        if entry.meta.id != entry.path.file_stem().and_then(|x| x.to_str()).unwrap_or_default() {
            errors.push(format!(
                "{label}: id {:?} must match filename stem",
                entry.meta.id
            ));
        }
        if !entry_ids.insert(entry.meta.id.clone()) {
            errors.push(format!("{label}: duplicate id {:?}", entry.meta.id));
        }
        if entry.meta.term.trim().is_empty() {
            errors.push(format!("{label}: term must be a non-empty string"));
        }
        if entry.meta.gloss.trim().is_empty() {
            errors.push(format!("{label}: gloss must be a non-empty string"));
        }
        if !ALLOWED_TYPES.contains(&entry.meta.kind.as_str()) {
            errors.push(format!("{label}: invalid type {:?}", entry.meta.kind));
        }
        if !ALLOWED_STATUSES.contains(&entry.meta.status.as_str()) {
            errors.push(format!("{label}: invalid status {:?}", entry.meta.status));
        }
        if entry.meta.domains.iter().any(|x| x.trim().is_empty()) {
            errors.push(format!("{label}: domains must contain only non-empty strings"));
        }
        if entry.meta.aliases.iter().any(|x| x.trim().is_empty()) {
            errors.push(format!("{label}: aliases must contain only non-empty strings"));
        }
        if !ALLOWED_AUTHORSHIP.contains(&entry.meta.origin.authorship.as_str()) {
            errors.push(format!(
                "{label}: invalid origin.authorship {:?}",
                entry.meta.origin.authorship
            ));
        }
        if !ALLOWED_CERTAINTY.contains(&entry.meta.origin.certainty.as_str()) {
            errors.push(format!(
                "{label}: invalid origin.certainty {:?}",
                entry.meta.origin.certainty
            ));
        }
        if entry.meta.origin.date.trim().is_empty() {
            errors.push(format!("{label}: origin.date is required"));
        }

        for (index, relation) in entry.meta.relations.iter().enumerate() {
            let rel_label = format!("{label}: relation[{index}]");
            if relation.kind.trim().is_empty() {
                errors.push(format!("{rel_label}.type must be a non-empty string"));
            } else if BAD_FRAGMENT_RELATIONS.contains(&relation.kind.as_str()) {
                warnings.push(format!(
                    "{rel_label}: {:?} is a grammatical fragment; rewrite the relation so the edge stands alone",
                    relation.kind
                ));
            } else if !PREFERRED_RELATIONS.contains(&relation.kind.as_str()) {
                warnings.push(format!(
                    "{rel_label}: experimental relation type {:?}; document it in docs/RELATIONS.md if intentional",
                    relation.kind
                ));
            }

            if relation.target.trim().is_empty() {
                errors.push(format!("{rel_label}.target must be a non-empty string"));
            } else if !registry.contains_key(relation.target.as_str()) {
                errors.push(format!(
                    "{rel_label} targets unregistered concept {:?}; capture it first",
                    relation.target
                ));
            }
        }

        let key = entry.meta.term.to_lowercase();
        if let Some(owner) = terms.insert(key.clone(), entry.meta.id.clone()) {
            if owner != entry.meta.id {
                errors.push(format!(
                    "{label}: term {:?} duplicates canonical term owned by {:?}",
                    entry.meta.term, owner
                ));
            }
        }
        alias_owners
            .entry(key)
            .or_default()
            .insert(entry.meta.id.clone());
        for alias in &entry.meta.aliases {
            alias_owners
                .entry(alias.to_lowercase())
                .or_default()
                .insert(entry.meta.id.clone());
        }

        if entry.meta.status == "canonical" {
            for heading in ["## Problem pressure", "## Provenance", "## Open questions"] {
                if !entry.body.contains(heading) {
                    warnings.push(format!("{label}: canonical entry is missing {heading:?}"));
                }
            }
        }

        let Some(record) = registry.get(entry.meta.id.as_str()) else {
            errors.push(format!(
                "{label}: entry has no predicate presence in concept registry"
            ));
            continue;
        };
        if record.materialization != "entry" {
            errors.push(format!(
                "{label}: registry record must be materialization 'entry', not {:?}",
                record.materialization
            ));
        }
        let expected = entry.path.to_string_lossy().replace('\\', "/");
        if record.entry.as_deref() != Some(expected.as_str()) {
            errors.push(format!(
                "{label}: registry entry path {:?} must equal {:?}",
                record.entry, expected
            ));
        }
        if record.term != entry.meta.term {
            errors.push(format!(
                "{label}: registry term {:?} does not match entry term {:?}",
                record.term, entry.meta.term
            ));
        }
    }

    for record in &corpus.registry.concepts {
        if record.materialization != "entry" {
            continue;
        }
        if !entry_ids.contains(&record.id) {
            errors.push(format!(
                "registry/concepts.yml: {:?} claims materialization as an entry but no canonical entry exists",
                record.id
            ));
        }
    }

    for (text, owners) in alias_owners {
        if owners.len() > 1 {
            warnings.push(format!(
                "alias/term collision {:?} resolves to multiple entries: {}",
                text,
                owners.into_iter().collect::<Vec<_>>().join(", ")
            ));
        }
    }

    for message in &errors {
        eprintln!("ERROR: {message}");
    }
    for message in &warnings {
        eprintln!("WARNING: {message}");
    }

    let materialized = corpus
        .registry
        .concepts
        .iter()
        .filter(|record| record.materialization == "entry")
        .count();
    let registry_only = corpus
        .registry
        .concepts
        .iter()
        .filter(|record| record.materialization == "registry-only")
        .count();

    println!(
        "Registry contains {} concept(s): {} materialized, {} registry-only.",
        corpus.registry.concepts.len(),
        materialized,
        registry_only
    );

    let registry_only_targets = registry_only_relation_targets(corpus);
    if !registry_only_targets.is_empty() {
        println!(
            "{} relation target(s) resolve to registry-only concepts: {}",
            registry_only_targets.len(),
            registry_only_targets.into_iter().collect::<Vec<_>>().join(", ")
        );
    }

    println!(
        "Validated {} entries: {} error(s), {} warning(s).",
        corpus.entries.len(),
        errors.len(),
        warnings.len()
    );

    Ok(errors.is_empty() && (!strict || warnings.is_empty()))
}
