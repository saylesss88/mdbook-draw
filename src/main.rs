use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use mdbook_preprocessor::book::Book;
use mdbook_preprocessor::{parse_input, Preprocessor, PreprocessorContext};
use std::{fs, io, path::Path};

use mdbook_draw::Draw;

// Embed draw.js into the binary at compile time.
const DRAW_JS: &str = include_str!("../theme/draw.js");

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

fn handle_init() -> Result<()> {
    println!("📦 Initializing mdbook-draw...");

    // 1. Create theme/ dir if needed
    let theme_dir = Path::new("theme");
    if !theme_dir.exists() {
        fs::create_dir(theme_dir).context("Failed to create theme/ directory")?;
    }

    // 2. Write draw.js into theme/
    let js_path = theme_dir.join("draw.js");
    fs::write(&js_path, DRAW_JS).context("Failed to write theme/draw.js")?;
    println!("✅ Created theme/draw.js");

    // 3. Patch book.toml — add additional-js if not already present
    let toml_path = Path::new("book.toml");
    if toml_path.exists() {
        let content = fs::read_to_string(toml_path)?;

        if content.contains("draw.js") {
            println!("ℹ️  book.toml already references draw.js — skipping.");
        } else if content.contains("[output.html]") {
            // Section exists — append our line inside it
            let patched = content.replace(
                "[output.html]",
                "[output.html]\nadditional-js = [\"theme/draw.js\"]",
            );
            fs::write(toml_path, patched)?;
            println!("✅ Added additional-js to [output.html] in book.toml");
        } else {
            // No [output.html] section — append the whole block
            let mut patched = content;
            patched.push_str("\n[output.html]\nadditional-js = [\"theme/draw.js\"]\n");
            fs::write(toml_path, patched)?;
            println!("✅ Added [output.html] section to book.toml");
        }
    } else {
        println!("⚠️  book.toml not found. Are you in your book's root directory?");
        println!("   Manually add this to book.toml:");
        println!("   [output.html]");
        println!("   additional-js = [\"theme/draw.js\"]");
    }

    // 4. Remind user to add the preprocessor line too
    println!("\n🚀 Done! Also make sure book.toml has:");
    println!("   [preprocessor.draw]");
    println!("   command = \"mdbook-draw\"");
    println!("\nThen run: mdbook build");

    Ok(())
}
