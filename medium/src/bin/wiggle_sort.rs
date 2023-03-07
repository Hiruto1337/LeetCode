fn main() {
    let mut nums = vec![1,2,3];

    wiggle_sort(&mut nums);

    println!("{nums:?}");
}

fn wiggle_sort(nums: &mut Vec<i32>) {
    nums.sort();

    let (mut left, mut right) = (1, nums.len() / 2 + (nums.len() % 2 == 1) as usize);

    let len = nums.len();

    while right != nums.len() {
        nums.insert(left, nums[right]);
        left += 2;
        right += 2;
    }

    while nums.len() != len {
        nums.pop();
    }
}

// fn valid(index: usize, nums: &mut Vec<i32>) -> bool {
//     if index % 2 == 0 && nums[index] >= nums[index + 1] {
//         return false;
//     } else if index % 2 == 1 && nums[index] <= nums[index + 1] {
//         return false;
//     }

//     true
// }

// [1,1,2,1,2,2,1]

// If a number is odd, there will be X small numbers and X-1 larger numbers
// If a number is even, there will be X small numbers and X large numbers
