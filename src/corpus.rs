use crate::model::{Entry, EntryMeta, RegistryFile};
use anyhow::{Context, Result, bail};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Corpus {
    pub root: PathBuf,
    pub registry: RegistryFile,
    pub entries: Vec<Entry>,
}

impl Corpus {
    pub fn load(root: impl AsRef<Path>) -> Result<Self> {
        let root = root.as_ref().to_path_buf();
        let registry_path = root.join("registry/concepts.yml");
        let registry_text = fs::read_to_string(&registry_path)
            .with_context(|| format!("reading {}", registry_path.display()))?;
        let registry: RegistryFile = serde_yaml::from_str(&registry_text)
            .with_context(|| format!("parsing {}", registry_path.display()))?;

        let entries_dir = root.join("entries");
        let mut paths = fs::read_dir(&entries_dir)
            .with_context(|| format!("reading {}", entries_dir.display()))?
            .filter_map(|item| item.ok().map(|x| x.path()))
            .filter(|path| {
                path.extension().and_then(|x| x.to_str()) == Some("md")
                    && path.file_name().and_then(|x| x.to_str()) != Some("_template.md")
            })
            .collect::<Vec<_>>();
        paths.sort();

        let mut entries = Vec::with_capacity(paths.len());
        for path in paths {
            entries.push(read_entry(&root, &path)?);
        }

        Ok(Self {
            root,
            registry,
            entries,
        })
    }

    pub fn registry_by_id(&self) -> BTreeMap<&str, &crate::model::RegistryConcept> {
        self.registry
            .concepts
            .iter()
            .map(|record| (record.id.as_str(), record))
            .collect()
    }

    pub fn entries_by_id(&self) -> BTreeMap<&str, &Entry> {
        self.entries
            .iter()
            .map(|entry| (entry.meta.id.as_str(), entry))
            .collect()
    }

    pub fn resolve(&self, key: &str) -> Option<&crate::model::RegistryConcept> {
        self.registry.concepts.iter().find(|record| {
            record.id == key || record.term.eq_ignore_ascii_case(key)
        })
    }

    pub fn entry(&self, id: &str) -> Option<&Entry> {
        self.entries.iter().find(|entry| entry.meta.id == id)
    }
}

pub fn read_entry(root: &Path, path: &Path) -> Result<Entry> {
    let raw = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let lines = raw.lines().collect::<Vec<_>>();

    if lines.first().map(|x| x.trim()) != Some("---") {
        bail!("{} does not begin with YAML frontmatter", path.display());
    }

    let Some(end) = lines.iter().enumerate().skip(1).find_map(|(i, line)| {
        (line.trim() == "---").then_some(i)
    }) else {
        bail!("{} has no closing frontmatter delimiter", path.display());
    };

    let meta_text = lines[1..end].join("\n");
    let meta: EntryMeta = serde_yaml::from_str(&meta_text)
        .with_context(|| format!("parsing frontmatter in {}", path.display()))?;
    let body = lines[end + 1..].join("\n").trim().to_string();

    Ok(Entry {
        path: path
            .strip_prefix(root)
            .unwrap_or(path)
            .to_path_buf(),
        problem_pressure: section(&body, "Problem pressure"),
        open_questions: section(&body, "Open questions"),
        meta,
        body,
    })
}

pub fn section(body: &str, heading: &str) -> Option<String> {
    let wanted = format!("## {heading}");
    let mut capture = false;
    let mut lines = Vec::new();

    for line in body.lines() {
        if line.trim() == wanted {
            capture = true;
            continue;
        }
        if capture && line.starts_with("## ") {
            break;
        }
        if capture {
            lines.push(line);
        }
    }

    let text = lines.join("\n").trim().to_string();
    (!text.is_empty()).then_some(text)
}
