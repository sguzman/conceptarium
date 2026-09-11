use serde::Deserialize;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
struct RepositoryRegistry {
    version: u64,
    registry: RegistryMeta,
    repositories: Vec<RepositoryRecord>,
}

#[derive(Debug, Deserialize)]
struct RegistryMeta {
    object_kind: String,
    canonicality: String,
    conceptarium_role: String,
    knowledge_scope: String,
    source_authority: String,
    policy: RegistryPolicy,
}

#[derive(Debug, Deserialize)]
struct RegistryPolicy {
    permitted: Vec<String>,
    excluded_by_default: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RepositoryRecord {
    id: String,
    name: String,
    repository: String,
    family_status: String,
    role: String,
    purpose: String,
    materialization: String,
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn load_registry() -> RepositoryRegistry {
    let path = root().join("registry/repositories.yml");
    let text = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
    serde_yaml::from_str(&text)
        .unwrap_or_else(|error| panic!("failed to parse {}: {error}", path.display()))
}

fn require_contains(values: &[String], expected: &str) {
    assert!(
        values.iter().any(|value| value == expected),
        "expected registry policy to contain {expected:?}"
    );
}

fn assert_materialization(path: &Path) {
    assert!(path.exists(), "repository materialization is missing: {}", path.display());
    let text = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
    assert!(
        text.contains("## Conceptarium knowledge boundary"),
        "{} must declare the Conceptarium knowledge boundary",
        path.display()
    );
    assert!(
        text.contains("## Authority boundary"),
        "{} must declare the authority boundary",
        path.display()
    );
}

#[test]
fn repository_registry_is_predicate_only_and_locally_sovereign() {
    let registry = load_registry();

    assert_eq!(registry.version, 1);
    assert_eq!(registry.registry.object_kind, "repository");
    assert_eq!(registry.registry.canonicality, "predicate-level");
    assert_eq!(registry.registry.conceptarium_role, "observer-non-governing");
    assert_eq!(registry.registry.knowledge_scope, "predicate-level");
    assert_eq!(registry.registry.source_authority, "local-repository");

    require_contains(&registry.registry.policy.permitted, "existence");
    require_contains(
        &registry.registry.policy.permitted,
        "canonical-repository-identity",
    );
    require_contains(
        &registry.registry.policy.permitted,
        "coarse-domain-purpose",
    );

    for excluded in [
        "internal-artifact-inventory",
        "internal-object-state",
        "private-person-records",
        "repository-local-conclusions",
        "unpublished-files",
        "live-workflow-state",
    ] {
        require_contains(&registry.registry.policy.excluded_by_default, excluded);
    }
}

#[test]
fn current_tarium_ecology_has_unique_materialized_repository_entities() {
    let registry = load_registry();
    let mut ids = BTreeSet::new();
    let mut repositories = BTreeSet::new();

    for record in &registry.repositories {
        assert!(!record.id.trim().is_empty(), "repository id must not be empty");
        assert!(!record.name.trim().is_empty(), "repository name must not be empty");
        assert!(!record.purpose.trim().is_empty(), "repository purpose must not be empty");
        assert!(
            matches!(record.family_status.as_str(), "core" | "adjacent"),
            "invalid family_status {:?} for {}",
            record.family_status,
            record.id
        );
        assert!(!record.role.trim().is_empty(), "repository role must not be empty");
        assert!(
            ids.insert(record.id.clone()),
            "duplicate repository id {:?}",
            record.id
        );
        assert!(
            repositories.insert(record.repository.clone()),
            "duplicate repository identity {:?}",
            record.repository
        );
        assert!(
            record.materialization.starts_with("repositories/"),
            "repository materialization must live under repositories/: {:?}",
            record.materialization
        );
        assert_materialization(&root().join(&record.materialization));
    }

    let expected = BTreeSet::from([
        "conceptarium".to_string(),
        "politicarium".to_string(),
        "linguarium".to_string(),
        "salvatarium".to_string(),
        "dramatarium".to_string(),
        "projectarium".to_string(),
        "software-philosophy".to_string(),
    ]);

    assert!(
        expected.is_subset(&ids),
        "current Tarium ecology is missing expected repository entities: {:?}",
        expected.difference(&ids).collect::<Vec<_>>()
    );

    let conceptarium = registry
        .repositories
        .iter()
        .find(|record| record.id == "conceptarium")
        .expect("Conceptarium repository entity must exist");
    assert_eq!(conceptarium.role, "meta-repository");

    let software_philosophy = registry
        .repositories
        .iter()
        .find(|record| record.id == "software-philosophy")
        .expect("Software Philosophy repository entity must exist");
    assert_eq!(software_philosophy.family_status, "adjacent");
}

#[test]
fn tarium_register_and_procedure_are_load_bearing_meta_artifacts() {
    let procedure_path = root().join("meta/tarium-procedure.md");
    let register_path = root().join("meta/tarium-register.md");

    let procedure = fs::read_to_string(&procedure_path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", procedure_path.display()));
    let register = fs::read_to_string(&register_path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", register_path.display()));

    assert!(procedure.contains("No suppression of testimony. No immunity from audit."));
    assert!(procedure.contains("Full standing. No epistemic privilege."));
    assert!(procedure.contains("## Non-prejudgment"));
    assert!(procedure.contains("## Not moderation"));

    assert!(register.contains("Tarium Procedure"));
    assert!(register.contains("structure rather than swagger"));
    assert!(register.contains("## Scope markers"));
    assert!(register.contains("## Anti-patterns"));
}

#[test]
fn tarium_repository_schema_preserves_four_sovereign_layers() {
    let schema_path = root().join("meta/tarium-repository-schema.md");
    let schema = fs::read_to_string(&schema_path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", schema_path.display()));

    for required_heading in [
        "## 1. Objective ledger",
        "## 2. Ontological structure",
        "## 3. Integration and analysis",
        "## 4. Phenomenological / confessional layer",
        "# Layer sovereignty",
        "# Cross-layer relations",
        "# Anti-collapse constitution",
    ] {
        assert!(
            schema.contains(required_heading),
            "Tarium Repository Schema is missing load-bearing section {required_heading:?}"
        );
    }

    assert!(schema.contains("distinct, sovereign, and first class"));
    assert!(schema.contains("Integration is achieved by typed relations, not by collapsing provenance."));
    assert!(schema.contains("confession -> fact"));
    assert!(schema.contains("analysis -> evidence"));
    assert!(schema.contains("ontology -> proof"));
    assert!(schema.contains(
        "Preserve fact, structure, analysis, and experience as distinct sovereign layers"
    ));
}
