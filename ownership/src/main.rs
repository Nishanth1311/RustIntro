fn main() {
    // person1 owns a string
    let mut person1: String = String::from("I own Coca-Cola");

    // person2 clones what person1 has and appends " new"
    let mut person2: String = person1.clone() + " new";

    // Now, person1 and person2 both have coca-cola
    println!("Person1: {}", person1); // Output: Person1: I own Coca-Cola
    println!("Person2: {}", person2); // Output: Person2: I own Coca-Cola new
    
    // person1 loses his business as everyone goes to Person2
    person1.clear();

    // Now, person1 does not own anything and person2 owns coca-cola new
    println!("Person1: {}", person1); // Output: Person1: (owns nothing)
    println!("Person2: {}", person2); // Output: Person2: I own Coca-Cola new
    

    // person1 is mad and takes ownership back
    person1 = person2.clone();

    // person2 clears their string
    person2.clear();

    // Now, person1 has the modified string, and person2's string is cleared
    println!("Person1: {}", person1); // Output: Person1: I own Coca-Cola new
    println!("Person2: {}", person2); // Output: Person2: (owns nothing)
}
