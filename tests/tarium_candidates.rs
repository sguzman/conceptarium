use serde::Deserialize;
use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct CandidateRegistry {
    version: u64,
    registry: CandidateRegistryMeta,
    candidates: Vec<CandidateRecord>,
}

#[derive(Debug, Deserialize)]
struct CandidateRegistryMeta {
    object_kind: String,
    canonicality: String,
    authority: String,
    lifecycle_states: Vec<String>,
    materialization_states: Vec<String>,
    rules: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct CandidateRecord {
    id: String,
    lifecycle_state: String,
    materialization_state: String,
    sovereignty_assessment: String,
    repository: Option<String>,
    stub: String,
    host_candidate: Option<String>,
    disposition_target: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RepositoryRegistry {
    repositories: Vec<RepositoryRecord>,
}

#[derive(Debug, Deserialize)]
struct RepositoryRecord {
    id: String,
    repository: String,
    purpose: String,
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn load_candidates() -> CandidateRegistry {
    let path = root().join("registry/tarium-candidates.yml");
    let text = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
    serde_yaml::from_str(&text)
        .unwrap_or_else(|error| panic!("failed to parse {}: {error}", path.display()))
}

#[test]
fn tarium_candidate_registry_has_constitutional_lifecycle() {
    let registry = load_candidates();

    assert_eq!(registry.version, 1);
    assert_eq!(registry.registry.object_kind, "tarium-candidate");
    assert_eq!(registry.registry.canonicality, "lifecycle-and-sovereignty");
    assert_eq!(registry.registry.authority, "conceptarium-pre-sovereignty");

    let lifecycle = registry
        .registry
        .lifecycle_states
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    for state in [
        "candidate",
        "incubating",
        "approved",
        "instantiated",
        "active",
        "merged",
        "rejected",
        "dormant",
    ] {
        assert!(lifecycle.contains(state), "missing lifecycle state {state:?}");
    }

    let materialization = registry
        .registry
        .materialization_states
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    for state in ["stub", "province", "repository"] {
        assert!(
            materialization.contains(state),
            "missing materialization state {state:?}"
        );
    }

    for rule in [
        "intensity-admits-but-does-not-grant-sovereignty",
        "sovereignty-requires-ontological-independence",
        "stub-creation-is-cheap",
        "repository-creation-is-expensive",
        "merged-and-rejected-candidates-remain-knowledge",
        "canonical-ownership-follows-deepest-ontology",
    ] {
        assert!(
            registry.registry.rules.iter().any(|value| value == rule),
            "missing candidate constitutional rule {rule:?}"
        );
    }
}

#[test]
fn eleven_approved_future_tariums_are_stubs_not_repositories() {
    let registry = load_candidates();
    let expected = BTreeSet::from([
        "mathematics",
        "engineering",
        "design",
        "habitat",
        "architecture",
        "economics",
        "biotarium",
        "geographia",
        "chemistry",
        "software-studies",
        "cuisine",
    ]);

    let approved = registry
        .candidates
        .iter()
        .filter(|candidate| candidate.lifecycle_state == "approved")
        .map(|candidate| candidate.id.as_str())
        .collect::<BTreeSet<_>>();

    assert_eq!(approved, expected);

    for candidate in registry
        .candidates
        .iter()
        .filter(|candidate| candidate.lifecycle_state == "approved")
    {
        assert_eq!(candidate.materialization_state, "stub");
        assert!(candidate.repository.is_none());
        assert_eq!(candidate.sovereignty_assessment, if candidate.id == "habitat" { "passed-in-principle-after-reframing" } else { "passed-in-principle" });
        assert!(root().join(&candidate.stub).exists(), "missing candidate stub {}", candidate.stub);
    }
}

#[test]
fn provinces_and_merged_candidates_preserve_negative_knowledge() {
    let registry = load_candidates();

    let incubating = registry
        .candidates
        .iter()
        .filter(|candidate| candidate.lifecycle_state == "incubating")
        .map(|candidate| candidate.id.as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(incubating, BTreeSet::from(["finance", "sartorial", "typography"]));

    for candidate in registry
        .candidates
        .iter()
        .filter(|candidate| candidate.lifecycle_state == "incubating")
    {
        assert_eq!(candidate.materialization_state, "province");
        assert!(candidate.host_candidate.is_some());
        assert!(candidate.repository.is_none());
        assert!(root().join(&candidate.stub).exists());
    }

    let merged = registry
        .candidates
        .iter()
        .filter(|candidate| candidate.lifecycle_state == "merged")
        .map(|candidate| candidate.id.as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        merged,
        BTreeSet::from(["animal-biology", "animalia", "human-biology", "plants"])
    );

    for candidate in registry
        .candidates
        .iter()
        .filter(|candidate| candidate.lifecycle_state == "merged")
    {
        assert!(candidate.disposition_target.is_some());
        assert!(root().join(&candidate.stub).exists());
    }
}

#[test]
fn sovereignty_document_preserves_promotion_threshold() {
    let path = root().join("meta/tarium-sovereignty-and-lifecycle.md");
    let text = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));

    for required in [
        "# The Tarium Sovereignty Test",
        "# Two independent axes",
        "candidate\n    ↓\nincubating\n    ↓\napproved\n    ↓\ninstantiated\n    ↓\nactive",
        "Creating the stub is cheap; creating the repository is expensive.",
        "Intensity admits a subject to consideration. It does not grant repository sovereignty.",
        "Merged and rejected candidates are knowledge",
        "# Incubation and provinces",
        "# Secession without duplication",
    ] {
        assert!(text.contains(required), "missing lifecycle invariant {required:?}");
    }
}

#[test]
fn somatarium_is_registered_as_active_embodied_tarium() {
    let path = root().join("registry/repositories.yml");
    let text = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
    let registry: RepositoryRegistry = serde_yaml::from_str(&text)
        .unwrap_or_else(|error| panic!("failed to parse {}: {error}", path.display()));

    let somatarium = registry
        .repositories
        .iter()
        .find(|record| record.id == "somatarium")
        .expect("Somatarium repository entity must exist");

    assert_eq!(somatarium.repository, "sguzman/somatarium");
    assert!(somatarium.purpose.contains("phenomenology"));
    assert!(root().join("repositories/somatarium.md").exists());

    let materialized = fs::read_to_string(root().join("repositories/somatarium.md"))
        .expect("Somatarium materialization must be readable");
    assert!(materialized.contains("Phenomenology is first-class"));
    assert!(materialized.contains("## Boundary with Salvatarium"));
    assert!(materialized.contains("## Boundary with Biotarium candidate"));
}
