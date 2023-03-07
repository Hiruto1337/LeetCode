fn main() {
    let mut nums = vec![];

    for i in 0..101 {
        nums.push(i);
    }

    nums.rotate_left(1);

    println!("{:?}", nums);

    let index = search(nums, 100);

    println!("Index = {index}");

}

fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    let mut offset = 0;

    // Sort nums and keep track of offset
    while !(nums[0] <= nums[nums.len() - 1]) {
        nums.rotate_left(1);
        offset += 1;
    }

    // Do a binary search of nums
    let mut left: i32 = 0;

    let mut right: i32 = nums.len() as i32 - 1;

    while left <= right {
        let mid = (left + right) / 2;

        if nums[mid as usize] == target {
            return ((mid + offset) % nums.len() as i32) as i32;
        } else if nums[mid as usize] < target {
            left = mid + 1;
        } else if nums[mid as usize] > target {
            right = mid - 1;
        }
    }

    -1
}