# 02 - Ownership, Borrowing & Slices

This module dissects the core innovation that makes Rust unique: **Compile-time Memory Safety without a Garbage Collector**.

---

## 📌 Concepts Covered

1. **The Three Rules of Ownership**
   - Every value in Rust has an owner.
   - There can only be one owner at a time.
   - When the owner goes out of scope, the memory is immediately deallocated (`drop`).

2. **Stack vs. Heap Memory Layout**
   - **Stack**: Fixed-size data with `Copy` semantics (integers, bools, chars, floats, fixed arrays).
   - **Heap**: Dynamic-size data (`String`, `Vec`). Represented by a stack triplet: `(pointer, length, capacity)`.

3. **Move Semantics vs. `clone()`**
   - Assignment of heap types moves ownership and invalidates the previous variable.
   - Deep copying is never implicit; it requires calling `.clone()`.

4. **Borrowing & References**
   - **Immutable References (`&T`)**: Shared read-only access.
   - **Mutable References (`&mut T`)**: Exclusive read-write access.

5. **The Golden Borrowing Rule (Aliasing XOR Mutability)**
   - You can have **either**:
     - Any number of immutable references (`&T`)
     - **OR** exactly one mutable reference (`&mut T`)
   - Never both simultaneously. This completely prevents data races at compile time.

6. **Slices (`&str`, `&[T]`)**
   - Non-owning views into a contiguous sequence of elements.
   - Eliminates out-of-sync index bugs.

---

## 🚀 How to Run

From the root of the workspace:
```bash
cargo run -p ownership_and_borrowing
```
