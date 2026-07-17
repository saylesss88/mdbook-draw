mod init;

use crate::init::handle_init;
use anyhow::Result;
use clap::{Parser, Subcommand};
use mdbook_preprocessor::book::Book;
use mdbook_preprocessor::{Preprocessor, PreprocessorContext, parse_input};
use std::io;

use mdbook_draw::Draw;

#[derive(Parser)]
#[command(name = "mdbook-draw")]
#[command(about = "An mdBook preprocessor for interactive drawing canvas blocks")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Report whether this preprocessor supports a given renderer.
    Supports { renderer: String },
    /// Initialize mdbook-draw in the current book directory.
    /// Run this once from your book's root (where book.toml lives).
    Init,
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Supports { renderer }) => {
            let supported = Draw.supports_renderer(&renderer).unwrap_or(false);
            std::process::exit(i32::from(!supported));
        }
        Some(Commands::Init) => handle_init(),
        // Default: act as mdBook preprocessor, read JSON from stdin
        None => {
            let (ctx, book): (PreprocessorContext, Book) = parse_input(io::stdin())?;
            let processed = Draw.run(&ctx, book)?;
            serde_json::to_writer(io::stdout(), &processed)?;
            Ok(())
        }
    }
}
