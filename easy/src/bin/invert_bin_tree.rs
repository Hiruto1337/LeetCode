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
      right: None
    }
  }
}

fn main(){
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None
                })))
            }))),
            right: None
        })))
    })));

    invert_tree(root);
}

use std::rc::Rc;
use std::cell::RefCell;
fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node_ref) = root {
        let new_left = self::invert_tree(node_ref.borrow().right.to_owned());
        let new_right = self::invert_tree(node_ref.borrow().left.to_owned());

        node_ref.borrow_mut().left = new_left;
        node_ref.borrow_mut().right = new_right;

        return Some(node_ref);
    }

    None
}

// Stand at a node
// Switch the left and right
// Go to left node
// Go to right node