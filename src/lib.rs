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
        unimplemented!();
    }

    fn supports_renderer(&self, renderer: &str) -> Result<bool, Error> {
        Ok(renderer == "html")
    }
}
