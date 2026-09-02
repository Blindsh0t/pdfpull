# pdfpull

> Work-in-progress: Product #1 of Han's move from Python scripting to Rust.
> This repo is the **code**. The plan, assessment, and the full product pipeline
> live in [`../learning-rust/docs/`](../learning-rust/docs/).

**Goal:** a single binary that turns PDF tables and statements (bank, credit
card, invoices) into structured CSV/JSON — local-only, no cloud, one file.

## Status

Scaffolded (cargo binary project). The real work starts with Milestone 0 in
`../learning-rust/docs/rust-roadmap.md` — ship `v0.1`: a `clap` CLI that reports
a file's size and type, with tests + CI.

## Guidelines (from the roadmap — non-negotiable)

- **Write first, review second.** No AI-generated code pasted in as "the answer."
- Every milestone ships a **working binary** — never hoard code nobody runs.
- Design (module layout, data model, template format) by hand before any AI sees it.
- rustfmt + clippy (`-D warnings`) + `cargo test` are part of "done," not polish.

## Toolchain

Pinned in `rust-toolchain.toml` so the build is reproducible anywhere.
