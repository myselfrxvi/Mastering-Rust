# 01 - Fundamentals and CLI Application

Welcome to Module 01! This module establishes the core foundational mechanics of Rust and walks through building a functional CLI application.

---

## 📌 Concepts Covered

1. **Rust Toolchain & Cargo Workspace Architecture**
   - How `Cargo.toml` manages package manifests, binaries, and dependencies.
   - Using `cargo check`, `cargo build`, and `cargo run`.

2. **Variables, Mutability & Safety**
   - Immutability by default (`let`).
   - Explicit mutable declaration (`let mut`).
   - Why compiler-enforced immutability prevents data race conditions.

3. **Shadowing vs Mutation**
   - Re-declaring variables with `let` to reuse identifier names.
   - Changing types during sequential transformations (e.g., `&str` -> `u32`).

4. **Data Types**
   - **Scalars**: `i32`/`u32` (integers), `f64` (floats), `bool` (booleans), `char` (4-byte Unicode scalar).
   - **Compounds**: Tuples (fixed size, heterogeneous) and Arrays (fixed size, homogeneous, stack-allocated).

5. **Statements vs. Expressions**
   - Statements perform actions without returning values (`let x = 5;`).
   - Expressions evaluate to values (`x + 1` without semicolon).
   - Implicit function return values.

6. **Console I/O & CLI Args**
   - Inspecting command-line arguments via `std::env::args()`.
   - Flushing stdout and reading lines via `std::io::stdin()`.
   - Handling input parsing and fallback values.

---

## 🚀 How to Run

From the root of the workspace:
```bash
cargo run -p fundamentals_and_cli
```

Or pass CLI flags:
```bash
cargo run -p fundamentals_and_cli -- --help
```
