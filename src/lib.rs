pub mod config;

use config::DrawConfig;
use mdbook_preprocessor::book::{Book, BookItem};
use mdbook_preprocessor::errors::Error;
use mdbook_preprocessor::{Preprocessor, PreprocessorContext};

/// The mdbook-draw preprocessor.
/// Finds fenced ```draw blocks and replaces them with canvas HTML.
pub struct Draw;

impl Draw {
    pub fn new() -> Self {
        Self
    }
}

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

/// Walk all lines of a chapter and replace ```draw blocks with HTML.
fn rewrite_chapter(input: &str) -> String {
    let mut out = String::new();
    let mut in_block = false;
    let mut buf = String::new();

    for line in input.lines() {
        if in_block {
            if line.trim_start().starts_with("```") {
                // End of the draw block. Render it
                out.push_str(&render_draw_html(&buf));
                in_block = false;
                buf.clear();
            } else {
                buf.push_str(line);
                buf.push('\n');
            }
        } else if line.trim_start().starts_with("```draw") {
            in_block = true;
            buf.clear();
        } else {
            out.push_str(line);
            out.push('\n');
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
    html.push_str("<div class=\"mdbook-draw-container\">\n");

    // Optional title above the canvas
    if !cfg.title.is_empty() {
        html.push_str(&format!(
            "  <p class=\"mdbook-draw-title\">{}</p>\n",
            cfg.title
        ));
    }

    // The canvas element itself — JS reads data-* to configure it
    html.push_str(&format!(
        "  <canvas\n    id=\"{}\"\n    class=\"mdbook-draw-canvas\"\n    \
         width=\"{}\"\n    height=\"{}\"\n    \
         data-background=\"{}\"\n    \
         style=\"border:1px solid #ccc; cursor:crosshair; \
                background:{};\">\n  </canvas>\n",
        cfg.id, cfg.width, cfg.height, cfg.background, cfg.background
    ));

    // Toolbar — pencil/eraser/clear (JS will wire these up)
    html.push_str(&format!(
        "  <div class=\"mdbook-draw-toolbar\" data-canvas-id=\"{}\">\n",
        cfg.id
    ));
    html.push_str("    <button data-tool=\"pencil\">✏️ Pencil</button>\n");
    html.push_str("    <button data-tool=\"eraser\">🧹 Eraser</button>\n");
    html.push_str(
        "    <input type=\"color\" data-role=\"color\" value=\"#000000\" title=\"Color\">\n",
    );
    html.push_str("    <input type=\"range\" data-role=\"size\" min=\"1\" max=\"30\" value=\"4\" title=\"Brush size\">\n");
    html.push_str("    <button data-tool=\"clear\">🗑️ Clear</button>\n");
    html.push_str("  </div>\n");

    html.push_str("</div>\n");
    html
}
