use crate::corpus::Corpus;
use anyhow::{Context, Result, bail};
use std::fs;
use std::path::{Path, PathBuf};
use surrealdb::Surreal;
use surrealdb::engine::local::SurrealKv;
use surrealdb::types::Value;

const NAMESPACE: &str = "conceptarium";
const DATABASE: &str = "main";

pub fn default_path(root: &Path) -> PathBuf {
    root.join(".conceptarium/surreal")
}

async fn connect(path: &Path) -> Result<Surreal<surrealdb::engine::local::Db>> {
    let endpoint = path.to_string_lossy().to_string();
    let db = Surreal::new::<SurrealKv>(endpoint.as_str())
        .await
        .with_context(|| format!("opening embedded SurrealKV store {}", path.display()))?;
    db.use_ns(NAMESPACE)
        .use_db(DATABASE)
        .await
        .context("selecting Conceptarium SurrealDB namespace/database")?;
    Ok(db)
}

pub async fn build(corpus: &Corpus, path: &Path) -> Result<()> {
    if path.exists() {
        fs::remove_dir_all(path)
            .with_context(|| format!("removing old SurrealDB projection {}", path.display()))?;
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("creating SurrealDB parent directory {}", parent.display()))?;
    }

    let db = connect(path).await?;

    for record in &corpus.registry.concepts {
        let entry = corpus.entry(&record.id);

        let kind = entry.map(|entry| entry.meta.kind.clone());
        let status = entry.map(|entry| entry.meta.status.clone());
        let gloss = entry.map(|entry| entry.meta.gloss.clone());
        let source = entry.map(|entry| entry.path.to_string_lossy().replace('\\', "/"));
        let body = entry.map(|entry| entry.body.clone());
        let problem_pressure = entry.and_then(|entry| entry.problem_pressure.clone());
        let open_questions = entry.and_then(|entry| entry.open_questions.clone());
        let domains = entry
            .map(|entry| entry.meta.domains.clone())
            .unwrap_or_default();
        let aliases = entry
            .map(|entry| entry.meta.aliases.clone())
            .unwrap_or_default();
        let origin_date = entry.map(|entry| entry.meta.origin.date.clone());
        let origin_authorship = entry.map(|entry| entry.meta.origin.authorship.clone());
        let origin_certainty = entry.map(|entry| entry.meta.origin.certainty.clone());
        let capture_note = record
            .capture
            .as_ref()
            .and_then(|capture| capture.note.clone());
        let capture_context = record
            .capture
            .as_ref()
            .and_then(|capture| capture.context.clone());

        db.query(
            r#"
            UPSERT type::record('concept', $id)
            SET
                concept_id = $id,
                term = $term,
                presence = $presence,
                materialization = $materialization,
                ontology_state = $ontology_state,
                kind = $kind,
                status = $status,
                gloss = $gloss,
                domains = $domains,
                aliases = $aliases,
                source = $source,
                body = $body,
                problem_pressure = $problem_pressure,
                open_questions = $open_questions,
                origin_date = $origin_date,
                origin_authorship = $origin_authorship,
                origin_certainty = $origin_certainty,
                queue_group = $queue_group,
                capture_note = $capture_note,
                capture_context = $capture_context;
            "#,
        )
        .bind(("id", record.id.clone()))
        .bind(("term", record.term.clone()))
        .bind(("presence", record.presence.clone()))
        .bind(("materialization", record.materialization.clone()))
        .bind(("ontology_state", record.ontology_state.clone()))
        .bind(("kind", kind))
        .bind(("status", status))
        .bind(("gloss", gloss))
        .bind(("domains", domains))
        .bind(("aliases", aliases))
        .bind(("source", source))
        .bind(("body", body))
        .bind(("problem_pressure", problem_pressure))
        .bind(("open_questions", open_questions))
        .bind(("origin_date", origin_date))
        .bind(("origin_authorship", origin_authorship))
        .bind(("origin_certainty", origin_certainty))
        .bind(("queue_group", record.queue_group.clone()))
        .bind(("capture_note", capture_note))
        .bind(("capture_context", capture_context))
        .await
        .with_context(|| format!("projecting concept {} into SurrealDB", record.id))?
        .check()
        .with_context(|| format!("checking SurrealDB statement for concept {}", record.id))?;
    }

    let mut relation_count = 0usize;
    for entry in &corpus.entries {
        for relation in &entry.meta.relations {
            db.query(
                r#"
                RELATE
                    type::record('concept', $source)
                    ->relation->
                    type::record('concept', $target)
                SET
                    predicate = $predicate,
                    source_id = $source,
                    target_id = $target;
                "#,
            )
            .bind(("source", entry.meta.id.clone()))
            .bind(("target", relation.target.clone()))
            .bind(("predicate", relation.kind.clone()))
            .await
            .with_context(|| {
                format!(
                    "projecting relation {} --{}--> {} into SurrealDB",
                    entry.meta.id, relation.kind, relation.target
                )
            })?
            .check()
            .with_context(|| {
                format!(
                    "checking SurrealDB statement for relation {} --{}--> {}",
                    entry.meta.id, relation.kind, relation.target
                )
            })?;
            relation_count += 1;
        }
    }

    let actual_concepts = count_table(&db, "concept").await?;
    let actual_relations = count_table(&db, "relation").await?;

    if actual_concepts != corpus.registry.concepts.len() {
        bail!(
            "SurrealDB concept count mismatch: expected {}, found {}",
            corpus.registry.concepts.len(),
            actual_concepts
        );
    }
    if actual_relations != relation_count {
        bail!(
            "SurrealDB relation count mismatch: expected {}, found {}",
            relation_count,
            actual_relations
        );
    }

    println!(
        "Built SurrealDB projection with {} concepts and {} relations at {}",
        actual_concepts,
        actual_relations,
        path.display()
    );
    Ok(())
}

async fn count_table(
    db: &Surreal<surrealdb::engine::local::Db>,
    table: &str,
) -> Result<usize> {
    let query = format!("SELECT count() AS count FROM {table} GROUP ALL");
    let mut response = db
        .query(&query)
        .await
        .with_context(|| format!("counting SurrealDB table {table}"))?
        .check()
        .with_context(|| format!("checking SurrealDB count query for {table}"))?;
    let value: Value = response
        .take(0)
        .with_context(|| format!("reading SurrealDB count result for {table}"))?;
    let json = value.into_json_value();
    let count = json
        .as_array()
        .and_then(|rows| rows.first())
        .and_then(|row| row.get("count"))
        .and_then(|count| count.as_u64())
        .with_context(|| format!("unexpected SurrealDB count shape for {table}: {json}"))?;
    Ok(count as usize)
}

fn validate_read_only(query: &str) -> Result<&str> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        bail!("SurrealQL query must not be empty");
    }

    let without_trailing = trimmed.trim_end_matches(';').trim();
    if without_trailing.contains(';') {
        bail!("SurrealDB query accepts one read-only statement at a time");
    }

    let lowered = without_trailing.to_ascii_lowercase();
    let allowed = lowered.starts_with("select ")
        || lowered.starts_with("return ")
        || lowered.starts_with("info ");

    if !allowed {
        bail!(
            "SurrealDB query is read-only; statement must begin with SELECT, RETURN, or INFO"
        );
    }

    Ok(without_trailing)
}

pub async fn query(path: &Path, query: &str) -> Result<()> {
    if !path.exists() {
        bail!(
            "SurrealDB projection does not exist at {}; run 'conceptarium surreal build' first",
            path.display()
        );
    }

    let query = validate_read_only(query)?;
    let db = connect(path).await?;
    let mut response = db
        .query(query)
        .await
        .with_context(|| format!("executing SurrealQL query {query:?}"))?;

    let statements = response.num_statements();
    for index in 0..statements {
        let value: Value = response
            .take(index)
            .with_context(|| format!("reading SurrealQL statement result {index}"))?;
        let json = value.into_json_value();
        println!("{}", serde_json::to_string_pretty(&json)?);
    }

    Ok(())
}

pub fn exists(path: &Path) -> bool {
    path.exists()
}

#[cfg(test)]
mod tests {
    use super::{default_path, validate_read_only};
    use std::path::Path;

    #[test]
    fn default_surreal_path_is_disposable() {
        assert_eq!(
            default_path(Path::new("/repo")),
            Path::new("/repo/.conceptarium/surreal")
        );
    }

    #[test]
    fn read_only_gate_accepts_queries() {
        assert!(validate_read_only("SELECT * FROM concept").is_ok());
        assert!(validate_read_only("RETURN 1;").is_ok());
        assert!(validate_read_only("INFO FOR DB").is_ok());
    }

    #[test]
    fn read_only_gate_rejects_mutation_and_batches() {
        assert!(validate_read_only("DELETE concept").is_err());
        assert!(validate_read_only("CREATE concept:test").is_err());
        assert!(validate_read_only("SELECT * FROM concept; DELETE concept").is_err());
    }
}
