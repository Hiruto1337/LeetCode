fn main() {
    let nums = vec![5, 1, 1, 2, 0, 0];

    println!("{:?}", sort_array(nums));
}

use std::{collections::BinaryHeap, cmp::Reverse};
fn sort_array(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let mut heap = BinaryHeap::new();

    while let Some(num) = nums.pop() {
        heap.push(Reverse(num));
    }

    let mut sorted = vec![];

    while let Some(num) = heap.pop() {
        sorted.push(num.0);
    }

    sorted
}
