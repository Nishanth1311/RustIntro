// Define a custom struct
#[derive(Debug)]    //if not added throws error 
struct Person {
    name: String,
    age: u32,
}

fn main() {
    // Creating a box containing an integer
    let boxed_integer = Box::new(42);

    // Dereferencing the box to access the integer value
    println!("Boxed integer: {}", *boxed_integer);

    // Creating a custom struct
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // Creating a box containing the custom struct
    let boxed_person = Box::new(person);

    // Dereferencing the box to access the custom struct
    println!("Boxed person: {:?}", *boxed_person);

    // Automatic Cleanup
    // When boxes go out of scope, Rust automatically releases the heap-allocated memory.

    // Boxes with Custom Types
    // Boxes can store custom structs as well.

    // When to Use Boxes
    // - When you need to store data on the heap with clear ownership semantics.
    // - When you want to transfer ownership of data between functions.
    // - When you're working with recursive data structures that need a known size at compile time.
    // - When you have large data that you want to avoid copying between function calls.
}
