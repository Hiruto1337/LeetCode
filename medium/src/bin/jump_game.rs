fn main() {
    let mut nums: Vec<i32> = vec![3,2,1,0,4];

    println!("{}", can_jump(nums));
}

fn can_jump(nums: Vec<i32>) -> bool {
    let mut x = 1;

    for i in (0..nums.len() - 1).rev() {
        if nums[i] < x {
            x += 1;
        } else {
            x = 1;
        }
    }

    x == 1
}
