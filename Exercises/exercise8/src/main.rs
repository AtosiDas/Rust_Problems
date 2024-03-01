//Given a binary tree, implement a function that returns the maximum depth of the tree.

// Definition of a binary tree node
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    // Constructor function to create a new binary tree node
    pub fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        None => 0, // Base case: depth of an empty tree is 0
        Some(node) => {
            // Recursively calculate the depth of the left and right subtrees
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);
            // Return the maximum depth of the left and right subtrees, plus 1 for the current node
            1 + left_depth.max(right_depth)
        }
    }
}

fn main() {
    // Example binary tree:
    //      3
    //     / \
    //    9  20
    //      /  \
    //     19   7
    //    /  \   
    //  25    5
    let root = Some(Box::new(TreeNode {
        val: 3,
        left: Some(Box::new(TreeNode::new(9))),
        right: Some(Box::new(TreeNode {
            val: 20,
            left: Some(Box::new(TreeNode {
                val: 19,
                left: Some(Box::new(TreeNode::new(25))),
                right:Some(Box::new(TreeNode::new(5))),
            })),
            right: Some(Box::new(TreeNode::new(7))),
        })),
    }));

    println!("Maximum depth of the binary tree: {}", max_depth(root));
}
