fn main() {
    let x = 10;
    let y = x;
    println!("x is {x} and y is {y}");

    let mut s1 = String::from("hello");
    let s2 = &s1;
    println!("s1 is {s1} and s2 is {s2}");
    takes_os(&s1);
    println!("{s1}");
    add_word(&mut s1);
    println!("s2 is still: {s1}");
    demonstrate_slices();
}


fn takes_os(some_string: &String) {
    println!("The string is {some_string}")
}


fn add_word(some_string: &mut String) {
    some_string.push_str(" world");
    println!("Modified: {some_string}");
}   

fn demonstrate_slices() {
    let message = String::from("Rustacean Power");
    let first_word = &message[0..9];
    let second_word = &message[10..15];
    println!("{first_word} and {second_word}");
}