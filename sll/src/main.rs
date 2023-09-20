// Define a singly linked list node for integers
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

// Define the singly linked list for integers
pub struct SLL {
    head: Option<Box<Node>>,
}

impl SLL {
    // Create an empty linked list
    pub fn new() -> Self {
        SLL { head: None }
    }

    // Insert an integer at the beginning of the list
    pub fn push(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    // Remove and return the first integer in the list
    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    // Check if the list is empty
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    // Print the integers in the list
    pub fn print(&self) {
        let mut current = &self.head;
        print!("List: ");
        while let Some(node) = current {
            print!("{}, ", node.value);
            current = &node.next;
        }
        println!();
    }
}

fn main() {
    let mut list = SLL::new();

    list.push(1);
    list.push(2);
    list.push(3);

    list.print(); // Output: List: 3, 2, 1,

    let popped_value = list.pop();
    println!("Popped value: {:?}", popped_value); // Output: Popped value: Some(3)

    list.print(); // Output: List: 2, 1,
}
