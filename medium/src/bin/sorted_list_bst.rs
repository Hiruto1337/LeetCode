#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

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

fn main() {
    let head = Some(Box::new(ListNode {
        val: 0,
        next: Some(Box::new(ListNode {
            val: 10,
            next: None
        }))
    }));

    println!("{:?}", sorted_list_to_bst(head));
}

use std::rc::Rc;
use std::cell::RefCell;
fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn create_bst(list: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if list.len() == 0 {
            return None;
        }
        if list.len() == 1 {
            return Some(Rc::new(RefCell::new(TreeNode::new(list[0]))));
        }

        let (left, right) = list.split_at(list.len() / 2);

        let (left, mut right) = (left.to_vec(), right.to_vec());

        let val = right.remove(0);

        Some(Rc::new(RefCell::new(TreeNode {
            val,
            left: create_bst(left),
            right: create_bst(right)
        })))
    }

    let mut list: Vec<i32> = vec![];
    let mut current = head.to_owned();

    while let Some(node) = current {
        list.push(node.val);
        current = node.next;
    }

    create_bst(list)
}