# mdbook-draw

An [mdBook](https://rust-lang.github.io/mdBook/) preprocessor that adds
interactive freehand drawing canvases to your book pages.

Write a fenced `draw` block in any chapter, and it becomes a live canvas your
readers can draw on: pencil, eraser, color picker, brush size, and clear.

## Demo

````markdown
```draw
id: my-diagram
width: 600
height: 400
title: My Diagram
background: #f8f9fa
```
````

This renders as an interactive canvas with a toolbar directly in the page.

## Installation

```bash
cargo install mdbook-draw
```

## Setup

Run this once from your book's root directory (where `book.toml` lives):

```bash
mdbook-draw init
```

This will:

1. Create `theme/draw.js` — the canvas drawing UI
2. Add `additional-js = ["theme/draw.js"]` to `[output.html]` in `book.toml`
3. Add the `[preprocessor.draw]` section to `book.toml`

Then build or serve your book as normal:

```bash
mdbook serve --open
```

## Draw block syntax

All fields are optional and have sensible defaults:

````markdown
```draw
id: diagram-1
width: 600
height: 400
title: My Diagram
background: #f8f9fa
```
````

| Field        | Default       | Description                                                        |
| :----------- | :------------ | :----------------------------------------------------------------- |
| `id`         | `draw-canvas` | Unique HTML id, required if you have multiple canvases on one page |
| `width`      | `600`         | Canvas width in pixels                                             |
| `height`     | `400`         | Canvas height in pixels                                            |
| `title`      | (none)        | Optional label rendered above the canvas                           |
| `background` | `#ffffff`     | Canvas background color                                            |

> ⚠️ If you have more than one draw block on a single page, each must have a
> unique id.

## Toolbar

| Control      | Description                          |
| :----------- | :----------------------------------- |
| ✏️ Pencil    | Frehand drawing (default tool)       |
| 🧹 Eraser    | Erase strokes (paints with bg color) |
| Color picker | Choose stroke color                  |
| Size slider  | Adjust brush/eraser size             |
| 🗑️ Clear     | Reset the entire canvas              |

## How it works

- Rust preprocessor (src/lib.rs): scans markdown for ```draw fenced blocks,
  parses the key-value config, and replaces them with raw HTML `<canvas>`
  elements and toolbar `<div>`s
- JavaScript (`theme/draw.js`): runs at page load, finds every canvas by its
  toolbar's data-canvas-id attribute, and wires up mouse events for drawing

The preprocessor follows the standard mdBook preprocessor protocol: it reads a
JSON-encoded book from stdin and writes the processed book to stdout.

### book.toml reference

After running `mdbook-draw init`, your `book.toml` will contain:

```toml
[preprocessor.draw]
command = "mdbook-draw"

[output.html]
additional-js = ["theme/draw.js"]
```

### License

[Apache-2.0](https://github.com/saylesss88/mdbook-draw/blob/main/LICENSE)
