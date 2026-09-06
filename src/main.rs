use anyhow::Result;
use clap::{Parser, Subcommand};
use conceptarium::{corpus::Corpus, project, query, registry, validate};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "conceptarium")]
#[command(about = "Local Rust query, validation, registry, and projection tooling")]
struct Cli {
    /// Conceptarium repository root.
    #[arg(long, global = true, default_value = ".")]
    root: PathBuf,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Validate registry, entries, relations, and corpus invariants.
    Validate {
        /// Treat warnings as failures.
        #[arg(long)]
        strict: bool,
    },

    /// Generate disposable projections from canonical Markdown/YAML.
    Project {
        /// Projection output directory.
        #[arg(long, default_value = "build")]
        output_dir: PathBuf,
    },

    /// Print a concept and its full materialized entry when available.
    Get {
        /// Stable concept ID or exact term.
        key: String,
    },

    /// Local scan search. Tantivy-backed ranked search is the next backend.
    Search {
        query: String,
        #[arg(long, default_value_t = 20)]
        limit: usize,
    },

    /// Filter the concept universe by structured metadata.
    List {
        #[arg(long)]
        domain: Option<String>,
        #[arg(long = "type")]
        kind: Option<String>,
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        materialization: Option<String>,
    },

    /// Show typed incoming/outgoing relations for a concept.
    Relations {
        key: String,
        #[arg(long, default_value = "both")]
        direction: String,
        #[arg(long)]
        relation: Option<String>,
    },

    /// Find a shortest directed path through explicit typed relations.
    Path {
        from: String,
        to: String,
    },

    /// Manipulate predicate-presence records.
    Registry {
        #[command(subcommand)]
        command: RegistryCommand,
    },

    /// Convenience alias for the registry-only promotion queue.
    Queue {
        #[arg(long)]
        group: Option<String>,
    },

    /// Show backend implementation order and current support.
    Backends,
}

#[derive(Debug, Subcommand)]
enum RegistryCommand {
    /// Capture a concept without requiring definition or ontology work.
    Capture {
        term: String,
        #[arg(long)]
        id: Option<String>,
        #[arg(long, default_value = "unknown")]
        date: String,
        #[arg(long)]
        group: Option<String>,
        #[arg(long)]
        note: Option<String>,
        #[arg(long)]
        context: Option<String>,
        #[arg(long, default_value = "unplaced")]
        ontology_state: String,
    },

    /// Point a registered concept at a newly created canonical entry.
    Materialize {
        id: String,
        #[arg(long)]
        entry: PathBuf,
    },

    /// Show registry-only concepts.
    Queue {
        #[arg(long)]
        group: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Validate { strict } => {
            let corpus = Corpus::load(&cli.root)?;
            if !validate::run(&corpus, strict)? {
                std::process::exit(1);
            }
        }
        Command::Project { output_dir } => {
            let corpus = Corpus::load(&cli.root)?;
            let output = if output_dir.is_absolute() {
                output_dir
            } else {
                cli.root.join(output_dir)
            };
            project::generate(&corpus, &output)?;
        }
        Command::Get { key } => {
            let corpus = Corpus::load(&cli.root)?;
            query::get(&corpus, &key)?;
        }
        Command::Search { query: text, limit } => {
            let corpus = Corpus::load(&cli.root)?;
            query::search(&corpus, &text, limit);
        }
        Command::List {
            domain,
            kind,
            status,
            materialization,
        } => {
            let corpus = Corpus::load(&cli.root)?;
            query::list(
                &corpus,
                domain.as_deref(),
                kind.as_deref(),
                status.as_deref(),
                materialization.as_deref(),
            );
        }
        Command::Relations {
            key,
            direction,
            relation,
        } => {
            let corpus = Corpus::load(&cli.root)?;
            query::relations(
                &corpus,
                &key,
                query::Direction::parse(&direction)?,
                relation.as_deref(),
            )?;
        }
        Command::Path { from, to } => {
            let corpus = Corpus::load(&cli.root)?;
            query::path(&corpus, &from, &to)?;
        }
        Command::Registry { command } => match command {
            RegistryCommand::Capture {
                term,
                id,
                date,
                group,
                note,
                context,
                ontology_state,
            } => registry::capture(
                &cli.root,
                registry::CaptureArgs {
                    term,
                    id,
                    date,
                    group,
                    note,
                    context,
                    ontology_state,
                },
            )?,
            RegistryCommand::Materialize { id, entry } => {
                registry::materialize(&cli.root, &id, &entry)?;
            }
            RegistryCommand::Queue { group } => {
                registry::queue(&cli.root, group.as_deref())?;
            }
        },
        Command::Queue { group } => {
            registry::queue(&cli.root, group.as_deref())?;
        }
        Command::Backends => query::backends(),
    }

    Ok(())
}
