#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
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

    println!("{:?}", preorder_traversal(root));
}

use std::cell::RefCell;
use std::rc::Rc;

pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn fill_order(root: Option<Rc<RefCell<TreeNode>>>, array: &mut Vec<i32>) {
        match root {
            Some(node_ref) => {
                let node_borrow = node_ref.borrow();
                (*array).push(node_borrow.val);
                fill_order(node_borrow.left.to_owned(), array);
                fill_order(node_borrow.right.to_owned(), array);
            },
            None => {}
        }
    }

    let mut order: Vec<i32> = vec![];

    fill_order(root, &mut order);

    order
}
