fn main() {
    // Creating a vector of numbers
    let numbers = vec![1, 2, 3, 4, 5];

    // Using a for loop to iterate over elements directly
    println!("Using a for loop:");
    for i in 0 .. numbers.len() {
        print!("{} ", numbers[i]);
    }
    println!();

    // Using a while loop
    println!("Using a while loop:");
    let mut i = 0;
    while i < numbers.len() {
        print!("{} ", numbers[i]);
        i += 1;
    }
    println!();

    // Creating an iterator
    let mut iterator = numbers.iter();

    // Looping over elements using the iterator
    println!("Looping over elements using an iterator:");
    for number in iterator {
        print!("{} ", number);
    }
    println!();

    // Using iterator methods: map
    // The `map` method transforms each element in the iterator using the closure provided.
    // Here, we double each number and collect the results into a new vector.
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    // Using iterator methods: filter
    // The `filter` method creates a new iterator that contains only the elements
    // that satisfy the given condition in the closure.
    // We filter and clone even numbers into a new vector.
    let evens: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).cloned().collect();
    println!("Evens: {:?}", evens);

    // Using iterator methods: fold
    // The `fold` method accumulates values in an iterator into a single result.
    // It takes an initial accumulator value and applies the closure to each element.
    // Here, we calculate the sum of all numbers in the vector.
    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("Sum: {}", sum);

    // Using iterator methods: any and all
    // The `any` and `all` methods check if any or all elements in the iterator satisfy
    // a given condition in the closure.
    let any_greater_than_five = numbers.iter().any(|&x| x > 5);
    println!("Any greater than 5: {}", any_greater_than_five);
}
