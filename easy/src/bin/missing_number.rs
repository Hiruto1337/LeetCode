fn main() {
    
}

fn missing_number(nums: Vec<i32>) -> i32 {
    let mut result: Vec<i32> = vec![];

    for i in 0..nums.len() + 1 {
        result.push(i as i32);
    }

    for num in nums {
        result[num as usize] = -1;
    }

    result.retain(|&num| num != -1);

    result[0]
}