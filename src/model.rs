use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryFile {
    #[serde(default)]
    pub version: Option<u64>,
    #[serde(default)]
    pub registry: Option<serde_yaml::Value>,
    pub concepts: Vec<RegistryConcept>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryConcept {
    pub id: String,
    pub term: String,
    pub presence: String,
    pub materialization: String,
    #[serde(default)]
    pub entry: Option<String>,
    pub ontology_state: String,
    #[serde(default)]
    pub registered_on: Option<String>,
    #[serde(default)]
    pub queue_group: Option<String>,
    #[serde(default)]
    pub capture: Option<Capture>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Capture {
    #[serde(default)]
    pub note: Option<String>,
    #[serde(default)]
    pub context: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryMeta {
    pub id: String,
    pub term: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub status: String,
    pub gloss: String,
    pub domains: Vec<String>,
    #[serde(default)]
    pub aliases: Vec<String>,
    pub origin: Origin,
    #[serde(default)]
    pub relations: Vec<Relation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Origin {
    pub date: String,
    pub authorship: String,
    pub certainty: String,
    #[serde(default)]
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    #[serde(rename = "type")]
    pub kind: String,
    pub target: String,
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub path: PathBuf,
    pub meta: EntryMeta,
    pub body: String,
    pub problem_pressure: Option<String>,
    pub open_questions: Option<String>,
}
