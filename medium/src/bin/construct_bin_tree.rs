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
    let preorder: Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10];

    let inorder: Vec<i32> = vec![2,4,3,5,1,6,7,10,9,8];

    build_tree(preorder, inorder);
}

use std::cell::RefCell;
use std::rc::Rc;
pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn build(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        match inorder.len() {
            0 => None,
            1 => Some(Rc::new(RefCell::new(TreeNode {
                val: inorder[0],
                left: None,
                right: None,
            }))),
            _ => {
                let index = inorder.iter().position(|&num| num == preorder[0]).unwrap();
    
                let (left, right) = inorder.split_at(index);
                let (left, right) = (left.to_vec(), right[1..].to_vec());
        
                Some(Rc::new(RefCell::new(TreeNode {
                    val: preorder[0],
                    left: build(preorder[1..index + 1].to_vec(), left),
                    right: build(preorder[index + 1..].to_vec(), right),
                })))
            }
        }
    }

    build(preorder, inorder)
}
