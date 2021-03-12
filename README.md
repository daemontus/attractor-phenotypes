## Biodivine Library Template

This is a template project for a general Rust based Biodivine library. It comes with a few useful features pre-enabled. 

Provided features:
 - Github Actions integration pre-configured with Codecov code coverage.
 - `LICENSE` and `.gitignore` files.
 - Git hooks to verify formatting and tests integrity before committing (run `git config core.hooksPath .githooks` to enable local git hooks).
 - Run `cargo make rich-doc` and `cargo make rich-doc-dev` to generate documentation with Mermaid and KaTeX enabled (`dev` variant includes also internal functions).
 - Run `cargo make` to run standard test process and compile basic docs, but also run automatic formatting tool of source code (make sure you apply formatting every time before commit).