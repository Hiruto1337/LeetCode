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
    let head = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode { val: 1, next: None })),
        })),
    }));

    sort_list(head);
}

fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn merge_sort(list: Vec<i32>) -> Vec<i32> {
        if list.len() <= 1 {
            return list;
        }

        let (left, right) = list.split_at(list.len() / 2);
        let sorted_left = merge_sort(left.to_vec());
        let sorted_right = merge_sort(right.to_vec());

        let mut p1 = 0;
        let mut p2 = 0;

        let mut sorted = vec![];

        while p1 < sorted_left.len() && p2 < sorted_right.len() {
            if sorted_left[p1] < sorted_right[p2] {
                sorted.push(sorted_left[p1]);
                p1 += 1;
            } else {
                sorted.push(sorted_right[p2]);
                p2 += 1;
            }
        }

        sorted.extend(sorted_right[p2..].to_vec());
        sorted.extend(sorted_left[p1..].to_vec());

        sorted
    }

    let mut head = head;

    let mut list = vec![];
    
    let mut current = &head;

    while let Some(node) = current {
        current = &node.next;
        list.push(node.val);
    }

    list = merge_sort(list);

    let mut index = 0;

    let new_current = &mut head;

    while let Some(mut node) = new_current {
        current = &node.next;
        (*node).val = list[index];
        index += 1;
    }

    head
}

// Traverse list and store all values in an array
// Use divide and conquer to merge sort the array
// Create a new tree from the sorted array
