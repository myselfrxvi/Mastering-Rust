fn main() {
    println!("==================================================");
    println!("   🦀 Module 02: Ownership, Borrowing & Slices    ");
    println!("==================================================");

    // 1. Stack Allocation (Copy Trait)
    demonstrate_copy_types();

    // 2. Heap Allocation & Move Semantics
    demonstrate_move_semantics();

    // 3. Cloning for Deep Copies
    demonstrate_cloning();

    // 4. Borrowing with Immutable References (&T)
    demonstrate_immutable_borrowing();

    // 5. Mutable References (&mut T) & The Aliasing Rule
    demonstrate_mutable_borrowing();

    // 6. Slices: Safe References into Contiguous Memory (&str, &[T])
    demonstrate_slices();
}

fn demonstrate_copy_types() {
    println!("\n[1] Copy Types (Stack Memory)");
    let a = 100;
    let b = a; // Bitwise copy because i32 implements Copy
    println!("  a = {a}, b = {b}");
    println!("  Both are valid on the stack independently.");
}

fn demonstrate_move_semantics() {
    println!("\n[2] Move Semantics (Heap Memory)");
    // String is a 3-word struct on stack: (ptr, len, cap) pointing to heap buffer
    let s1 = String::from("Rustaceans");
    
    // Ownership MOVES from s1 to s2.
    // Rust does not shallow copy or deep copy; it moves ownership and invalidates s1.
    let s2 = s1;
    println!("  s2 received ownership: \"{s2}\"");
    // println!("{s1}"); // <-- If uncommented, compile error: borrow of moved value!
}

fn demonstrate_cloning() {
    println!("\n[3] Explicit Deep Copy via Clone");
    let original = String::from("Immutable Data");
    let duplicate = original.clone(); // Allocates new heap buffer
    println!("  original: \"{original}\", duplicate: \"{duplicate}\"");
}

fn demonstrate_immutable_borrowing() {
    println!("\n[4] Immutable Borrowing (&T)");
    let text = String::from("Concurrent Readers");
    
    // Multiple readers are allowed simultaneously
    let r1 = &text;
    let r2 = &text;
    let r3 = &text;
    
    println!("  Reader 1: \"{r1}\"");
    println!("  Reader 2: \"{r2}\"");
    println!("  Reader 3: \"{r3}\"");
    println!("  Calculated length via borrow: {}", get_length(&text));
    println!("  'text' is still owned here: \"{text}\"");
}

fn get_length(s: &String) -> usize {
    s.len()
} // `s` goes out of scope here, but since it doesn't own the data, nothing is deallocated.

fn demonstrate_mutable_borrowing() {
    println!("\n[5] Mutable Borrowing (&mut T)");
    let mut document = String::from("Draft");
    println!("  Original: \"{document}\"");

    // Borrow exclusively to modify
    append_signature(&mut document);
    println!("  After modification: \"{document}\"");

    // Rust Rule: You CANNOT have an immutable reference while a mutable reference exists!
    // Why? To guarantee that readers never observe half-written or reallocated data.
}

fn append_signature(doc: &mut String) {
    doc.push_str(" [Signed & Verified]");
}

fn demonstrate_slices() {
    println!("\n[6] String & Array Slices");
    let full_string = String::from("Hello Rust World");

    // Slices let you reference a contiguous sequence without copying
    let first_word: &str = &full_string[0..5];
    let second_word: &str = &full_string[6..10];
    println!("  First word slice: \"{first_word}\"");
    println!("  Second word slice: \"{second_word}\"");

    // Array slices
    let numbers = [10, 20, 30, 40, 50];
    let middle_slice: &[i32] = &numbers[1..4];
    println!("  Array slice [1..4]: {middle_slice:?}");
}
