fn main() {
    let root: Option<Rc<RefCell<TreeNode>>> =
        Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None
                })))
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 8,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 9,
                        left: None,
                        right: None
                    })))
                }))),
            })))
        })));
    
    println!("{:?}", inorder_traversal(root));
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn evaluate_node(root: Option<Rc<RefCell<TreeNode>>>, traversal_log: &mut Vec<i32>) {
        match root {
            Some(node_ref) => {
                let node = node_ref.borrow();
                match node.left.to_owned() {
                    Some(left_node_ref) => {
                        evaluate_node(Some(left_node_ref), traversal_log);
                    },
                    None => {}
                }
    
                traversal_log.push(node.val);
    
                match node.right.to_owned() {
                    Some(right_node_ref) => {
                        evaluate_node(Some(right_node_ref), traversal_log);
                    },
                    None => {}
                }
            },
            None => {}
        }
    }

    let mut traversal_log: Vec<i32> = vec![];

    evaluate_node(root, &mut traversal_log);

    traversal_log
}
