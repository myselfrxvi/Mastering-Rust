# 04 - Collections & Robust Error Handling

This module explores Rust's standard dynamic collections (`Vec<T>` and `HashMap<K, V>`) and production-grade error handling using `Result<T, E>` and the `?` propagation operator.

---

## 📌 Concepts Mastered

1. **Vectors (`Vec<T>`)**
   - Heap-allocated, dynamic resizable arrays.
   - Pushing elements (`.push()`).
   - In-place mutation with `&mut` and dereferencing (`*num += 5`).
   - Safe access with `.get(index)` returning `Option<&T>` to prevent out-of-bounds panics.

2. **HashMaps (`HashMap<K, V>`)**
   - Key-value mappings stored on the heap.
   - The `.entry(key).or_insert(default)` and `.or_default()` patterns for frequency counting and grouping.

3. **Error Handling with `Result<T, E>` & `?` Operator**
   - Early return and error propagation with `?`.
   - Converting error types using `.map_err()`.
   - Transforming missing `Option` values into actionable errors with `.ok_or_else()`.

4. **Mini-Project: Student GradeBook System**
   - Combining `HashMap<String, Vec<u32>>` to model multi-tier relationships.
   - Parsing and validating raw string inputs into bounded numeric records with full error propagation.

---

## 🚀 How to Run

From the root directory:
```bash
cargo run -p error_handling_and_collections
```
