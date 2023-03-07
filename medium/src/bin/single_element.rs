fn main() {
    let nums = vec![1,1,3,3,4,4,8,8,69];
    // TEST VED nums.len() == 1
    // TEST VED EDGE CASES

    println!("{}", single_non_duplicate(nums));
}

fn single_non_duplicate(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left < right {
        let mid = (right + left) / 2;

        let even_index = mid % 2 == 0;
        let border_index = nums[mid] != nums[mid + 1];
        
        if even_index == border_index {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    nums[left]
}
