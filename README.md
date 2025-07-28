# pak000 Repository

This repository contains a simple WebAssembly-based PAK file parser written in Rust and a minimal JavaScript front end.

## Known Issues and Fixes

* **Outdated `typed-arena` dependency** – The crate version used in `Cargo.toml` no longer exists on crates.io. It has been removed.
* **Unused imports** – `lib.rs` and `security.rs` imported `typed_arena::Arena` and `std::cmp` but never used them. These imports were removed.
* **Broken tests** – `security.rs` referenced a `MAX_FILE_SIZE` constant that does not exist. The faulty test was removed.
* **Invalid package version** – `package.json` requested `webpack-bundle-analyzer@^4.11.0`, which is not published. It now targets `^4.10.2`.
* **Missing entry file** – The webpack configuration expected `src/index.js`, which was missing. A simple module re-export file has been added.
* **Broken JSX markup** – The navigation buttons in `HexEditor.jsx` lacked closing tags. They are now properly closed.
* **Missing newline in several files** – Some files lacked a terminating newline which could cause warnings. Newlines were added where required.

## Building

The project includes a Rust parser and a small JavaScript front‑end. You can build the Rust crate and front end separately or use the provided Dockerfile for a combined build. After cloning the repo, run:

```bash
# Build Rust code
cd pak_parser
cargo build --release

# Install Node dependencies and build front end
cd ../pak_editor
npm install
npm run build
```

To run the test suite for the Rust parser:

```bash
cd pak_parser
cargo test
```

## Security Considerations

The parser performs only minimal validation on `.pak` headers and entries. Further improvements such as bounds checking on entry names or sanitizing WebAssembly interactions should be considered for production use.

