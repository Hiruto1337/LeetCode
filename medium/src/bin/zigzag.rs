#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

fn main(){
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: None
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 15,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None
            })))
        })))
    })));

    println!("{:?}", zigzag_level_order(root));
}

use std::rc::Rc;
use std::cell::RefCell;
fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    fn traversal(treenode: Option<Rc<RefCell<TreeNode>>>, depth: usize, order: &mut Vec<Vec<i32>>) {
        if let Some(node) = treenode {
            let node = node.borrow();

            match order.get(depth) {
                Some(_) => {
                    (*order)[depth].push(node.val);
                }
                None => {
                    (*order).push(vec![node.val]);
                }
            }

            traversal(node.left.to_owned(), depth + 1, order);
            traversal(node.right.to_owned(), depth + 1, order);
        }
    }

    let mut order: Vec<Vec<i32>> = vec![];

    traversal(root, 0, &mut order);

    for i in 0..order.len() {
        if i % 2 != 0 {
            order[i].reverse();
        }
    }

    order
}

// Log self, log left node, recursive call to right node
// Keep depth in mind