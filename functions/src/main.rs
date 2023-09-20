// Define a function to add two numbers
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Define a function to subtract two numbers
fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

// Define a function to multiply two numbers
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// Define a function to divide two numbers
fn divide(a: i32, b: i32) -> Result<f32, String> {
    if b == 0 {
        return Err(String::from("Division by zero is not allowed"));
    }
    Ok(a as f32 / b as f32)
}

// Define a function with default parameters
fn greet(name: &str, greeting: &str) {
    println!("{} says: {}", name, greeting);
}

fn main() {
    // Call the add function
    let result_add = add(5, 3);
    println!("5 + 3 = {}", result_add);

    // Call the subtract function
    let result_subtract = subtract(10, 4);
    println!("10 - 4 = {}", result_subtract);

    // Call the multiply function
    let result_multiply = multiply(6, 7);
    println!("6 * 7 = {}", result_multiply);

    // Call the divide function
    match divide(8, 2) {
        Ok(result_divide) => println!("8 / 2 = {}", result_divide),
        Err(err) => println!("Error: {}", err),
    }

    // Call the greet function with default parameters
    greet("Alice", "Hello!"); // Alice says: Hello!
    greet("Bob", "Hi!");     // Bob says: Hi!
}
