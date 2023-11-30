/**
Main Problem: Implement a Binary Search Tree with Box and print pre-order, in-order, post-order and level order traversals

Objective:
Create a binary tree data structure.
Use Box for pointers to the child nodes (left and right).

Tasks:
Define a Node struct containing the value and pointers to left and right children using Box.
Implement a BinaryTree struct that holds the root of the tree.
Provide functions to insert elements into the tree.
Ensure that the tree remains balanced after each insertion.
Learning Outcomes:

Understanding the use of Box for heap allocation.
Grasping the concepts of ownership and borrowing in Rust.
Practical experience with recursive data structures.
*/

// Define the structure of a tree node
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

/**
 * Implement your code below for insertion, pre-order, in-order, post-order and level order traversals
 */
impl TreeNode {
    fn new(value: i32) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

struct BinaryTree {
    root: Option<Box<TreeNode>>,
}

impl BinaryTree {
    fn new() -> Self {
        BinaryTree { root: None }
    }

    fn insert(&mut self, value: i32) {
        let new_node = Box::new(TreeNode::new(value));
        match self.root {
            None => {
                self.root = Some(new_node);
            }
            Some(ref mut node) => {
                Self::insert_node(node, new_node);
            }
        }
    }

    fn insert_node(node: &mut Box<TreeNode>, new_node: Box<TreeNode>) {
        // if we want to insert to left subtree
        if new_node.value < node.value {
            match node.left {
                None => node.left = Some(new_node),
                Some(ref mut left) => {
                    Self::insert_node(left, new_node);
                }
            }
        } else {
            match node.right {
                None => node.right = Some(new_node),
                Some(ref mut right) => {
                    Self::insert_node(right, new_node);
                }
            }
        }
    }
}

impl BinaryTree {
    fn pre_order_traversal(&self) {
        print!("Pre-order traversal: ");
        Self::pre_order_helper(&self.root);
        println!();
    }

    fn pre_order_helper(node: &Option<Box<TreeNode>>) {
        if let Some(ref box_node) = node {
            print!("{} ", box_node.value);
            Self::pre_order_helper(&box_node.left);
            Self::pre_order_helper(&box_node.right);
        }
    }

    fn in_order_traversal(&self) {
        print!("In-order traversal: ");
        Self::in_order_helper(&self.root);
        println!();
    }

    fn in_order_helper(node: &Option<Box<TreeNode>>) {
        if let Some(ref box_node) = node {
            Self::in_order_helper(&box_node.left);
            print!("{} ", box_node.value);
            Self::in_order_helper(&box_node.right);
        }
    }

    fn post_order_traversal(&self) {
        print!("Post-order traversal: ");
        Self::post_order_helper(&self.root);
        println!();
    }

    fn post_order_helper(node: &Option<Box<TreeNode>>) {
        if let Some(ref box_node) = node {
            Self::post_order_helper(&box_node.left);
            Self::post_order_helper(&box_node.right);
            print!("{} ", box_node.value);
        }
    }
}
fn main() {
    let mut tree = BinaryTree::new();

    // Inserting some values into the tree
    tree.insert(10);
    tree.insert(5);
    tree.insert(15);
    tree.insert(3);
    tree.insert(7);
    tree.insert(12);
    tree.insert(18);

    // Printing the different types of traversals
    tree.pre_order_traversal();
    tree.in_order_traversal();
    tree.post_order_traversal();
}
