fn main() {
    let nums = vec![1,2,3];

    println!("{:?}", product_except_self(nums));
}

fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![1; nums.len()];

    let (mut left_prod, mut right_prod) = (1, 1);

    for i in 1..nums.len() {
        let right = nums.len() - 1 - i;

        left_prod *= nums[i - 1];
        result[i] *= left_prod;

        right_prod *= nums[right + 1];
        result[right] *= right_prod;
    }

    result
}
