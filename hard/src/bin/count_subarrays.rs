fn main() {
    let nums = vec![942922,26282,908345,908345,252308,908345,908345,865114,797201,26282,26282,26282,771220,908345,226478,801741,26282,908345,26282,628321,26282,26282,26282,317964,908345,375285,212793,389830,26282,26282,908345,199587,225849,137360,908345,26282,881084,938510,991656,920318];
    let min_k = 26282;
    let max_k = 908345;

    println!("{}", count_subarrays(nums, min_k, max_k));
}

fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
    let (mut min, mut max, mut left) = (-1, -1, -1);

    let mut result = 0;

    for i in 0..nums.len() {
        if nums[i] < min_k || max_k < nums[i] {
            left = i as i64;
            continue;
        }

        if nums[i] == min_k {
            min = i as i64;
        }

        if nums[i] == max_k {
            max = i as i64;
        }

        result += 0.max(min.min(max) - left);
    }

    result
}
