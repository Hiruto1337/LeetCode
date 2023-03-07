fn main(){
    let nums = vec![1,2,3,4,5,6,7,8];
    let n = 4;

    println!("{:?}", shuffle(nums, n));
}

fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];

    for i in 0..nums.len() - n as usize {
        result.push(nums[i]);
        result.push(nums[i + n as usize]);
    }

    result
}