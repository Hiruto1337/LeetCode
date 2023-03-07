fn main() {
    let mut nums = vec![1,2];
    let k = 100000;

    rotate(&mut nums, k);

    println!("{nums:?}");
}

fn rotate(nums: &mut Vec<i32>, k: i32) {
    let k = k as usize % nums.len();
    let mut new = nums[nums.len() - k..].to_vec();
    new.extend(nums[..nums.len() - k].iter());
    *nums = new;
}

// Iterer gennem nums med en forskydning på k

// [1,2,3,4,5,6] k = 3
//  1
// [1,2,3,4,5,6]
//        1
// [1,2,3,4,5,6]
//        4
// [1,2,3,1,5,6]
//  4
// [1,2,3,1,5,6]
//  1
// [4,2,3,1,5,6]
//        1
// [4,2,3,1,5,6]

// [1,2,3,4,5] k = 2
//  1
// [1,2,3,4,5]
//      1
// [1,2,3,4,5]
//      3
// [1,2,1,4,5]
//          3
// [1,2,1,4,5]
//          5
// [1,2,1,4,3]
//    5
// [1,2,1,4,3]
//    2
// [1,5,1,4,3]
//        2
// [1,5,1,4,3]
//        4
// [1,5,1,2,3]
//  4
// [1,5,1,2,3]
// [4,5,1,2,3]
