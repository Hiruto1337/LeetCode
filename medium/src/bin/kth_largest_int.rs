fn main(){
    let nums = vec![-10000,6,33,1,4,6,3,4,5,67,7,53,456,10000];
    let k = 2;

    println!("{}", find_kth_largest(nums, k));
}

fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut counts = vec![0; usize::pow(10, 4) * 2 + 1];

    for num in nums {
        counts[(i32::pow(10, 4) + num) as usize] += 1;
    }

    let mut k = k;

    for i in (0..counts.len()).rev() {
        while counts[i] != 0 {
            counts[i] -= 1;
            k -= 1;

            if k == 0 {
                return i as i32 - i32::pow(10, 4);
            }
        }
    }

    0
}

