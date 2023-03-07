fn main(){
    let mut nums: Vec<i32> = vec![1,2,0,3,4,0,5,6];

    move_zeroes(&mut nums);

    println!("{nums:?}");
}

fn move_zeroes(nums: &mut Vec<i32>) {
    let mut left_most_zero = 0;

    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums.swap(left_most_zero, i);
            left_most_zero += 1;
        }
    }
}
