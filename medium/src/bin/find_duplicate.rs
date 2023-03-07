fn main(){
    let nums = vec![1,3,4,2,2];

    println!("{}", find_duplicate(nums));
}

fn find_duplicate(nums: Vec<i32>) -> i32 {
    let mut counts: Vec<i32> = vec![0; usize::pow(10, 5) + 1];

    for num in nums {
        if counts[num as usize] > 0 {
            return num;
        } else {
            counts[num as usize] += 1;
        }
    }

    0
}