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
        val: 90,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 69,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 49,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 52,
                    left: None,
                    right: None
                })))
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 89,
                left: None,
                right: None
            })))
        }))),
        right: None
    })));

    println!("{}", min_diff_in_bst(root));
}

use std::rc::Rc;
use std::cell::RefCell;

fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn traverse(treenode: Option<Rc<RefCell<TreeNode>>>, array: &mut Vec<i32>) {
        if let Some(node_ref) = treenode {
            let node = node_ref.borrow();

            array.push(node.val);

            traverse(node.left.to_owned(), array);
            traverse(node.right.to_owned(), array);
        }
    }

    let mut array = vec![];

    traverse(root, &mut array);

    array.sort();

    let mut minimum = i32::MAX;

    for pair in array.windows(2) {
        minimum = minimum.min((pair[0] - pair[1]).abs());
    }

    minimum
}