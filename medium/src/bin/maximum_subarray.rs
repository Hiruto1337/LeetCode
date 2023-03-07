fn main() {
    let nums = vec![-4, -3, -2, 2, -1, 3, -3, -4];

    println!("{}", max_sub_array(nums));
}

fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max = nums[0];

    let mut prev_sum = 0;

    for i in 0..nums.len() {
        let temp_sum = prev_sum + nums[i];

        if temp_sum <= 0 {
            prev_sum = 0;
            max = std::cmp::max(max, nums[i]);
        } else {
            prev_sum = temp_sum;
            max = std::cmp::max(max, temp_sum);
        }
    }

    max
}
