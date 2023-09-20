fn main() {
    // Creating a variable
    let x = 42;

    // References
    // Immutable reference
    let reference = &x;
    println!("Immutable reference: {}", reference);

    // Mutable reference
    let mut y = 10;
    let mutable_reference = &mut y;
    *mutable_reference += 5; // Changing the value through a mutable reference
    println!("Mutable reference: {}", mutable_reference);

    // Raw Pointers
    // Immutable raw pointer
    let raw_pointer: *const i32 = &x;
    unsafe {
        println!("Immutable raw pointer: {}", *raw_pointer);
    }

    // Mutable raw pointer
    let mut z = 7;
    let mutable_raw_pointer: *mut i32 = &mut z;
    unsafe {
        *mutable_raw_pointer += 3; // Changing the value through a mutable raw pointer
        println!("Mutable raw pointer: {}", *mutable_raw_pointer);
    }

    // Memory Safety and Ownership
    // Rust ensures memory safety through ownership and borrowing
    let a = 100;
    let reference1 = &a;
    let reference2 = &a; // Multiple immutable references are allowed
    println!("References: {} and {}", reference1, reference2);

    // Uncomment the following line to see a compilation error:
    // let mutable_reference2 = &mut a; // Error: Cannot have a mutable reference while there are immutable references

    // Using Pointer Operations
    // Pointer arithmetic can be done safely using methods like `offset` and `wrapping_*`
    let array = [1, 2, 3, 4, 5];
    let raw_array_pointer: *const i32 = &array[0];

    // Using offset to access array elements safely
    unsafe {
        let second_element = raw_array_pointer.offset(1);
        println!("Second element of the array: {}", *second_element);
    }
}
