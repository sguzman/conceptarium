use crate::corpus::Corpus;
use anyhow::{Context, Result, bail};
use std::fs;
use std::path::{Path, PathBuf};
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{Schema, STORED, STRING, TEXT, Value};
use tantivy::{Index, TantivyDocument};

pub fn default_path(root: &Path) -> PathBuf {
    root.join(".conceptarium/tantivy")
}

pub fn build(corpus: &Corpus, index_path: &Path) -> Result<()> {
    if index_path.exists() {
        fs::remove_dir_all(index_path)
            .with_context(|| format!("removing old Tantivy index {}", index_path.display()))?;
    }
    fs::create_dir_all(index_path)
        .with_context(|| format!("creating Tantivy index directory {}", index_path.display()))?;

    let mut schema_builder = Schema::builder();
    let id = schema_builder.add_text_field("id", STRING | STORED);
    let term = schema_builder.add_text_field("term", TEXT | STORED);
    let aliases = schema_builder.add_text_field("aliases", TEXT);
    let gloss = schema_builder.add_text_field("gloss", TEXT | STORED);
    let problem_pressure = schema_builder.add_text_field("problem_pressure", TEXT | STORED);
    let body = schema_builder.add_text_field("body", TEXT);
    let capture = schema_builder.add_text_field("capture", TEXT);
    let domains = schema_builder.add_text_field("domains", TEXT);
    let kind = schema_builder.add_text_field("type", STRING | STORED);
    let status = schema_builder.add_text_field("status", STRING | STORED);
    let materialization = schema_builder.add_text_field("materialization", STRING | STORED);
    let ontology_state = schema_builder.add_text_field("ontology_state", STRING | STORED);
    let schema = schema_builder.build();

    let index = Index::create_in_dir(index_path, schema)?;
    let mut writer = index.writer(50_000_000)?;

    for record in &corpus.registry.concepts {
        let mut doc = TantivyDocument::default();
        doc.add_text(id, &record.id);
        doc.add_text(term, &record.term);
        doc.add_text(materialization, &record.materialization);
        doc.add_text(ontology_state, &record.ontology_state);

        if let Some(entry) = corpus.entry(&record.id) {
            doc.add_text(gloss, &entry.meta.gloss);
            if let Some(pressure) = &entry.problem_pressure {
                doc.add_text(problem_pressure, pressure);
            }
            doc.add_text(body, &entry.body);
            doc.add_text(kind, &entry.meta.kind);
            doc.add_text(status, &entry.meta.status);
            for alias in &entry.meta.aliases {
                doc.add_text(aliases, alias);
            }
            for domain in &entry.meta.domains {
                doc.add_text(domains, domain);
            }
        } else if let Some(meta) = &record.capture {
            if let Some(note) = &meta.note {
                doc.add_text(capture, note);
            }
            if let Some(context) = &meta.context {
                doc.add_text(capture, context);
            }
        }

        writer.add_document(doc)?;
    }

    writer.commit()?;
    println!(
        "Built Tantivy index with {} concepts at {}",
        corpus.registry.concepts.len(),
        index_path.display()
    );
    Ok(())
}

pub fn search(
    index_path: &Path,
    query_text: &str,
    field: Option<&str>,
    limit: usize,
) -> Result<()> {
    if !index_path.exists() {
        bail!(
            "Tantivy index does not exist at {}; run 'conceptarium index build' first",
            index_path.display()
        );
    }

    let index = Index::open_in_dir(index_path)
        .with_context(|| format!("opening Tantivy index {}", index_path.display()))?;
    let schema = index.schema();

    let id_field = schema.get_field("id")?;
    let term_field = schema.get_field("term")?;
    let gloss_field = schema.get_field("gloss")?;

    let default_fields = match field {
        None => vec![
            schema.get_field("term")?,
            schema.get_field("aliases")?,
            schema.get_field("gloss")?,
            schema.get_field("problem_pressure")?,
            schema.get_field("body")?,
            schema.get_field("capture")?,
            schema.get_field("domains")?,
        ],
        Some(name) => {
            let normalized = match name {
                "problem-pressure" | "pressure" => "problem_pressure",
                "type" | "kind" => "type",
                other => other,
            };
            vec![schema.get_field(normalized).with_context(|| {
                format!("unknown Tantivy field {name:?}")
            })?]
        }
    };

    let reader = index.reader()?;
    let searcher = reader.searcher();
    let parser = QueryParser::for_index(&index, default_fields);
    let query = parser
        .parse_query(query_text)
        .with_context(|| format!("parsing Tantivy query {query_text:?}"))?;
    let hits = searcher.search(&query, &TopDocs::with_limit(limit).order_by_score())?;

    for (score, address) in hits {
        let doc = searcher.doc::<TantivyDocument>(address)?;
        let id = doc
            .get_first(id_field)
            .and_then(|value| value.as_str())
            .unwrap_or("");
        let term = doc
            .get_first(term_field)
            .and_then(|value| value.as_str())
            .unwrap_or("");
        let gloss = doc
            .get_first(gloss_field)
            .and_then(|value| value.as_str())
            .unwrap_or("");
        println!("{score:.4}\t{id}\t{term}\t{gloss}");
    }

    Ok(())
}

pub fn exists(path: &Path) -> bool {
    path.join("meta.json").exists()
}
