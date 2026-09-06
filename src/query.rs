use crate::corpus::Corpus;
use anyhow::{Result, bail};
use petgraph::algo::astar;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Incoming,
    Outgoing,
    Both,
}

impl Direction {
    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "incoming" | "in" => Ok(Self::Incoming),
            "outgoing" | "out" => Ok(Self::Outgoing),
            "both" => Ok(Self::Both),
            _ => bail!("direction must be incoming, outgoing, or both"),
        }
    }
}

pub fn get(corpus: &Corpus, key: &str) -> Result<()> {
    let Some(record) = corpus.resolve(key) else {
        bail!("unknown concept {key:?}");
    };

    println!("{} ({})", record.term, record.id);
    println!("materialization: {}", record.materialization);
    println!("ontology_state: {}", record.ontology_state);

    if let Some(entry) = corpus.entry(&record.id) {
        println!("type: {}", entry.meta.kind);
        println!("status: {}", entry.meta.status);
        if !entry.meta.domains.is_empty() {
            println!("domains: {}", entry.meta.domains.join(", "));
        }
        println!("\n{}\n", entry.meta.gloss);
        println!("{}", entry.body);
    } else if let Some(capture) = &record.capture {
        if let Some(note) = &capture.note {
            println!("capture_note: {note}");
        }
        if let Some(context) = &capture.context {
            println!("context: {context}");
        }
    }
    Ok(())
}

pub fn list(
    corpus: &Corpus,
    domain: Option<&str>,
    kind: Option<&str>,
    status: Option<&str>,
    materialization: Option<&str>,
) {
    let mut rows = corpus.registry.concepts.iter().collect::<Vec<_>>();
    rows.sort_by_key(|record| record.term.to_lowercase());

    for record in rows {
        if materialization.is_some_and(|wanted| record.materialization != wanted) {
            continue;
        }

        let entry = corpus.entry(&record.id);
        if domain.is_some_and(|wanted| {
            entry.is_none_or(|entry| {
                !entry
                    .meta
                    .domains
                    .iter()
                    .any(|value| value.eq_ignore_ascii_case(wanted))
            })
        }) {
            continue;
        }
        if kind.is_some_and(|wanted| entry.is_none_or(|entry| entry.meta.kind != wanted)) {
            continue;
        }
        if status.is_some_and(|wanted| entry.is_none_or(|entry| entry.meta.status != wanted)) {
            continue;
        }

        let suffix = entry
            .map(|entry| format!(" [{} / {}]", entry.meta.kind, entry.meta.status))
            .unwrap_or_else(|| " [registry-only]".to_string());
        println!("{}\t{}{}", record.id, record.term, suffix);
    }
}

pub fn search(corpus: &Corpus, query: &str, limit: usize) {
    let query = query.to_lowercase();
    let mut scored = Vec::new();

    for record in &corpus.registry.concepts {
        let mut score = 0u32;
        let term = record.term.to_lowercase();
        if term == query {
            score += 100;
        } else if term.contains(&query) {
            score += 40;
        }
        if record.id.to_lowercase().contains(&query) {
            score += 20;
        }

        if let Some(entry) = corpus.entry(&record.id) {
            if entry.meta.gloss.to_lowercase().contains(&query) {
                score += 12;
            }
            if entry
                .problem_pressure
                .as_deref()
                .unwrap_or_default()
                .to_lowercase()
                .contains(&query)
            {
                score += 10;
            }
            if entry.body.to_lowercase().contains(&query) {
                score += 4;
            }
        } else if let Some(capture) = &record.capture {
            if capture
                .note
                .as_deref()
                .unwrap_or_default()
                .to_lowercase()
                .contains(&query)
            {
                score += 8;
            }
            if capture
                .context
                .as_deref()
                .unwrap_or_default()
                .to_lowercase()
                .contains(&query)
            {
                score += 6;
            }
        }

        if score > 0 {
            scored.push((score, record));
        }
    }

    scored.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.term.cmp(&b.1.term)));
    for (score, record) in scored.into_iter().take(limit) {
        let gloss = corpus
            .entry(&record.id)
            .map(|entry| entry.meta.gloss.as_str())
            .or_else(|| record.capture.as_ref()?.note.as_deref())
            .unwrap_or("");
        println!("{score:>3}\t{}\t{}\t{}", record.id, record.term, gloss);
    }
}

pub fn relations(
    corpus: &Corpus,
    key: &str,
    direction: Direction,
    relation_filter: Option<&str>,
) -> Result<()> {
    let Some(record) = corpus.resolve(key) else {
        bail!("unknown concept {key:?}");
    };
    let id = record.id.as_str();

    if matches!(direction, Direction::Outgoing | Direction::Both) {
        for entry in &corpus.entries {
            if entry.meta.id != id {
                continue;
            }
            for relation in &entry.meta.relations {
                if relation_filter.is_some_and(|wanted| relation.kind != wanted) {
                    continue;
                }
                let target = corpus
                    .resolve(&relation.target)
                    .map(|x| x.term.as_str())
                    .unwrap_or("?");
                println!("OUT\t{}\t{}\t{}\t{}", id, relation.kind, relation.target, target);
            }
        }
    }

    if matches!(direction, Direction::Incoming | Direction::Both) {
        for entry in &corpus.entries {
            for relation in &entry.meta.relations {
                if relation.target != id {
                    continue;
                }
                if relation_filter.is_some_and(|wanted| relation.kind != wanted) {
                    continue;
                }
                println!(
                    "IN\t{}\t{}\t{}\t{}",
                    entry.meta.id, relation.kind, id, entry.meta.term
                );
            }
        }
    }
    Ok(())
}

pub fn path(corpus: &Corpus, from: &str, to: &str) -> Result<()> {
    let Some(from_record) = corpus.resolve(from) else {
        bail!("unknown concept {from:?}");
    };
    let Some(to_record) = corpus.resolve(to) else {
        bail!("unknown concept {to:?}");
    };

    let mut graph = DiGraph::<String, String>::new();
    let mut index: BTreeMap<String, NodeIndex> = BTreeMap::new();

    for record in &corpus.registry.concepts {
        let node = graph.add_node(record.id.clone());
        index.insert(record.id.clone(), node);
    }

    for entry in &corpus.entries {
        let Some(&source) = index.get(&entry.meta.id) else {
            continue;
        };
        for relation in &entry.meta.relations {
            if let Some(&target) = index.get(&relation.target) {
                graph.add_edge(source, target, relation.kind.clone());
            }
        }
    }

    let start = index[&from_record.id];
    let goal = index[&to_record.id];
    let Some((_cost, nodes)) = astar(
        &graph,
        start,
        |node| node == goal,
        |_| 1usize,
        |_| 0usize,
    ) else {
        bail!("no directed relation path from {} to {}", from_record.id, to_record.id);
    };

    for pair in nodes.windows(2) {
        let source = pair[0];
        let target = pair[1];
        let edge = graph
            .edges(source)
            .find(|edge| edge.target() == target)
            .expect("path edge exists");
        println!(
            "{} --{}--> {}",
            graph[source],
            edge.weight(),
            graph[target]
        );
    }
    Ok(())
}

pub fn backends() {
    println!("backend\tstatus\trole");
    println!("memory/petgraph\tACTIVE\tcanonical local structural query engine");
    println!("tantivy\tACTIVE\tBM25/full-text index over semantic fields");
    println!("sqlite\tACTIVE\trelational/interop projection");
    println!("surrealdb\tNEXT\tunified document+graph+vector experimental projection");
    println!("oxigraph\tPLANNED\tRDF/SPARQL ontology projection");
    println!("qdrant-edge\tPLANNED\tdense/sparse semantic geometry projection");
}
