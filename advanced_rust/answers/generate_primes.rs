use std::collections::BTreeSet;
/**
Problem: Generates prime numbers in a multi-threaded manner

Implementation
Use threads to check if a number is a prime number
Note: YOU CANNOT USE ANY SORT methods

Pass all the ASSERTIONS
 */
use std::sync::{Arc, Mutex};
use std::thread;

// Function to check if a number is prime
fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n & 1 == 0 {
        return false;
    }
    let sqrt_n = (n as f64).sqrt() as u32;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn find_primes_parallel(range_start: u32, range_end: u32, thread_count: u32) -> String {
    if thread_count > (range_end - range_start) {
        return String::from("Too many threads!");
    }
    let range_per_thread = (range_end - range_start) / thread_count;

    // Create a BTreeSet inside a Mutex, wrapped in an Arc for shared ownership
    let primes_set = Arc::new(Mutex::new(BTreeSet::new()));
    let mut threads = vec![];

    for i in 0..thread_count {
        let start = range_start + i * range_per_thread;
        let end = if i == thread_count - 1 {
            range_end
        } else {
            start + range_per_thread
        };

        // Clone the Arc to get a new reference for the thread
        let primes_set_clone = primes_set.clone();

        threads.push(thread::spawn(move || {
            for n in start..end {
                if is_prime(n) {
                    // Lock the Mutex and add the prime number to the set
                    let mut set = primes_set_clone.lock().unwrap();
                    set.insert(n);
                }
            }
        }));
    }

    // Wait for all threads to complete
    for thread in threads {
        thread.join().unwrap();
    }

    // Access the final set of primes
    let final_primes = primes_set.lock().unwrap();
    return format!("Primes: {:?}", *final_primes);
}

fn main() {
    assert_eq!(find_primes_parallel(2, 100, 4), String::from("Primes: {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97}"));
    assert_eq!(find_primes_parallel(100, 200, 10), String::from("Primes: {101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199}"));
    assert_eq!(find_primes_parallel(200, 300, 100), String::from("Primes: {211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293}"));
    assert_eq!(
        find_primes_parallel(300, 400, 101),
        String::from("Too many threads!")
    );

    println!("Congratulations for succeeding the test!");
}
