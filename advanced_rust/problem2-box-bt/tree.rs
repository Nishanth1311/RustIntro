/**
Main Problem: Implement a Binary Tree with Box and print pre-order, in-order, post-order and level order traversals
Side quest: Build a Binary Search Tree instead of Binary Tree

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