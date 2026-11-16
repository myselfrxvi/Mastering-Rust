// ====================================================================
// Module 03: Structs, Methods & Enums with Pattern Matching
// ====================================================================

// --------------------------------------------------------------------
// 1. Enum Definition (Algebraic Data Types: Each variant can hold data!)
// --------------------------------------------------------------------
#[derive(Debug)]
enum Membership {
    Free,
    Premium { days_remaining: u32 }, // variant holding struct data
    VIP(String),                     // variant holding a tuple string
}

// --------------------------------------------------------------------
// 2. Struct Definition & Debug Derivation
// --------------------------------------------------------------------
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    age: u8,
    sign_incount: u64,
    phone_number: Option<String>,
    membership: Membership,
}

// --------------------------------------------------------------------
// 3. Methods & Associated Functions (impl Block)
// --------------------------------------------------------------------
impl User {
    // Associated function (Constructor): returns Self
    fn new(username: &str, email: &str, age: u8, membership: Membership) -> Self {
        Self {
            username: username.to_string(),
            email: email.to_string(),
            age,
            sign_incount: 1,
            phone_number: None,
            membership,
        }
    }

    // Method borrowing &self (read-only access)
    fn display_profile(&self) {
        println!("👤 {} ({}) - Age: {}", self.username, self.email, self.age);
        if let Some(phone) = &self.phone_number {
            println!("   └─ Phone: {phone}");
        } else {
            println!("   └─ Phone: [Not provided]");
        }
        print_membership_benefits(&self.membership);
    }

    // Method borrowing &mut self (mutates the instance)
    fn celebrate_birthday(&mut self) {
        self.age += 1;
        println!("🎉 Happy Birthday, {}! You are now {} years old.", self.username, self.age);
    }
}

// --------------------------------------------------------------------
// 4. Exhaustive Pattern Matching with `match`
// --------------------------------------------------------------------
fn print_membership_benefits(membership: &Membership) {
    match membership {
        Membership::Free => {
            println!("   └─ Tier: Free (Standard access, ads enabled)");
        }
        Membership::Premium { days_remaining } => {
            println!("   └─ Tier: Premium ({days_remaining} days of ad-free streaming remaining)");
        }
        Membership::VIP(access_code) => {
            println!("   └─ Tier: VIP Concierge active with code [{access_code}]");
        }
    }
}

// --------------------------------------------------------------------
// 5. Main Application Flow
// --------------------------------------------------------------------
fn main() {
    println!("==================================================");
    println!("     🦀 Module 03: Structs, Enums & Matching      ");
    println!("==================================================");

    // Creating User 1 with Free tier
    println!("\n[1] Creating User 1 (Alice):");
    let mut user1 = User::new("alice123", "alice@example.com", 32, Membership::Free);
    user1.display_profile();

    println!("\n[2] Celebrating Alice's Birthday (Mutation via &mut self):");
    user1.celebrate_birthday();
    user1.display_profile();

    // Creating User 2 with VIP tier
    println!("\n[3] Creating User 2 with VIP Membership (Bruce Wayne):");
    let mut user2 = User::new(
        "Bruce Wayne",
        "bruce@wayne-enterprises.com",
        35,
        Membership::VIP(String::from("BATCAVE-ALPHA-9")),
    );
    user2.phone_number = Some(String::from("+1-555-BATMAN"));
    user2.display_profile();

    // Creating User 3 with Premium tier
    println!("\n[4] Creating User 3 with Premium Membership (Clark Kent):");
    let user3 = User::new(
        "Clark Kent",
        "clark@dailyplanet.com",
        33,
        Membership::Premium { days_remaining: 365 },
    );
    user3.display_profile();

    println!("\n[5] Debug Printing Entire User Struct:");
    println!("{:#?}", user3);
    println!("==================================================");
}
