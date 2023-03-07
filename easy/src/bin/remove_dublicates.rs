fn main() {
    let mut nums: Vec<i32> = vec![-100, -100, -69, -69, -69, -20, -20, -5, 0, 0, 0, 1, 1, 2, 3, 4, 4, 4, 4, 4, 5, 5, 6, 6];

    println!("{}", remove_duplicates(&mut nums));

    println!("{nums:?}");
}

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut prev_num = -101;

    for i in 0..nums.len() {
        if nums[i] != prev_num {
            prev_num = nums[i];
        } else {
            nums[i] = 101;
        }
    }

    nums.retain(|&num| num != 101);

    nums.len() as i32
}