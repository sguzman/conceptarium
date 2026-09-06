use crate::corpus::Corpus;
use anyhow::{Context, Result, bail};
use rusqlite::{Connection, OpenFlags, params, types::ValueRef};
use std::fs;
use std::path::{Path, PathBuf};

pub fn default_path(root: &Path) -> PathBuf {
    root.join(".conceptarium/conceptarium.sqlite")
}

pub fn build(corpus: &Corpus, path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("creating SQLite parent directory {}", parent.display()))?;
    }
    if path.exists() {
        fs::remove_file(path)
            .with_context(|| format!("removing old SQLite projection {}", path.display()))?;
    }

    let mut conn = Connection::open(path)
        .with_context(|| format!("opening SQLite projection {}", path.display()))?;

    conn.execute_batch(
        r#"
        PRAGMA foreign_keys = ON;

        CREATE TABLE concepts (
            id TEXT PRIMARY KEY,
            term TEXT NOT NULL,
            presence TEXT NOT NULL,
            materialization TEXT NOT NULL,
            ontology_state TEXT NOT NULL,
            type TEXT,
            status TEXT,
            gloss TEXT,
            source TEXT,
            origin_date TEXT,
            origin_authorship TEXT,
            origin_certainty TEXT,
            body TEXT,
            problem_pressure TEXT,
            open_questions TEXT,
            queue_group TEXT,
            capture_note TEXT,
            capture_context TEXT
        );

        CREATE TABLE domains (
            concept_id TEXT NOT NULL,
            domain TEXT NOT NULL,
            PRIMARY KEY (concept_id, domain),
            FOREIGN KEY (concept_id) REFERENCES concepts(id)
        );

        CREATE TABLE aliases (
            concept_id TEXT NOT NULL,
            alias TEXT NOT NULL,
            PRIMARY KEY (concept_id, alias),
            FOREIGN KEY (concept_id) REFERENCES concepts(id)
        );

        CREATE TABLE relations (
            source TEXT NOT NULL,
            predicate TEXT NOT NULL,
            target TEXT NOT NULL,
            PRIMARY KEY (source, predicate, target),
            FOREIGN KEY (source) REFERENCES concepts(id),
            FOREIGN KEY (target) REFERENCES concepts(id)
        );

        CREATE INDEX idx_concepts_term ON concepts(term);
        CREATE INDEX idx_concepts_type ON concepts(type);
        CREATE INDEX idx_concepts_status ON concepts(status);
        CREATE INDEX idx_concepts_materialization ON concepts(materialization);
        CREATE INDEX idx_concepts_ontology_state ON concepts(ontology_state);
        CREATE INDEX idx_domains_domain ON domains(domain);
        CREATE INDEX idx_relations_source ON relations(source);
        CREATE INDEX idx_relations_target ON relations(target);
        CREATE INDEX idx_relations_predicate ON relations(predicate);
        "#,
    )?;

    let tx = conn.transaction()?;
    {
        let mut insert_concept = tx.prepare(
            r#"
            INSERT INTO concepts (
                id, term, presence, materialization, ontology_state,
                type, status, gloss, source,
                origin_date, origin_authorship, origin_certainty,
                body, problem_pressure, open_questions,
                queue_group, capture_note, capture_context
            )
            VALUES (
                ?1, ?2, ?3, ?4, ?5,
                ?6, ?7, ?8, ?9,
                ?10, ?11, ?12,
                ?13, ?14, ?15,
                ?16, ?17, ?18
            )
            "#,
        )?;
        let mut insert_domain =
            tx.prepare("INSERT INTO domains (concept_id, domain) VALUES (?1, ?2)")?;
        let mut insert_alias =
            tx.prepare("INSERT INTO aliases (concept_id, alias) VALUES (?1, ?2)")?;
        let mut insert_relation = tx.prepare(
            "INSERT INTO relations (source, predicate, target) VALUES (?1, ?2, ?3)",
        )?;

        for record in &corpus.registry.concepts {
            let entry = corpus.entry(&record.id);
            let capture_note = record
                .capture
                .as_ref()
                .and_then(|capture| capture.note.as_deref());
            let capture_context = record
                .capture
                .as_ref()
                .and_then(|capture| capture.context.as_deref());

            insert_concept.execute(params![
                record.id,
                record.term,
                record.presence,
                record.materialization,
                record.ontology_state,
                entry.map(|entry| entry.meta.kind.as_str()),
                entry.map(|entry| entry.meta.status.as_str()),
                entry.map(|entry| entry.meta.gloss.as_str()),
                entry.map(|entry| entry.path.to_string_lossy().to_string()),
                entry.map(|entry| entry.meta.origin.date.as_str()),
                entry.map(|entry| entry.meta.origin.authorship.as_str()),
                entry.map(|entry| entry.meta.origin.certainty.as_str()),
                entry.map(|entry| entry.body.as_str()),
                entry.and_then(|entry| entry.problem_pressure.as_deref()),
                entry.and_then(|entry| entry.open_questions.as_deref()),
                record.queue_group.as_deref(),
                capture_note,
                capture_context,
            ])?;

            if let Some(entry) = entry {
                for domain in &entry.meta.domains {
                    insert_domain.execute(params![record.id, domain])?;
                }
                for alias in &entry.meta.aliases {
                    insert_alias.execute(params![record.id, alias])?;
                }
                for relation in &entry.meta.relations {
                    insert_relation.execute(params![record.id, relation.kind, relation.target])?;
                }
            }
        }
    }
    tx.commit()?;

    let relation_count = corpus
        .entries
        .iter()
        .map(|entry| entry.meta.relations.len())
        .sum::<usize>();

    println!(
        "Built SQLite projection with {} concepts and {} relations at {}",
        corpus.registry.concepts.len(),
        relation_count,
        path.display()
    );
    Ok(())
}

pub fn query(path: &Path, sql: &str) -> Result<()> {
    if !path.exists() {
        bail!(
            "SQLite projection does not exist at {}; run 'conceptarium sqlite build' first",
            path.display()
        );
    }

    let conn = Connection::open_with_flags(
        path,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_URI,
    )
    .with_context(|| format!("opening SQLite projection {} read-only", path.display()))?;

    let mut stmt = conn
        .prepare(sql)
        .with_context(|| format!("preparing SQLite query {sql:?}"))?;

    if stmt.column_count() == 0 {
        bail!("SQLite query must return rows; mutation statements are intentionally disabled");
    }

    let columns = stmt
        .column_names()
        .iter()
        .map(|name| name.to_string())
        .collect::<Vec<_>>();
    println!("{}", columns.join("\t"));

    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let mut values = Vec::with_capacity(columns.len());
        for index in 0..columns.len() {
            let value = match row.get_ref(index)? {
                ValueRef::Null => String::new(),
                ValueRef::Integer(value) => value.to_string(),
                ValueRef::Real(value) => value.to_string(),
                ValueRef::Text(value) => String::from_utf8_lossy(value).into_owned(),
                ValueRef::Blob(value) => format!("<blob:{}>", value.len()),
            };
            values.push(value.replace('\t', " ").replace('\n', "\\n"));
        }
        println!("{}", values.join("\t"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::default_path;
    use std::path::Path;

    #[test]
    fn default_sqlite_path_is_disposable() {
        assert_eq!(
            default_path(Path::new("/repo")),
            Path::new("/repo/.conceptarium/conceptarium.sqlite")
        );
    }
}
