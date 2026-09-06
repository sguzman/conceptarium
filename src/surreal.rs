use crate::corpus::Corpus;
use anyhow::{Context, Result, bail};
use serde::Serialize;
use serde_json::Value;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const NAMESPACE: &str = "conceptarium";
const DATABASE: &str = "main";

pub fn default_path(root: &Path) -> PathBuf {
    root.join(".conceptarium/surreal")
}

fn surreal_bin() -> OsString {
    env::var_os("CONCEPTARIUM_SURREAL_BIN").unwrap_or_else(|| OsString::from("surreal"))
}

fn endpoint(path: &Path) -> String {
    let normalized = path.to_string_lossy().replace('\\', "/");

    if path.is_absolute() {
        if cfg!(windows) {
            format!("surrealkv:///{normalized}")
        } else {
            format!("surrealkv://{normalized}")
        }
    } else {
        format!(
            "surrealkv://{}",
            normalized.trim_start_matches("./")
        )
    }
}

fn ensure_cli() -> Result<()> {
    let bin = surreal_bin();
    let output = Command::new(&bin).arg("version").output().with_context(|| {
        format!(
            "could not execute {:?}. Install the official SurrealDB CLI or set CONCEPTARIUM_SURREAL_BIN to its path",
            bin
        )
    })?;

    if !output.status.success() {
        bail!(
            "SurrealDB CLI {:?} exists but 'surreal version' failed: {}",
            bin,
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }

    Ok(())
}

fn run_sql(path: &Path, sql: &str, json: bool) -> Result<String> {
    ensure_cli()?;

    let bin = surreal_bin();
    let endpoint = endpoint(path);
    let mut command = Command::new(&bin);
    command
        .arg("sql")
        .arg("--endpoint")
        .arg(&endpoint)
        .arg("--namespace")
        .arg(NAMESPACE)
        .arg("--database")
        .arg(DATABASE)
        .arg("--hide-welcome")
        .arg("--log")
        .arg("error")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    if json {
        command.arg("--json");
    }

    let mut child = command
        .spawn()
        .with_context(|| format!("starting SurrealDB CLI {:?} against {endpoint}", bin))?;

    child
        .stdin
        .take()
        .context("opening SurrealDB CLI stdin")?
        .write_all(sql.as_bytes())
        .context("sending SurrealQL to SurrealDB CLI")?;

    let output = child
        .wait_with_output()
        .context("waiting for SurrealDB CLI")?;

    if !output.status.success() {
        bail!(
            "SurrealDB CLI query failed against {endpoint}: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn literal<T: Serialize>(value: &T) -> Result<String> {
    Ok(serde_json::to_string(value)?)
}

fn option_literal<T: Serialize>(value: Option<&T>) -> Result<String> {
    match value {
        Some(value) => literal(value),
        None => Ok("null".to_string()),
    }
}

fn build_script(corpus: &Corpus) -> Result<String> {
    let mut script = String::new();

    for record in &corpus.registry.concepts {
        let entry = corpus.entry(&record.id);
        let kind = entry.map(|entry| &entry.meta.kind);
        let status = entry.map(|entry| &entry.meta.status);
        let gloss = entry.map(|entry| &entry.meta.gloss);
        let source = entry.map(|entry| entry.path.to_string_lossy().replace('\\', "/"));
        let body = entry.map(|entry| &entry.body);
        let problem_pressure = entry.and_then(|entry| entry.problem_pressure.as_ref());
        let open_questions = entry.and_then(|entry| entry.open_questions.as_ref());
        let domains = entry
            .map(|entry| &entry.meta.domains)
            .cloned()
            .unwrap_or_default();
        let aliases = entry
            .map(|entry| &entry.meta.aliases)
            .cloned()
            .unwrap_or_default();
        let origin_date = entry.map(|entry| &entry.meta.origin.date);
        let origin_authorship = entry.map(|entry| &entry.meta.origin.authorship);
        let origin_certainty = entry.map(|entry| &entry.meta.origin.certainty);
        let capture_note = record
            .capture
            .as_ref()
            .and_then(|capture| capture.note.as_ref());
        let capture_context = record
            .capture
            .as_ref()
            .and_then(|capture| capture.context.as_ref());

        script.push_str(&format!(
            "UPSERT type::record(\"concept\", {}) SET \
concept_id = {}, term = {}, presence = {}, materialization = {}, ontology_state = {}, \
kind = {}, status = {}, gloss = {}, domains = {}, aliases = {}, source = {}, body = {}, \
problem_pressure = {}, open_questions = {}, origin_date = {}, origin_authorship = {}, \
origin_certainty = {}, queue_group = {}, capture_note = {}, capture_context = {};\n",
            literal(&record.id)?,
            literal(&record.id)?,
            literal(&record.term)?,
            literal(&record.presence)?,
            literal(&record.materialization)?,
            literal(&record.ontology_state)?,
            option_literal(kind)?,
            option_literal(status)?,
            option_literal(gloss)?,
            literal(&domains)?,
            literal(&aliases)?,
            option_literal(source.as_ref())?,
            option_literal(body)?,
            option_literal(problem_pressure)?,
            option_literal(open_questions)?,
            option_literal(origin_date)?,
            option_literal(origin_authorship)?,
            option_literal(origin_certainty)?,
            option_literal(record.queue_group.as_ref())?,
            option_literal(capture_note)?,
            option_literal(capture_context)?,
        ));
    }

    for (index, entry) in corpus.entries.iter().enumerate() {
        for (rel_index, relation) in entry.meta.relations.iter().enumerate() {
            let suffix = format!("{index}_{rel_index}");
            script.push_str(&format!(
                "LET $source_{suffix} = type::record(\"concept\", {});\n\
LET $target_{suffix} = type::record(\"concept\", {});\n\
RELATE $source_{suffix}->relation->$target_{suffix} SET predicate = {}, source_id = {}, target_id = {};\n",
                literal(&entry.meta.id)?,
                literal(&relation.target)?,
                literal(&relation.kind)?,
                literal(&entry.meta.id)?,
                literal(&relation.target)?,
            ));
        }
    }

    Ok(script)
}

fn find_named_u64(value: &Value, key: &str) -> Option<u64> {
    match value {
        Value::Object(map) => {
            if let Some(number) = map.get(key).and_then(Value::as_u64) {
                return Some(number);
            }
            map.values().find_map(|value| find_named_u64(value, key))
        }
        Value::Array(values) => values
            .iter()
            .find_map(|value| find_named_u64(value, key)),
        _ => None,
    }
}

fn count_table(path: &Path, table: &str, field: &str) -> Result<usize> {
    let sql = format!("SELECT count() AS {field} FROM {table} GROUP ALL;");
    let output = run_sql(path, &sql, true)?;
    let json: Value = serde_json::from_str(output.trim()).with_context(|| {
        format!("parsing JSON returned by SurrealDB count query: {output:?}")
    })?;
    let count = find_named_u64(&json, field).with_context(|| {
        format!("SurrealDB count result did not contain {field:?}: {json}")
    })?;
    Ok(count as usize)
}

pub fn build(corpus: &Corpus, path: &Path) -> Result<()> {
    if path.exists() {
        fs::remove_dir_all(path)
            .with_context(|| format!("removing old SurrealDB projection {}", path.display()))?;
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("creating SurrealDB parent directory {}", parent.display()))?;
    }

    let script = build_script(corpus)?;
    run_sql(path, &script, false)?;

    let expected_relations = corpus
        .entries
        .iter()
        .map(|entry| entry.meta.relations.len())
        .sum::<usize>();
    let actual_concepts = count_table(path, "concept", "concepts")?;
    let actual_relations = count_table(path, "relation", "relations")?;

    if actual_concepts != corpus.registry.concepts.len() {
        bail!(
            "SurrealDB concept count mismatch: expected {}, found {}",
            corpus.registry.concepts.len(),
            actual_concepts
        );
    }
    if actual_relations != expected_relations {
        bail!(
            "SurrealDB relation count mismatch: expected {}, found {}",
            expected_relations,
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
        bail!("SurrealDB query is read-only; statement must begin with SELECT, RETURN, or INFO");
    }

    Ok(without_trailing)
}

pub fn query(path: &Path, query: &str) -> Result<()> {
    if !path.exists() {
        bail!(
            "SurrealDB projection does not exist at {}; run 'conceptarium surreal build' first",
            path.display()
        );
    }

    let query = validate_read_only(query)?;
    let output = run_sql(path, &format!("{query};"), true)?;
    print!("{output}");
    if !output.ends_with('\n') {
        println!();
    }
    Ok(())
}

pub fn exists(path: &Path) -> bool {
    path.exists()
}

#[cfg(test)]
mod tests {
    use super::{default_path, endpoint, validate_read_only};
    use std::path::Path;

    #[test]
    fn default_surreal_path_is_disposable() {
        assert_eq!(
            default_path(Path::new("/repo")),
            Path::new("/repo/.conceptarium/surreal")
        );
    }

    #[test]
    fn relative_endpoint_uses_embedded_surreal_kv() {
        assert_eq!(
            endpoint(Path::new(".conceptarium/surreal")),
            "surrealkv://.conceptarium/surreal"
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
