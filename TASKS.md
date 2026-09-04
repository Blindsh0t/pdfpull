# pdfpull — Task Board

Tracking tickets as you'd track work tickets. Finish all checklist items, then
close the ticket (mark it done). A ticket isn't done until its checks pass.

---

## Ticket M0 — Foundations → `v0.1`  *(active)*

**Owner:** Han · **Depends on:** nothing · **Status:** ✅ under way (done: PDF
listing + sizes; todo: shape it professionally)

**Goal:** a real `clap` CLI that takes a directory path and prints every PDF in
it with its size in bytes — as a properly structured, tested, lint-clean Rust
program. (You've got the hard part working; this ticket gives the same behavior
a professional shape.)

### Definition of Done — all boxes must be true
- [ ] CLI takes the directory as an **argument** via `clap` (no interactive `stdin` prompt)
- [ ] Still lists PDF files + sizes in bytes (keep current behavior)
- [ ] `cargo run -- /some/dir` is the way you run it
- [ ] `cargo clippy -- -D warnings` passes with **zero warnings** — this is the gate
- [ ] PDF-filtering lives in a **pure function** `pdfs_in_dir(dir) -> Vec<(String, u64)>`
      (reads a dir, returns only PDF paths + sizes; no printing inside it)
- [ ] At least **one unit test** on `pdfs_in_dir` using a `testdata/` fixture folder
      (no hardcoded `/Users/...` paths in tests)
- [ ] `cargo test` passes
- [ ] README.md is committed (no longer gitignored)

### Why this shape (so you're aiming at the right target)
- **args not stdin** → real CLIs are scriptable, pipe-able, and testable.
- **iterators** (`filter_map` + `collect`) → kills your 4-deep nesting; clippy flags the old shape.
- **clippy clean** → the linter catches the mistakes a reviewer would, before I do.
- **pure function + test** → forces you to separate *logic* from *I/O*. That split **is architecture** — and it's the whole point of Milestone 0's refactor.

### Steps (suggested order — do them one at a time)
1. `cargo add clap --features derive`
2. Open the `clap` derive example (docs.rs/clap → "derive" example) — copy the *shape*, write your own fields.
3. Replace the `stdin` prompt with a `#[derive(Parser)]` `Args` struct holding a `PathBuf` path.
4. Extract the filter loop into `fn pdfs_in_dir(...)` — move the body out of `main`.
5. Refactor the nested `if let` / `if` to an iterator chain (`filter_map` + `collect`).
6. Run `cargo clippy -- -D warnings` and fix until green.
7. Make `testdata/` with 1–2 real sample PDFs; write `#[cfg(test)] mod tests` asserting the function returns them + correct sizes.
8. `cargo test`.
9. Commit with a clear message; push.

### Resources
- **What is clippy:** a linter (a picky senior reviewing your code). `-D warnings` = treat every warning as an error. Explain: `rustc --explain <code>` when a compile error confuses you.
- **clap:** docs.rs/clap
- **clippy:** doc.rust-lang.org/clippy (`cargo fix` auto-applies some fixes)
- **std iterators:** doc.rust-lang.org/std/iter (`filter_map`, `collect`)

---

## (Future tickets — parked)
- **M1 — PDF text extraction → `v0.2`** (lib/bin split, `lopdf`, golden tests, 4-gate CI)
- **M2 — `pdfpull` MVP → `v0.3`** (table detection, TOML templates, CSV/JSON writers)
- **M3 — Distribution & first sale → `v0.4`** (cross-platform release, Gumroad/LemonSqueezy, launch)
- **M4 — Maintenance → `v0.5`+** (real user bug → fix → release; then `pdfscrub`, `watchpage`)

Full detail for each: `../learning-rust/docs/rust-roadmap.md`.
