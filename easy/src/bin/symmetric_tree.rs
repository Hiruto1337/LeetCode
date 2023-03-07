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

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        }))),
    })));

    println!("{}", is_symmetric(root));
}

use std::cell::RefCell;
use std::rc::Rc;
fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn inorder_left(node: Option<Rc<RefCell<TreeNode>>>, log: &mut Vec<i32>) {
        match node {
            Some(node_ref) => {
                let node_handle = node_ref.borrow();
                match node_handle.left.to_owned() {
                    Some(left_node_ref) => {
                        log.push(0);
                        inorder_left(Some(left_node_ref), log);
                    },
                    None => {}
                }

                log.push(node_handle.val);

                match node_handle.right.to_owned() {
                    Some(right_node_ref) => {
                        inorder_left(Some(right_node_ref), log);
                    },
                    None => {}
                }
            },
            None => {}
        }
    }

    fn inorder_right(node: Option<Rc<RefCell<TreeNode>>>, log: &mut Vec<i32>) {
        match node {
            Some(node_ref) => {
                let node_handle = node_ref.borrow();

                match node_handle.right.to_owned() {
                    Some(right_node_handle) => {
                        log.push(0);
                        inorder_right(Some(right_node_handle), log);
                    },
                    None => {}
                }

                log.push(node_handle.val);

                match node_handle.left.to_owned() {
                    Some(left_node_handle) => {
                        inorder_right(Some(left_node_handle), log);
                    },
                    None => {}
                }
            },
            None => {}
        }
    }

    let root = root.unwrap();

    let mut left_nodes: Vec<i32> = vec![];
    let mut right_nodes: Vec<i32> = vec![];

    let left_branch: Option<Rc<RefCell<TreeNode>>> = root.borrow().left.to_owned();
    let right_branch: Option<Rc<RefCell<TreeNode>>> = root.borrow().right.to_owned();

    inorder_left(left_branch, &mut left_nodes);
    inorder_right(right_branch, &mut right_nodes);

    println!("{left_nodes:?}");
    println!("{right_nodes:?}");

    if left_nodes == right_nodes {
        return true;
    } else {
        return false;
    }
}
