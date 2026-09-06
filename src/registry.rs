use crate::model::{Capture, RegistryConcept, RegistryFile};
use anyhow::{Context, Result, bail};
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

fn registry_path(root: &Path) -> PathBuf {
    root.join("registry/concepts.yml")
}

pub fn load(root: &Path) -> Result<RegistryFile> {
    let path = registry_path(root);
    let text = fs::read_to_string(&path)
        .with_context(|| format!("reading {}", path.display()))?;
    serde_yaml::from_str(&text).with_context(|| format!("parsing {}", path.display()))
}

fn save(root: &Path, registry: &RegistryFile) -> Result<()> {
    let path = registry_path(root);
    let text = serde_yaml::to_string(registry)?;
    fs::write(&path, text).with_context(|| format!("writing {}", path.display()))
}

pub fn slugify(term: &str) -> String {
    let re = Regex::new(r"[^a-z0-9]+").expect("static regex");
    let lowered = term.trim().to_lowercase().replace('/', " ");
    re.replace_all(&lowered, "-").trim_matches('-').to_string()
}

#[derive(Debug)]
pub struct CaptureArgs {
    pub term: String,
    pub id: Option<String>,
    pub date: String,
    pub group: Option<String>,
    pub note: Option<String>,
    pub context: Option<String>,
    pub ontology_state: String,
}

pub fn capture(root: &Path, args: CaptureArgs) -> Result<()> {
    let mut registry = load(root)?;
    let id = args.id.unwrap_or_else(|| slugify(&args.term));
    if id.is_empty() {
        bail!("could not derive a stable id; provide --id");
    }

    if let Some(existing) = registry.concepts.iter().find(|record| record.id == id) {
        println!(
            "{} already registered as {:?} ({})",
            id, existing.term, existing.materialization
        );
        return Ok(());
    }

    let capture = if args.note.is_some() || args.context.is_some() {
        Some(Capture {
            note: args.note,
            context: args.context,
        })
    } else {
        None
    };

    registry.concepts.push(RegistryConcept {
        id: id.clone(),
        term: args.term.trim().to_string(),
        presence: "registered".to_string(),
        materialization: "registry-only".to_string(),
        entry: None,
        ontology_state: args.ontology_state,
        registered_on: Some(args.date),
        queue_group: args.group,
        capture,
    });
    registry.concepts.sort_by(|a, b| a.id.cmp(&b.id));
    save(root, &registry)?;
    println!("Registered {id}: {}", args.term.trim());
    Ok(())
}

pub fn materialize(root: &Path, id: &str, entry: &Path) -> Result<()> {
    if entry.is_absolute() {
        bail!("--entry must be repository-relative");
    }

    let mut registry = load(root)?;
    let Some(record) = registry.concepts.iter_mut().find(|record| record.id == id) else {
        bail!("{id:?} is not registered");
    };

    record.materialization = "entry".to_string();
    record.entry = Some(entry.to_string_lossy().replace('\\', "/"));
    if record.ontology_state == "unplaced" {
        record.ontology_state = "unassessed".to_string();
    }

    save(root, &registry)?;
    println!("Materialized {id} at {}", entry.display());
    Ok(())
}

pub fn queue(root: &Path, group: Option<&str>) -> Result<()> {
    let registry = load(root)?;
    let mut records = registry
        .concepts
        .iter()
        .filter(|record| record.materialization == "registry-only")
        .filter(|record| group.is_none_or(|wanted| record.queue_group.as_deref() == Some(wanted)))
        .collect::<Vec<_>>();

    records.sort_by_key(|record| {
        (
            record.queue_group.clone().unwrap_or_default(),
            record.term.to_lowercase(),
        )
    });

    let mut current: Option<&str> = None;
    for record in &records {
        let this_group = record.queue_group.as_deref().unwrap_or("Ungrouped");
        if current != Some(this_group) {
            if current.is_some() {
                println!();
            }
            println!("[{this_group}]");
            current = Some(this_group);
        }
        println!("- {} ({})", record.term, record.id);
    }
    println!("\n{} registry-only concept(s)", records.len());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::slugify;

    #[test]
    fn slugifies_terms() {
        assert_eq!(slugify("Models don’t confer title"), "models-don-t-confer-title");
        assert_eq!(slugify("Fallen / Village"), "fallen-village");
    }
}
