// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

fn main() {
    let list1 = Some(Box::new(ListNode {
        val: 9,
        next: Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: None
                }))
            }))
        }))
    }));

    let list2 = Some(Box::new(ListNode {
        val: 3,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 8,
                    next: None
                }))
            }))
        }))
    }));
}

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut number1: Vec<i32> = vec![];
    let mut number2: Vec<i32> = vec![];

    fn traverse_list(list: Option<Box<ListNode>>, array: &mut Vec<i32>) {
        match list {
            Some(node_ref) => {
                match node_ref.to_owned().next {
                    Some(next_node_ref) => {
                        traverse_list(Some(next_node_ref), array);
                    },
                    None => {}
                }
                array.push(node_ref.val);
            },
            None => {}
        }
    }

    traverse_list(l1.to_owned(), &mut number1);
    traverse_list(l2.to_owned(), &mut number2);

    number1.reverse();
    number2.reverse();

    if number1.len() > number2.len() {
        while number1.len() > number2.len() {
            number2.push(0);
        }
    } else if number2.len() > number1.len() {
        while number2.len() > number1.len() {
            number1.push(0);
        }
    }

    let mut sum_array: Vec<i32> = vec![];

    for i in 0..number1.len() {
        let sum = number1[i] + number2[i];
        sum_array.push(sum);
    }

    for i in 0..sum_array.len() {
        if 9 < sum_array[i] {
            sum_array[i] -= 10;

            if sum_array.len() < i + 2 {
                sum_array.push(1);
            } else {
                sum_array[i + 1] += 1;
            }
        }
    }

    fn create_node(array: Vec<i32>, i: usize) -> Option<Box<ListNode>> {
        if i + 1 <= array.len() {
            Some(Box::new(ListNode{
                val: array[i],
                next: create_node(array, i + 1)
            }))
        } else {
            None
        }
    }

    println!("{number1:?}");
    println!("{number2:?}");

    create_node(sum_array, 0)
}