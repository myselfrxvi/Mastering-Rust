// ====================================================================
// Module 04: Collections (Vec, HashMap) & Robust Error Handling (?)
// ====================================================================

use std::collections::HashMap;

// --------------------------------------------------------------------
// 1. Mini-Project: Student Gradebook System
//    Combines HashMap<String, Vec<u32>> with Result & ? propagation
// --------------------------------------------------------------------
#[derive(Debug, Default)]
struct GradeBook {
    // Maps each student's name to a dynamic Vector of exam scores
    records: HashMap<String, Vec<u32>>,
}

impl GradeBook {
    fn new() -> Self {
        Self {
            records: HashMap::new(),
        }
    }

    // Add a score from raw text, validating and parsing with the `?` operator
    fn add_score(&mut self, student: &str, raw_score: &str) -> Result<(), String> {
        // Use ? to propagate parsing failure, converting the error to a String
        let score: u32 = raw_score
            .trim()
            .parse::<u32>()
            .map_err(|_| format!("'{raw_score}' is not a valid integer score"))?;

        if score > 100 {
            return Err(format!("Score {score} exceeds maximum possible of 100"));
        }

        // Use entry().or_default() to find or create the student's score Vector, then push!
        self.records
            .entry(student.to_string())
            .or_default()
            .push(score);

        Ok(())
    }

    // Calculate student's average score
    fn calculate_average(&self, student: &str) -> Result<f64, String> {
        let scores = self
            .records
            .get(student)
            .ok_or_else(|| format!("Student '{student}' not found in records"))?;

        if scores.is_empty() {
            return Err(format!("Student '{student}' has no scores recorded"));
        }

        let sum: u32 = scores.iter().sum();
        let avg = sum as f64 / scores.len() as f64;
        Ok(avg)
    }

    // Print all students and their recorded scores
    fn print_all(&self) {
        println!("\n📋 Current GradeBook Records:");
        for (student, scores) in &self.records {
            println!("  👤 {student}: {scores:?}");
        }
    }
}

// --------------------------------------------------------------------
// 2. Standalone Concept Demonstrations
// --------------------------------------------------------------------
fn demonstrate_vectors() {
    println!("\n--- [A] Vectors (Vec<T>) ---");
    let mut numbers = vec![10, 20, 30];
    numbers.push(40);

    // Mutating elements in-place with &mut and dereferencing *
    for num in &mut numbers {
        *num += 5;
    }
    println!("  Scores after +5 bonus: {numbers:?}");

    // Safe retrieval with .get() -> Option
    let safe_item = numbers.get(1); // Some(&25)
    let out_of_bounds = numbers.get(99); // None
    println!("  Index 1: {safe_item:?}, Index 99: {out_of_bounds:?}");
}

fn demonstrate_hashmaps() {
    println!("\n--- [B] HashMaps & Word Frequency ---");
    let text = "rust is fast rust is safe rust is productive";
    let mut frequencies = HashMap::new();

    for word in text.split_whitespace() {
        let count = frequencies.entry(word).or_insert(0);
        *count += 1;
    }
    println!("  Word Frequencies: {frequencies:#?}");
}

fn parse_and_double(raw_text: &str) -> Result<i32, std::num::ParseIntError> {
    let number = raw_text.trim().parse::<i32>()?;
    Ok(number * 2)
}

// --------------------------------------------------------------------
// 3. Main Application Flow
// --------------------------------------------------------------------
fn main() {
    println!("==================================================");
    println!("    🦀 Module 04: Collections & Error Handling    ");
    println!("==================================================");

    // 1. Concept Demos
    demonstrate_vectors();
    demonstrate_hashmaps();

    println!("\n--- [C] Error Propagation with '?' ---");
    println!("  parse_and_double(\" 25 \")  => {:?}", parse_and_double(" 25 "));
    println!("  parse_and_double(\"invalid\") => {:?}", parse_and_double("invalid"));

    // 2. Running GradeBook Mini-Project
    println!("\n--- [D] GradeBook System in Action ---");
    let mut gradebook = GradeBook::new();

    // Adding valid scores
    let _ = gradebook.add_score("Alice", "90");
    let _ = gradebook.add_score("Alice", "95");
    let _ = gradebook.add_score("Bob", "82");
    let _ = gradebook.add_score("Bob", "88");

    // Handling errors with ? and match
    match gradebook.add_score("Charlie", "one-hundred") {
        Ok(_) => println!("  Added Charlie's score!"),
        Err(e) => println!("  [Expected Error Caught]: {e}"),
    }

    match gradebook.add_score("Charlie", "150") {
        Ok(_) => println!("  Added Charlie's score!"),
        Err(e) => println!("  [Expected Error Caught]: {e}"),
    }

    gradebook.print_all();

    // Calculate averages
    match gradebook.calculate_average("Alice") {
        Ok(avg) => println!("\n  🎯 Alice's Average: {avg:.2}%"),
        Err(e) => println!("  Error: {e}"),
    }

    match gradebook.calculate_average("UnknownStudent") {
        Ok(avg) => println!("  Average: {avg:.2}%"),
        Err(e) => println!("  [Expected Error Caught]: {e}"),
    }

    println!("==================================================");
}
