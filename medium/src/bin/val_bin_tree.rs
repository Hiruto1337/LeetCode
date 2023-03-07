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

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: None,
                right: None,
            }))),
        }))),
    })));

    println!("{}", is_valid_bst(root));
}

fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn traverse(treenode: Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        match treenode {
            Some(node) => {
                let node = node.borrow();

                let val = node.val as i64;

                if val <= min || max <= val {
                    return false;
                }

                let left_tree_valid = traverse(node.left.to_owned(), min, val);

                let right_tree_valid = traverse(node.right.to_owned(), val, max);

                left_tree_valid && right_tree_valid
            }
            None => true,
        }
    }

    traverse(root, i64::MIN, i64::MAX)
}
