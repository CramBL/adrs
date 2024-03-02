use anyhow::Result;
use clap::{Parser, Subcommand};
use serde::Serialize;

pub mod adr;
mod cmd;

#[derive(Debug, Serialize)]
struct TemplateContext {
    title: String,
    number: i32,
    date: String,
    status: String,
    context: String,
    decision: String,
    consequences: String,
}

#[derive(Parser)]
#[command(version, about, long_about = None )]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initializes the directory of Architecture Decision Records
    Init(cmd::init::InitArgs),
    /// Create a new, numbered Architectural Decision Record
    New(cmd::new::NewArgs),
    /// Link Architectural Decision Records
    Link(cmd::link::LinkArgs),
    /// List Architectural Decision Records
    List(cmd::list::ListArgs),
    /// Show the current configuration
    Config {},
    /// Generates summary documentation about the Architectural Decision Records
    #[command(subcommand)]
    Generate(GenerateCommands),
}

#[derive(Debug, Subcommand)]
enum GenerateCommands {
    Toc {},
    Graph {},
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Init(args) => {
            cmd::init::run(args)?;
        }
        Commands::New(args) => {
            cmd::new::run(args)?;
        }
        Commands::Link(args) => {
            cmd::link::run(args)?;
        }
        Commands::List(args) => {
            cmd::list::run(args)?;
        }
        Commands::Config {} => {
            tracing::debug!("config");
        }
        Commands::Generate(c) => match c {
            GenerateCommands::Toc {} => {
                tracing::debug!("generate toc");
            }
            GenerateCommands::Graph {} => {
                tracing::debug!("generate graph");
            }
        },
    }
    Ok(())
}

// pub(crate) fn now() -> Result<String> {
//     let now = time::OffsetDateTime::now_local()?;
//     let x = now.format(format_description!("[year]-[month]-[day]"))?;
//     Ok(x)
// }

// pub(crate) fn adr_filename(title: &str) -> String {
//     title
//         .split_whitespace()
//         .collect::<Vec<&str>>()
//         .join("-")
//         .to_lowercase()
// }

// pub(crate) fn next_adr_sequence(path: impl AsRef<Path>) -> Result<i32> {
//     let entries = std::fs::read_dir(path)?;
//     let mut max = 0;
//     for entry in entries {
//         let entry = entry?;
//         let path = entry.path();
//         if path.is_file() {
//             let file_name = path.file_name().unwrap().to_str().unwrap();
//             if file_name.starts_with(char::is_numeric) {
//                 if let Some((num, _rest)) = file_name.split_once('-') {
//                     if let Ok(number) = num.parse::<i32>() {
//                         if number > max {
//                             max = number;
//                         }
//                     }
//                 }
//             }
//         }
//     }
//     Ok(max + 1)
// }
