# 🦀 Mastering Rust: Zero to Hero

A modular, hands-on, zero-to-hero curriculum and reference repository for mastering modern Rust. From foundational memory models, ownership, and zero-cost abstractions to multithreaded systems, asynchronous runtimes, and full-stack CLI applications.

---

## 📚 Curriculum Roadmap

| Module | Topic | Key Concepts | Status |
| :--- | :--- | :--- | :---: |
| **[01-fundamentals-and-cli](./01-fundamentals-and-cli)** | **Runtime, Types & CLI** | Variables, `mut`, Shadowing, Types, Expressions, CLI args, Std I/O | ✅ Completed |
| **[02-ownership-and-borrowing](./02-ownership-and-borrowing)** | **The Memory Engine** | Stack vs Heap, Move semantics, Borrowing (`&`, `&mut`), Lifetimes | 🔄 In Progress |
| **03-structs-enums-pattern-matching** | **Modeling Data** | Structs, Methods (`impl`), Algebraic Enums, Exhaustive `match`, `if let` | ⏳ Upcoming |
| **04-error-handling-and-collections** | **Robust Applications** | `Result<T, E>`, `Option<T>`, `?` operator, `Vec`, `HashMap`, Strings | ⏳ Upcoming |
| **05-traits-generics-and-lifetimes** | **Polymorphism & Abstractions** | Traits, Generics, Derives, Trait Objects (`dyn Trait`), Explicit Lifetimes | ⏳ Upcoming |
| **06-memory-iterators-and-closures** | **Idiomatic Functional Rust** | `Box`, `Rc`, `Arc`, `RefCell`, Functional Closures, Zero-cost Iterators | ⏳ Upcoming |
| **07-concurrency-and-threads** | **Fearless Concurrency** | OS Threads, Message Passing (`mpsc`), Shared State (`Mutex`, `RwLock`), Data-race prevention | ⏳ Upcoming |
| **08-async-and-networking** | **Asynchronous Systems** | TCP, Async/Await, Tokio Runtime, Streams, HTTP services | ⏳ Upcoming |
| **Project-RustyVault** | **Capstone Project** | Production-grade encrypted CLI secret & snippet vault with zero-unsafe Rust | ⏳ Upcoming |

---

## 🛠️ Quick Start

This repository is configured as a unified **Cargo Workspace**.

### 1. Build all modules:
```bash
cargo build
```

### 2. Run a specific module:
```bash
# Using cargo package flag
cargo run -p fundamentals_and_cli

# Or navigating into the directory
cd 01-fundamentals-and-cli
cargo run
```

### 3. Check code without producing binaries (faster):
```bash
cargo check
```
