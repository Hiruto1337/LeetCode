fn main(){
    let nums = vec![7,3,5,7,8,2];

    println!("{:?}", length_of_lis(nums));
}

fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut stacks: Vec<i32> = vec![];

    for num in nums {
        if let Err(index) = stacks.binary_search(&num) {
            if index == stacks.len() {
                stacks.push(num);
            } else {
                stacks[index] = num;
            }
        }
    }

    stacks.len() as i32
}
