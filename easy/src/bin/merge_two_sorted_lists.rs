// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

fn main() {
    let list_69 = Option::from(Box::new(ListNode {
        val: 1,
        next: Option::from(Box::new(ListNode {
            val: 2,
            next: Option::from(Box::new(ListNode {
                val: 4,
                next: None
            }))
        }))
    }));

    let list_420 = Option::from(Box::new(ListNode {
        val: 1,
        next: Option::from(Box::new(ListNode {
            val: 3,
            next: Option::from(Box::new(ListNode {
                val: 4,
                next: None
            }))
        }))
    }));
    
    merge_two_lists(list_69, list_420);
}

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut list_one: Vec<i32> = vec![];
    let mut list_two: Vec<i32> = vec![];

    let mut list1_remains: Option<Box<ListNode>> = list1.to_owned();

    // Loop through all nodes in list1
    loop {
        match &list1_remains {
            Some(value) => {
                list_one.push(value.val);
                list1_remains = value.next.to_owned();
            },
            None => break
        }
    }

    let mut list2_remains: Option<Box<ListNode>> = list2.to_owned();

    // Loop through all nodes in list2
    loop {
        match &list2_remains {
            Some(value) => {
                list_two.push(value.val);
                list2_remains = value.next.to_owned();
            },
            None => break
        }
    }

    let mut list_combination: Vec<i32> = vec![];

    list_combination.extend_from_slice(&list_one);

    list_combination.extend_from_slice(&list_two);

    // Sort list_combination in ascending order
    list_combination.sort();

    // Reverse list_combination
    list_combination.reverse();

    let mut shell: Option<Box<ListNode>> = None;

    for number in list_combination {
        let new_layer = Option::from(Box::new(ListNode {
            val: number,
            next: shell.to_owned()
        }));

        shell = new_layer;
    }

    shell
}
