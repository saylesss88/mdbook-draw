# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2026-07-17

### Added

- Unit tests for lib
- Add text
- Drag line/Drag circle
- Save Button/ Export PNG Button
- Auto-save/Auto-restore

### Fixed

- init injection appends to existing additional-js rather than adding another
  causing errors
- Non-working bg color, changes in `draw.js`

## [0.1.0] - 2026-07-17

### Added

- `mdbook-draw init` command to add everything required to `book.toml`
- JavaScript for the canvas draw.js
- Implement required methods for mdbook-preprocessor
- Basic structure based off of mdbooks no-op preprocessor as well as
  mdbook-nix-repl
- Project README
- This Changelog
- Apache License
- Initialize project with dependencies

### Fixed

- JS problems
- Refactor project
