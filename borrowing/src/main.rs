fn main() {
    // You own a car
    let mut your_car = String::from("Your Car");

    // Your friend wants to borrow your car and modify it
    let borrowed_car = &mut your_car;

    // Your friend can modify it temporarily
    borrowed_car.push_str("'s Color");

    // You can't access your car while your friend has borrowed it
    // Uncommenting the following line would result in a compilation error:
    // your_car.push_str("'s Model"); // Error: Cannot borrow `your_car` as mutable because it is also borrowed as mutable
    println!("Your friend is modifying: {}", borrowed_car);

    // Once your friend is done, you can use it again
    your_car.push_str("'s Model");
    println!("You own: {}", your_car);
}
