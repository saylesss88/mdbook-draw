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
    html.push_str("<button data-tool=\"text\">T Text</button>");
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

#[cfg(test)]
mod tests {
    use super::*;

    // rewrite_chapter

    #[test]
    fn passthrough_no_draw_blocks() {
        let input = "# Hello\n\nSome text.\n\n```rust\nlet x = 1;\n```\n";
        assert_eq!(rewrite_chapter(input), input);
    }

    #[test]
    fn single_draw_block_replaced_with_html() {
        let input = "Before\n```draw\n```\nAfter\n";
        let out = rewrite_chapter(input);
        assert!(out.contains("mdbook-draw-container"));
        assert!(out.contains("Before"));
        assert!(out.contains("After"));
        assert!(!out.contains("```draw"));
    }

    #[test]
    fn draw_block_with_config_content() {
        let input = "```draw\ntitle: My Canvas\n```\n";
        let out = rewrite_chapter(input);
        assert!(out.contains("mdbook-draw-title"));
        assert!(out.contains("My Canvas"));
    }

    #[test]
    fn multiple_draw_blocks_all_replaced() {
        let input = "```draw\n```\nMiddle\n```draw\n```\n";
        let out = rewrite_chapter(input);
        let count = out.matches("mdbook-draw-container").count();
        assert_eq!(count, 2);
        assert!(out.contains("Middle"));
    }

    #[test]
    fn unclosed_draw_block_passed_through() {
        let input = "```draw\nsome content\n";
        let out = rewrite_chapter(input);
        assert!(out.contains("```draw"));
        assert!(out.contains("some content"));
    }

    #[test]
    fn non_draw_fenced_block_untouched() {
        let input = "```python\nprint('hi')\n```\n";
        assert_eq!(rewrite_chapter(input), input);
    }

    #[test]
    fn closing_fence_with_trailing_content_not_treated_as_close() {
        let input = "```draw\n``` not a close\n```\n";
        let out = rewrite_chapter(input);
        assert!(out.contains("mdbook-draw-container"));
        // "not a close" becomes part of the canvas content, not raw text
        assert!(!out.contains("``` not a close"));
    }
    //  render_draw_html

    #[test]
    fn html_contains_required_elements() {
        let html = render_draw_html("");
        assert!(html.contains("<canvas"));
        assert!(html.contains("mdbook-draw-canvas"));
        assert!(html.contains("mdbook-draw-toolbar"));
        assert!(html.contains("mdbook-draw-container"));
    }

    #[test]
    fn html_no_title_element_when_empty() {
        let html = render_draw_html("");
        assert!(!html.contains("mdbook-draw-title"));
    }

    #[test]
    fn html_title_present_when_configured() {
        let html = render_draw_html("title: Diagram\n");
        assert!(html.contains("mdbook-draw-title"));
        assert!(html.contains("Diagram"));
    }

    #[test]
    fn html_toolbar_has_all_tools() {
        let html = render_draw_html("");
        for tool in &["pencil", "line", "circle", "text", "eraser", "clear"] {
            assert!(html.contains(tool), "missing tool: {tool}");
        }
        assert!(html.contains("data-role=\"save\""));
        assert!(html.contains("data-role=\"export-png\""));
    }

    #[test]
    fn html_canvas_id_matches_toolbar_data_canvas_id() {
        let html = render_draw_html("");
        // Extract id="..." value and check toolbar references the same id
        let id_start = html.find("id=\"").unwrap() + 4;
        let id_end = html[id_start..].find('"').unwrap() + id_start;
        let canvas_id = &html[id_start..id_end];
        let expected = format!("data-canvas-id=\"{}\"", canvas_id);
        assert!(html.contains(&expected));
    }

    #[test]
    fn html_background_applied_to_both_attribute_and_style() {
        // Default background should appear in both data-background and style
        let html = render_draw_html("");
        let bg_attr_count = html.matches("data-background=").count();
        assert_eq!(bg_attr_count, 1);
        assert!(html.contains("background:"));
    }

    #[test]
    fn html_wrapped_in_blank_lines_for_md_parser() {
        let html = render_draw_html("");
        assert!(html.starts_with('\n'));
        assert!(html.ends_with('\n'));
    }

    #[test]
    fn config_defaults() {
        let cfg = DrawConfig::from_block("");
        assert_eq!(cfg.id, "draw-canvas");
        assert_eq!(cfg.width, 600);
        assert_eq!(cfg.height, 400);
        assert!(cfg.title.is_empty());
        assert_eq!(cfg.background, "#ffffff");
    }

    #[test]
    fn config_parses_all_fields() {
        let input = "id: my-id\nwidth: 800\nheight: 500\ntitle: Test\nbackground: #000000\n";
        let cfg = DrawConfig::from_block(input);
        assert_eq!(cfg.id, "my-id");
        assert_eq!(cfg.width, 800);
        assert_eq!(cfg.height, 500);
        assert_eq!(cfg.title, "Test");
        assert_eq!(cfg.background, "#000000");
    }

    #[test]
    fn config_ignores_unknown_keys() {
        let cfg = DrawConfig::from_block("foo: bar\n");
        assert_eq!(cfg.width, 600); // defaults intact
    }

    #[test]
    fn config_bad_width_falls_back_to_default() {
        let cfg = DrawConfig::from_block("width: notanumber\n");
        assert_eq!(cfg.width, 600);
    }
}
