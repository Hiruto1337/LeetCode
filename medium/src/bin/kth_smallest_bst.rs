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
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None
            })))
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None
            })))
        })))
    })));

    let k = 5;

    println!("{}", kth_smallest(root, k));
}

use std::rc::Rc;
use std::cell::RefCell;

fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    fn traverse(treenode: Option<Rc<RefCell<TreeNode>>>, nums: &mut Vec<i32>) {
        if let Some(node_ref) = treenode {
            let node = node_ref.borrow();

            traverse(node.left.to_owned(), nums);
            nums.push(node.val);
            traverse(node.right.to_owned(), nums);
        }
    }

    let mut nums: Vec<i32> = vec![];

    traverse(root, &mut nums);

    nums[k as usize - 1]
}

// Create an array representing all possible values and their counts
// Iterate from left to right, subtract from k when an instance is met
// k == 0, return i