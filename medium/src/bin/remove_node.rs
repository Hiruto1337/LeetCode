use std::ops::DerefMut;

fn main() {
    let head = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 5,
                        next: Some(Box::new(ListNode {
                            val: 6,
                            next: Some(Box::new(ListNode {
                                val: 7,
                                next: Some(Box::new(ListNode {
                                    val: 8,
                                    next: Some(Box::new(ListNode {
                                        val: 9,
                                        next: Some(Box::new(ListNode {
                                            val: 10,
                                            next: None
                                        }))
                                    }))
                                }))
                            }))
                        }))
                    }))
                }))
            }))
        }))
    }));

    let new_head = remove_nth_from_end(head, 3);

    fn read_node(node: Option<Box<ListNode>>) {
        match node {
            Some(list_node) => {
                println!("{}", list_node.val);
                read_node(list_node.next);
            },
            None => {
            }
        }
    }

    read_node(new_head);

}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    fn dig_and_replace(mut node: &mut Option<Box<ListNode>>, position: i32, target: i32) {
        if position != target {
            match node {
                Some(list_node) => {
                    dig_and_replace(&mut list_node.next , position + 1, target);
                },
                None => {}
            }
        } else {
            match node {
                Some(_) => {
                    *node = node.to_owned().unwrap().next;
                },
                None => {}
            }
        }
    }
    
    // Get length of list
    let mut length = 0;

    let mut current_node = head.to_owned();

    while current_node != None {
        length += 1;

        current_node = current_node.unwrap().next;
    }

    // Get length from start to node that should be removed
    let target = length - n + 1;

    let mut head = head;

    dig_and_replace(&mut head, 1, target);

    head
}