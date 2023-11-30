/**
This code has a clone call to avoid ownership conflicts
self.posts.iter().find(|p| p.title == title).cloned()

The goal of the problem is to use lifetimes to get rid of the cloned() call.
*/
use std::time::{SystemTime, UNIX_EPOCH};

struct Post<'a> {
    title: &'a str,
    timestamp: u64,
}

struct User<'a> {
    name: &'a str,
    posts: Vec<Post<'a>>,
}

impl<'a> User<'a> {
    fn new(name: &'a str) -> Self {
        User { name, posts: Vec::new() }
    }

    fn add_post(&mut self, title: &'a str) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH")
            .as_secs();

        self.posts.push(Post { title, timestamp });
    }

    fn get_post_by_title(&self, title: &str) -> Option<&Post> {
        self.posts.iter().find(|p| p.title == title)
    }
}

fn main() {
    let mut user = User::new("Harry");
    user.add_post("My Rust Journey");
    user.add_post("My Journey to the center of Earth");
    user.add_post("My trip to Jumanji");

    if let Some(post) = user.get_post_by_title("My Rust Journey") {
        println!("Found post: {} uploaded on: {}", post.title, post.timestamp);
    }
}
