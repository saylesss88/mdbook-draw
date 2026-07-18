pub mod config;

use config::DrawConfig;
use mdbook_preprocessor::book::{Book, BookItem};
use mdbook_preprocessor::errors::Error;
use mdbook_preprocessor::{Preprocessor, PreprocessorContext};
use std::fmt::Write as _;

#[allow(clippy::doc_markdown)]
/// The mdbook-draw preprocessor.
/// Finds fenced ```draw blocks and replaces them with canvas HTML.
pub struct Draw;

impl Preprocessor for Draw {
    fn name(&self) -> &'static str {
        "draw"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        book.for_each_mut(|item| {
            if let BookItem::Chapter(ref mut ch) = *item {
                ch.content = rewrite_chapter(&ch.content);
            }
        });
        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> Result<bool, Error> {
        Ok(renderer == "html")
    }
}

#[allow(clippy::doc_markdown)]
/// Walk all lines of a chapter and replace ```draw blocks with HTML.
fn rewrite_chapter(input: &str) -> String {
    let mut out = String::new();
    let mut in_block = false;
    let mut buf = String::new();

    for line in input.lines() {
        if in_block {
            // Closing fence: line is ONLY backticks (trimmed), nothing else after
            let trimmed = line.trim();
            if trimmed == "```" {
                // End of the draw block. Render it
                out.push_str(&render_draw_html(&buf));
                in_block = false;
                buf.clear();
            } else {
                buf.push_str(line);
                buf.push('\n');
            }
        } else {
            let trimmed = line.trim_start();
            if trimmed == "```draw" || trimmed.starts_with("```draw") {
                in_block = true;
                buf.clear();
            } else {
                out.push_str(line);
                out.push('\n');
            }
        }
    }

    // Unclosed block? Just pass the content through as-is.
    if in_block {
        out.push_str("```draw\n");
        out.push_str(&buf);
    }

    out
}

/// Emit the HTML placeholder that draw.js will pick up and turn into a canvas.
fn render_draw_html(content: &str) -> String {
    let cfg = DrawConfig::from_block(content);

    // We use data-* attributes to pass config to the JS side.
    // This is the same pattern nix-repl uses for its widget divs.
    let mut html = String::new();
    // Leading blank line, tells md parser "raw HTML starts here"
    html.push('\n');
    html.push_str("<div class=\"mdbook-draw-container\">\n");

    // Optional title above the canvas
    if !cfg.title.is_empty() {
        let _ = writeln!(html, "<p class=\"mdbook-draw-title\">{}</p>", cfg.title);
    }

    // The canvas element itself, JS reads data-* to configure it
    let _ = write!(
        html,
        "<canvas\n    id=\"{}\"\n    class=\"mdbook-draw-canvas\"\n    \
          width=\"{}\"\n    height=\"{}\"\n    \
          data-background=\"{}\"\n    \
          style=\"border:1px solid #ccc; cursor:crosshair; \
                 background:{};\">\n  </canvas>\n",
        cfg.id, cfg.width, cfg.height, cfg.background, cfg.background
    );

    // Toolbar: pencil/eraser/clear (JS will wire these up)
    let _ = writeln!(
        html,
        "<div class=\"mdbook-draw-toolbar\" data-canvas-id=\"{}\">",
        cfg.id
    );
    html.push_str("<button data-tool=\"pencil\">✏️ Pencil</button>");
    html.push_str("<button data-tool=\"line\">╱ Line</button>");
    html.push_str("<button data-tool=\"circle\">○ Circle</button>");
    html.push_str("<button data-tool=\"eraser\">🧹 Eraser</button>");
    html.push_str("<input type=\"color\" data-role=\"color\" value=\"#000000\" title=\"Color\">");
    html.push_str("<input type=\"range\" data-role=\"size\" min=\"1\" max=\"30\" value=\"4\" title=\"Brush size\">");
    html.push_str("<button data-tool=\"clear\">🗑️ Clear</button>");
    html.push_str("<button data-role=\"save\">💾 Save</button>");
    html.push_str("<button data-role=\"export-png\">🖼️ Export PNG</button>");
    html.push_str("</div>\n");
    html.push_str("</div>\n");

    // Trailing blank line, closes the raw HTML block for the md parser
    html.push('\n');

    html
}
