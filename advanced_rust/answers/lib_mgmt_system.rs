/**
Problem: Library Management System

Objective:
Create a system to manage books in a library, including adding, removing, and checking out books.

Implementation
Define Structs and Enums:

Book: A struct representing a book with fields like title, author, and a unique identifier.
Library: A struct representing the library, containing a collection of books.
LibraryAction: An enum representing actions that can be performed on the library (e.g., AddBook, RemoveBook, CheckoutBook).

Library Management Logic:
AddBook returns:
AddBookPassed if there is enough space in the library
AddBookFailed if there is not enough space in the library

RemoveBookByName returns:
RemoveBookPassed if the book is available in the library.
RemoveBookFailed if the book is not available in the library.

CheckoutBookByName returns:
CheckoutBookPassed if the book is available in the library.
CheckoutBookFailed if the book is not available in the library.

Pass all the ASSERTIONS

Implement methods on the Library struct to handle different actions, such as adding or removing books.
 */
use std::collections::HashMap;

#[derive(Clone)]
struct Book {
    id: u32,
    title: String,
    author: String,
    count: u32,
}

#[derive(PartialEq, Debug)]
enum ActionStatus {
    AddBookPassed,
    AddBookFailed,
    RemoveBookPassed,
    RemoveBookFailed,
    CheckoutBookPassed,
    CheckoutBookFailed,
}

enum LibraryAction {
    AddBook(Book),
    // RemoveBookById(u32),
    RemoveBookByName(String),
    // CheckoutBookById(u32),
    CheckoutBookByName(String),
}

struct Library {
    books: HashMap<String, Book>,
    size: u32,
}

impl Library {
    fn new() -> Self {
        Library {
            books: HashMap::new(),
            size: 100,
        }
    }

    fn perform_action(&mut self, action: LibraryAction) -> ActionStatus {
        match action {
            LibraryAction::AddBook(book) => {
                println!("Adding book: {}", book.title);
                if book.count > self.size {
                    return ActionStatus::AddBookFailed;
                }
                
                self.size -= book.count;
                if let Some(existing_book) = self.books.get_mut(&book.title) {
                    existing_book.count += book.count;
                } else {
                    self.books.insert(book.title.clone(), book);
                }
                return ActionStatus::AddBookPassed;
            }
            LibraryAction::RemoveBookByName(book_title) => {
                println!("Removing book: {}", book_title);
                if let Some(book) = self.books.get_mut(&book_title) {
                    if book.count > 0 {
                        self.size += book.count;
                        book.count = 0;
                        return ActionStatus::RemoveBookPassed;
                    }
                }
                return ActionStatus::RemoveBookFailed;
            }
            LibraryAction::CheckoutBookByName(book_title) => {
                if let Some(book) = self.books.get_mut(&book_title) {
                    if book.count > 0 {
                        book.count -= 1;
                        println!(
                            "Checking out book: {}. Books left: {}",
                            book_title, book.count
                        );
                        return ActionStatus::CheckoutBookPassed;
                    }
                }
                return ActionStatus::CheckoutBookFailed;
            }
        }
    }
}

fn main() {
    let mut library = Library::new();

    // Test adding a book
    let book1 = Book {
        id: 1,
        title: "Rust Programming".to_string(),
        author: "John Doe".to_string(),
        count: 25,
    };
    let book2 = Book {
        id: 2,
        title: "Harry Potter".to_string(),
        author: "J K Rowling".to_string(),
        count: 35,
    };
    let book3 = Book {
        id: 3,
        title: "Game of Thrones".to_string(),
        author: "George R R Martin".to_string(),
        count: 30,
    };

    assert_eq!(
        library.perform_action(LibraryAction::AddBook(book1.clone())),
        ActionStatus::AddBookPassed
    );
    assert_eq!(
        library.perform_action(LibraryAction::AddBook(book2.clone())),
        ActionStatus::AddBookPassed
    );
    assert_eq!(
        library.perform_action(LibraryAction::AddBook(book3.clone())),
        ActionStatus::AddBookPassed
    );
    assert_eq!(
        library.perform_action(LibraryAction::AddBook(book2.clone())),
        ActionStatus::AddBookFailed
    );
    assert_eq!(
        library.perform_action(LibraryAction::RemoveBookByName(book3.title.clone())),
        ActionStatus::RemoveBookPassed
    );
    assert_eq!(
        library.perform_action(LibraryAction::RemoveBookByName(book3.title.clone())),
        ActionStatus::RemoveBookFailed
    );
    assert_eq!(
        library.perform_action(LibraryAction::AddBook(book2.clone())),
        ActionStatus::AddBookPassed
    );

    for _counter in 0..book1.count as usize {
        assert_eq!(
            library.perform_action(LibraryAction::CheckoutBookByName(book1.title.clone())),
            ActionStatus::CheckoutBookPassed
        );
    }
    assert_eq!(
        library.perform_action(LibraryAction::CheckoutBookByName(book1.title.clone())),
        ActionStatus::CheckoutBookFailed
    );

    println!("All library actions performed successfully!");
}
