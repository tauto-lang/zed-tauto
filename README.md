# zed-tauto

A [Zed](https://zed.dev/) extension for the Tauto programming language.

- Recognizes `.taut` files as Tauto.
- Syntax highlighting, bracket matching, indentation and outline, using [tree-sitter-tauto](https://github.com/tauto-lang/tree-sitter-tauto).
- Starts the Tauto language server (`tauto lsp`) for diagnostics, completion, hover, go to definition, references, rename, code actions and semantic highlighting.

## Requirements

The `tauto` command must be on your `PATH`. The extension does not download it.

## Installing from source

1. Install Rust via [rustup](https://rustup.rs/) (Zed builds the extension with it). The first build downloads the nightly toolchain and the `wasm32-wasip2` target pinned in `rust-toolchain.toml`.
2. Clone this repository.
3. In Zed, run `zed: install dev extension` from the command palette and select the cloned directory.

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
