# 01 - Fundamentals and CLI Application

A complete, beginner-friendly CLI application demonstrating the foundational pillars of Rust.

---

## 📌 Concepts Mastered

1. **Variables & Mutability**
   - Immutability by default (`let`).
   - Explicit mutability (`let mut input = String::new();`).

2. **Standard I/O & Borrowing Basics**
   - Reading lines from standard input (`io::stdin().read_line(&mut input)`).
   - Passing a mutable reference `&mut` to allow the function to populate the buffer without taking ownership.

3. **Shadowing**
   - Transforming text to a clean trimmed slice (`let input = input.trim();`).
   - Re-declaring identifiers across transformations.

4. **Pattern Matching with `Result<T, E>`**
   - Handling operations that can fail without runtime exceptions or crashes.
   - Matching on `Ok(num)` vs `Err(_)`.

5. **Looping with Return Values**
   - Using `loop` as an expression.
   - Using `break num;` to exit the loop and return a value directly into a variable.

6. **Expressions vs. Statements**
   - `if` / `else` as an expression that evaluates directly to a value.
   - Functions with implicit expression returns (`price * quantity` without `return` or `;`).

---

## 🚀 How to Run

From the root directory:
```bash
cargo run -p fundamentals_and_cli
```
