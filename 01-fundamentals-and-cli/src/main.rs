use std::io;

fn main() {
    println!("==================================================");
    println!("       🦀 Module 01: Price Calculator CLI         ");
    println!("==================================================");

    // 1. Prompt user for item price using a loop with error handling
    let price: i32 = loop {
        println!("Enter the item price:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Trim newline and attempt parsing
        let input = input.trim();

        // Pattern match: either break with the valid number or ask again
        match input.parse() {
            Ok(num) => break num,
            Err(_) => println!(" [!] That's not a valid number. Please try again!\n"),

        }
    
    };

    // 2. Quantity (fixed for now, or easily expanded)
    let quantity = 5;

    // 3. Compute total using a custom expression-based function
    let total = calculate_total(price, quantity);

    // 4. Compute discount using an expression-based if/else
    let discount = if total > 100 {
        20
    } else {
        5
    };

    // 5. Compute final price
    let final_price = total - discount;
    // 6. Print summary
    println!("\n---------------- Summary ----------------");
    println!("Unit Price:  ${price}");
    println!("Quantity:    {quantity}");
    println!("Subtotal:    ${total}");
    println!("Discount:   -${discount}");
    println!("-----------------------------------------");
    println!("Final Price: ${final_price}");
    println!("=========================================");
    demonstrate_for_loop();
    demonstrate_while_loop();
    demonstrate_loop_labels();
}

// Helper function: returns price * quantity (expression-based return)
fn calculate_total(price: i32, quantity: i32) -> i32 {
    price * quantity
}

fn demonstrate_for_loop() {
    for count in (1..=100).rev() {
        println!("{count}...");
    }
    println!("Blast off!!");
}

fn demonstrate_while_loop() {
    let mut battery = 0;
    while battery < 100 {
        battery += 25;
        println!("charging... {battery}%");
    }
    println!("Phone charged.")
}

fn demonstrate_loop_labels() {
    let mut outer_count = 0;

    'outer: loop {
        println!("Outer loop: {outer_count}");
        let mut inner_count = 0;

        loop {
            println!("Inner loop: {inner_count}");
            if inner_count == 2 {
                break;
            }
            if outer_count == 1{
                println!("Target reached!");
                break 'outer;
            }
            inner_count += 1;
        }
        outer_count += 1;
        }
        println!("Loop finished.");
    }