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
