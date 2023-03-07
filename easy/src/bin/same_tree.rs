fn main(){
    let tree1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 10,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: None
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 15,
            left: None,
            right: None
        })))
    })));

    let tree2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 10,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 15,
                left: None,
                right: None
            })))
        }))),
        right: None
    })));

    println!("{}", is_same_tree(tree1, tree2));
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::rc::Rc;
use std::cell::RefCell;

fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn scan(pos_node: Option<Rc<RefCell<TreeNode>>>, array: &mut Vec<i32>) {
        match pos_node {
            Some(node_ref) => {
                let node = node_ref.borrow();

                match node.left.to_owned() {
                    Some(left_node) => {
                        array.push(-1);
                        scan(Some(left_node), array);
                    },
                    None => {}
                }

                array.push(node.val);

                match node.right.to_owned() {
                    Some(right_node) => {
                        array.push(1);
                        scan(Some(right_node), array);
                    },
                    None => {}
                }
            },
            None => {}
        }
    }

    let mut arr1: Vec<i32> = vec![];
    let mut arr2: Vec<i32> = vec![];

    scan(p, &mut arr1);
    scan(q, &mut arr2);

    println!("{arr1:?}");
    println!("{arr2:?}");

    arr1 == arr2
}