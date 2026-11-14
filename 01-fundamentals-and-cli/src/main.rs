use std::env;
use std::io::{self, Write};

fn main() {
    println!("==================================================");
    println!("   🦀 Module 01: Rust Fundamentals & CLI Engine   ");
    println!("==================================================");

    // 1. Command-Line Arguments Parsing
    let args: Vec<String> = env::args().collect();
    
    // Demonstrate CLI arguments handling
    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        print_usage(&args[0]);
        return;
    }

    println!("\n[1] Type System & Immutability");
    demonstrate_types_and_mutability();

    println!("\n[2] Shadowing & Transformations");
    demonstrate_shadowing();

    println!("\n[3] Interactive Age & Category Calculator");
    run_interactive_cli();
}

fn print_usage(program_name: &str) {
    println!("Usage: {program_name} [options]");
    println!("Options:");
    println!("  -h, --help    Show this help message");
}

fn demonstrate_types_and_mutability() {
    // Scalar Types
    let integer: i32 = 42;
    let float: f64 = 3.14159;
    let boolean: bool = true;
    let character: char = '🦀'; // Rust char is 4-byte Unicode scalar value!

    // Immutability vs Mutability
    let immutable_val = 10;
    let mut mutable_counter = 0;
    mutable_counter += 5;

    // Compound Types
    let tuple_person: (&str, u32, bool) = ("Ferris", 9, true);
    let (name, age, is_mascot) = tuple_person; // Destructuring

    let numbers_array: [i32; 4] = [10, 20, 30, 40]; // Fixed size on stack

    println!("  Scalars: int={integer}, float={float}, bool={boolean}, char={character}");
    println!("  Immutable: {immutable_val}, Mutated: {mutable_counter}");
    println!("  Tuple: name={name}, age={age}, mascot={is_mascot}");
    println!("  Array: len={}, first={}", numbers_array.len(), numbers_array[0]);
}

fn demonstrate_shadowing() {
    let raw_input = "   1024   ";
    println!("  Initial &str: \"{raw_input}\"");

    // Shadow 1: Trim whitespace, keeping &str
    let raw_input = raw_input.trim();
    println!("  Trimmed &str: \"{raw_input}\"");

    // Shadow 2: Change TYPE from &str to u32
    let raw_input: u32 = raw_input.parse().expect("Failed to parse into number");
    println!("  Transformed to u32: {raw_input}");

    // Shadow 3: Scale value
    let raw_input = raw_input * 2;
    println!("  Scaled (u32 * 2): {raw_input}");
}

fn run_interactive_cli() {
    let name = prompt_input("Enter your name (or press enter for 'Explorer'): ");
    let name = if name.is_empty() { "Explorer".to_string() } else { name };

    let current_year = 2026;
    let birth_year_str = prompt_input("Enter your birth year (e.g. 2000): ");
    
    // Expression-based parsing with fallback
    let birth_year: i32 = match birth_year_str.trim().parse() {
        Ok(year) => year,
        Err(_) => {
            println!("  [!] Invalid year entered. Defaulting to 2000.");
            2000
        }
    };

    let age = calculate_age(birth_year, current_year);
    let category = classify_stage(age);

    println!("\n  Hello, {name}!");
    println!("  Calculated Age: {age} years");
    println!("  Life Stage Category: {category}");
}

// Helpers demonstrating Expression-based returns (no 'return' keyword, no trailing semicolon)
fn calculate_age(birth_year: i32, current_year: i32) -> i32 {
    if birth_year > current_year {
        0
    } else {
        current_year - birth_year
    }
}

fn classify_stage(age: i32) -> &'static str {
    match age {
        0..=2 => "Infant",
        3..=12 => "Child",
        13..=19 => "Teenager",
        20..=35 => "Young Adult",
        36..=59 => "Adult",
        _ => "Senior / Veteran",
    }
}

fn prompt_input(message: &str) -> String {
    print!("{message}");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line from stdin");

    buffer.trim().to_string()
}
