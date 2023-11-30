/**
This code has a clone call to avoid ownership conflicts
self.posts.iter().find(|p| p.title == title).cloned()

The goal of the problem is to use lifetimes to get rid of the cloned() call.
*/
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
struct Post {
    title: String,
    timestamp: u64
}

struct User {
    name: String,
    posts: Vec<Post>,
}

impl User {
    fn new(name: String) -> Self {
        User { name, posts: Vec::new() }
    }

    fn add_post(&mut self, title: String) {
        if let Ok(duration_since_epoch) = SystemTime::now().duration_since(UNIX_EPOCH) {
            let timestamp = duration_since_epoch.as_secs();
            self.posts.push(Post { title, timestamp });
        } else {
            // Handle the unlikely case of time going backwards (before Unix epoch)
            panic!("SystemTime before UNIX EPOCH!");
        }
    }

    // Here, we clone the post to avoid ownership issues
    fn get_post_by_title(&self, title: &str) -> Option<Post> {
        self.posts.iter().find(|p| p.title == title).cloned()
    }
}

fn main() {
    let mut user = User::new("Harry".to_string());
    user.add_post("My Rust Journey".to_string());
    user.add_post("My Journey to the center of Earth".to_string());
    user.add_post("My trip to Jumanji".to_string());

    if let Some(post) = user.get_post_by_title("My Rust Journey") {
        println!("Found post: {} uploaded on: {}", post.title, post.timestamp);
    }
}
