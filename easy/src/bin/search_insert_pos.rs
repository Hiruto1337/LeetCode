fn main(){
    let nums = vec![1,3];
    let target = 0;

    println!("{}", search_insert(nums, target));
}

fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;

        if mid == 0 && target < nums[left] {
            return 0;
        } else if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] < target {
            left = mid + 1;
        } else if target < nums[mid] {
            right = mid - 1;
        }
    }

    left as i32
}

// let nums = vec![1,3,4,5,6,7,8,9,10];
// let target = 2;

// mid = 4

// let nums = vec![1,3,4,5];

// mid = 1

// let nums = vec![1];

