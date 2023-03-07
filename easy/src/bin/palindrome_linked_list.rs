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

fn main(){}

fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    fn traverse(mut list_node: Option<Box<ListNode>>, list: &mut Vec<i32>) {
        while let Some(node) = list_node {
            list_node = node.next;

            (*list).push(node.val);
        }
    }

    let mut list: Vec<i32> = vec![];

    traverse(head, &mut list);

    let mut rev = list.to_owned();

    rev.reverse();

    list == rev
}