# 03 - Structs, Methods & Enums with Pattern Matching

This module demonstrates how modern Rust models real-world domain entities without traditional Object-Oriented class hierarchies or null references.

---

## 📌 Concepts Mastered

1. **Structs & Data Modeling**
   - Defining custom types with named fields.
   - Deriving `Debug` (`#[derive(Debug)]`) for `{:#?}` pretty-printing.

2. **The `impl` Block (Methods & Associated Functions)**
   - **Associated Functions (Constructors)**: `User::new(...) -> Self` (no `self` parameter).
   - **Immutable Methods**: `&self` for read-only access to instance data.
   - **Mutable Methods**: `&mut self` for in-place state mutation.

3. **Algebraic Data Types (Enums with Data)**
   - Unlike C/Java/TypeScript enums, Rust enum variants can embed rich data:
     - Unit variant: `Membership::Free`
     - Struct variant: `Membership::Premium { days_remaining: u32 }`
     - Tuple variant: `Membership::VIP(String)`

4. **The `Option<T>` Enum (Zero Nulls)**
   - How Rust permanently eliminates `null` pointer exceptions.
   - Using `Some(T)` and `None`.
   - Handling optional fields safely with `if let Some(...) = ...`.

5. **Exhaustive Pattern Matching (`match`)**
   - Matching variants and destructuring their inner data at compile-time.
   - Compiler guarantees no cases are ever forgotten.

---

## 🚀 How to Run

From the root directory:
```bash
cargo run -p structs_enums_and_pattern_matching
```
