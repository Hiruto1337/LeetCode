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
    let nums: Vec<i32> = vec![-10, -3, 0, 5, 9];

    sorted_array_to_bst(nums);
}

use std::cell::RefCell;
use std::rc::Rc;
pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn create_branches(array: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if array.len() == 0 {
            return None;
        } else {
            let root_index = array.len() / 2;

            let root_val = array[root_index];

            let branch_slices: Vec<&[i32]> = array.split(|&element| element == root_val).collect();

            let mut branches: Vec<Vec<i32>> = vec![];
        
            for branch in branch_slices {
                branches.push(branch.to_vec());
            }

            Some(Rc::new(RefCell::new(TreeNode {
                val: root_val,
                left: create_branches(branches[0].to_owned()),
                right: create_branches(branches[1].to_owned())
            })))
        }
    }

    let root = create_branches(nums);

    root
}
