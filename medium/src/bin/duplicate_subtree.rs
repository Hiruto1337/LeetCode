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
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None
            }))),
            right: None
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None
                }))),
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None
            })))
        })))
    })));

    // find_duplicate_subtrees(root);
}

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
fn find_duplicate_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    fn traverse(treenode: Option<Rc<RefCell<TreeNode>>>, map: &mut HashMap<(i32, i32), Vec<Option<Rc<RefCell<TreeNode>>>>>, duplicates: &mut Vec<Option<Rc<RefCell<TreeNode>>>>) -> i32 {
        match treenode.to_owned() {
            Some(node_ref) => {
                let node = node_ref.borrow();
                let left = traverse(node.left.to_owned(), map, duplicates);
                let right = traverse(node.right.to_owned(), map, duplicates);
                let key = (left, right);

                map.entry(key).and_modify(|list| {
                    if list.contains(&treenode) && !duplicates.contains(&treenode) {
                        duplicates.push(treenode);
                    } else {
                        list.push(treenode);
                    }
                }).or_insert(vec![Some(node_ref.to_owned())]);

                return left + right + 1;
            },
            None => return 0
        }
    }

    let mut map: HashMap<(i32, i32), Vec<Option<Rc<RefCell<TreeNode>>>>> = HashMap::new();
    let mut duplicates: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];

    traverse(root, &mut map, &mut duplicates);

    duplicates
}