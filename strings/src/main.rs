fn main() {
    // Creating a string literal
    let greeting = "Hello, Rust!";
    println!("String Literal: {}", greeting);

    // Creating a String type
    let mut dynamic_string = String::from("Hello, ");
    dynamic_string.push_str("Rust!");
    println!("String Type: {}", dynamic_string);

    // Concatenation
    let hello = "Hello";
    let world = "World";
    let greeting_concatenation = hello.to_string() + ", " + world;
    println!("Concatenation: {}", greeting_concatenation);

    // Concatenation with format! macro
    let greeting_format = format!("{}, {}", hello, world);
    println!("Concatenation with format! macro: {}", greeting_format);

    // String length
    let message = "This is a message.";
    let length = message.len();
    println!("String Length: {}", length);

    // Slicing
    let text = "Rust Programming";
    let slice = &text[0..4]; // "Rust"
    println!("Sliced Text: {}", slice);

    // Iterating over characters
    let text_to_iterate = "Rust";
    println!("Iterating Over Characters:");
    for c in text_to_iterate.chars() {
        println!("{}", c);
    }

    // String Ownership
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2
    // Printing s2 as s1 cannot be used anymore due to ownership transfer
    println!("String Ownership: {}", s2);

    // String Mutability
    let mut mutable_string = String::from("Change me!");
    mutable_string.push_str(" Now I'm different.");
    println!("Mutated String: {}", mutable_string);
}
