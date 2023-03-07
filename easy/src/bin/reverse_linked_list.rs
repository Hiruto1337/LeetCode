#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn main() {
    let mut list = ListNode::new(1);

    list.next = Some(Box::new(ListNode::new(2)));

    reverse_list(Some(Box::new(list)));
}

fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut current = head;

    while let Some(mut node) = current {
        current = node.next;
        node.next = prev;

        prev = Some(node);
    }

    prev
}
