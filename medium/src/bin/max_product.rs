fn main() {
    let mut nums = vec![1, -2, 3, 4, -1, -2];

    // for i in 1..20001 {
    //     if i % 3 == 0 {
    //         nums.push(0);
    //     } else if i % 2 == 0 {
    //         nums.push(-1);
    //     } else {
    //         nums.push(1);
    //     }
    // }

    println!("{}", max_product(nums));
}

fn max_product(nums: Vec<i32>) -> i32 {
    let mut maximum = if nums.contains(&0) { 0 } else { nums[0] };

    for slice in nums.split(|&val| val == 0) {
        if slice.len() == 1 {
            maximum = maximum.max(slice[0]);
        } else if slice.len() != 0 {
            let mut neg_occ = 0;

            for num in slice {
                if *num < 0 {
                    neg_occ += 1;
                }
            }

            if neg_occ == 0 || neg_occ % 2 == 0 {
                maximum = maximum.max(slice.iter().fold(1, |acc, x| acc * x));
            } else {
                let mut left = 0;
                let mut right = slice.len() - 1;

                while !(slice[left] < 0) {
                    left += 1;
                }

                while !(slice[right] < 0) {
                    right -= 1;
                }

                maximum = maximum.max(slice[left + 1..].iter().fold(1, |acc, x| acc * x));
                maximum = maximum.max(slice[..right].iter().fold(1, |acc, x| acc * x));
            }
        }
    }

    maximum
}

// Split nums at every zero
// Get the number of negative numbers in each subarray
// - If this number is even, convert the subarray to one product
// - If this number is odd, consider the whole subarray and backtrack undtil finding a negative number
// - - This should be done from both ends
