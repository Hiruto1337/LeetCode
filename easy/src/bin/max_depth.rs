// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

fn main() {
    // let root = Some(Rc::new(RefCell::new(TreeNode {
    //     val: 3,
    //     left: Some(Rc::new(RefCell::new(TreeNode {
    //         val: 2,
    //         left: None,
    //         right: None
    //     }))),
    //     right: Some(Rc::new(RefCell::new(TreeNode {
    //         val: 20,
    //         left: Some(Rc::new(RefCell::new(TreeNode {
    //             val: 15,
    //             left: None,
    //             right: None,
    //         }))),
    //         right: Some(Rc::new(RefCell::new(TreeNode {
    //             val: 7,
    //             left: None,
    //             right: None,
    //         }))),
    //     }))),
    // })));
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 69,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 420,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 69,
                left: None,
                right: None
            })))
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 10,
            left: None,
            right: None
        })))
    })));

    println!("{}", max_depth(root));
}

use std::rc::Rc;
use std::cell::RefCell;
fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn get_depth(node: Option<Rc<RefCell<TreeNode>>>, depth: i32) -> i32 {
        if let Some(node_ref) = node {

            let node_borrow = node_ref.borrow();

            let left_depth = get_depth(node_borrow.left.to_owned(), depth + 1);
            let right_depth = get_depth(node_borrow.right.to_owned(), depth + 1);

            return depth.max(left_depth.max(right_depth));
        }

        0
    }

    get_depth(root, 1)
}

// Initialize depth to 0
// Recursively go down one layer and give the new depth to the function