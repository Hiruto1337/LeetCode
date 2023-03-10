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
struct Solution {
    head: Option<Box<ListNode>>,
    length: u16,
}

use rand::Rng;

impl Solution {
    fn new(head: Option<Box<ListNode>>) -> Self {
        let mut current = head.to_owned();
        let mut length = 0;

        while let Some(node) = current {
            length += 1;
            current = node.next;
        }

        Solution {
            head,
            length
        }
    }
    
    fn get_random(&self) -> i32 {
        let mut current = self.head.to_owned();
        let mut count = 0;
        let random_num = rand::thread_rng().gen_range(0, self.length);
        loop {
            let node = current.unwrap();

            if count == random_num {
                return node.val;
            }

            count += 1;
            current = node.next;
        }
    }
}
