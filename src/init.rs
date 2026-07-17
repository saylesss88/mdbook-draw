use anyhow::{Context, Result};
use std::{fs, path::Path};

// Embed draw.js into the binary at compile time.
const DRAW_JS: &str = include_str!("../theme/draw.js");

pub fn handle_init() -> Result<()> {
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

    // 3. Patch book.toml, add additional-js if not already present
    let toml_path = Path::new("book.toml");

    if toml_path.exists() {
        let content = fs::read_to_string(toml_path)?;
        let mut patched = content.clone();

        // --- [preprocessor.draw] ---
        if content.contains("[preprocessor.draw]") {
            println!("ℹ️  book.toml already has [preprocessor.draw] — skipping.");
        } else {
            patched.push_str("\n[preprocessor.draw]\ncommand = \"mdbook-draw\"\n");
            println!("✅ Added [preprocessor.draw] to book.toml");
        }

        // --- [output.html] additional-js ---
        if patched.contains("draw.js") {
            println!("ℹ️  book.toml already references draw.js — skipping.");
        } else if patched.contains("[output.html]") {
            // Section exists — insert our line right after the header
            patched = patched.replace(
                "[output.html]",
                "[output.html]\nadditional-js = [\"theme/draw.js\"]",
            );
            println!("✅ Added additional-js to [output.html] in book.toml");
        } else {
            patched.push_str("\n[output.html]\nadditional-js = [\"theme/draw.js\"]\n");
            println!("✅ Added [output.html] section to book.toml");
        }

        fs::write(toml_path, patched)?;
    } else {
        println!("⚠️  book.toml not found. Are you in your book's root directory?");
        println!("   Manually add to book.toml:");
        println!("   [preprocessor.draw]");
        println!("   command = \"mdbook-draw\"");
        println!();
        println!("   [output.html]");
        println!("   additional-js = [\"theme/draw.js\"]");
    }

    println!("\n🚀 Done! Run: mdbook build");

    Ok(())
}
