# 02 - Ownership, Borrowing & Slices

This module dissects the core innovation that makes Rust uniquely fast and secure: **Compile-time Memory Safety without a Garbage Collector**.

---

## 📌 Concepts Mastered

1. **Stack vs. Heap Memory Layout**
   - **Stack**: Fixed-size data with `Copy` semantics (integers, bools, floats, char).
   - **Heap**: Dynamic-size data (`String`). Represented by a stack triplet: `(pointer, length, capacity)`.

2. **Move Semantics**
   - Assigning heap types (`let s2 = s1;`) transfers ownership and invalidates `s1`.
   - Prevents double-free bugs at compile time.

3. **Borrowing with Immutable References (`&T`)**
   - Passing references to functions allows reading data without taking ownership (`takes_os(&s1);`).
   - Multiple immutable readers can exist simultaneously.

4. **Mutable References (`&mut T`)**
   - Allows functions to modify borrowed data in-place (`add_word(&mut s1);`).
   - Requires the underlying variable to be `mut`.

5. **The Golden Rule of Borrowing (Aliasing XOR Mutability)**
   - You can have **either**:
     - Any number of immutable references (`&T`)
     - **OR** exactly one mutable reference (`&mut T`)
   - Never both simultaneously. This completely prevents dangling pointers and data races.

6. **Slices (`&str`)**
   - Zero-allocation views into contiguous memory chunks (`&message[0..9]`).

---

## 🚀 How to Run

From the root directory:
```bash
cargo run -p ownership_and_borrowing
```
