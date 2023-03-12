use std::cmp::Reverse;

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

fn main() {}

fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    use std::collections::BinaryHeap;

    let mut min_heap: BinaryHeap<i32> = BinaryHeap::new();

    for list in lists {
        let mut current = list;
        while let Some(node) = current {
            current = node.next;
            min_heap.push(node.val);
        }
    }

    let mut result = None;

    while let Some(val) = min_heap.pop() {
        let node = ListNode {
            val,
            next: result
        };

        result = Some(Box::new(node));
    }

    result
}
