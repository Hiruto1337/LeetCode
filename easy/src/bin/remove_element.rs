fn main() {
    let mut nums: Vec<i32> = vec![2,2,3];

    println!("{}", remove_element(&mut nums, 2));

    println!("{nums:?}");
}

fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    match nums.len() {
        0 => return 0,
        1 => {
            if nums[0] == val {
                return 0;
            } else {
                return 1;
            }
        }
        _ => {
            let mut left = 0;
            let mut right = nums.len() - 1;

            while left < right {
                while nums[right] == val && right != 0 && right != left {
                    right -= 1;
                }

                if nums[left] == val {
                    nums.swap(left, right);
                }

                left += 1;
            }

            for i in 0..nums.len() {
                if nums[i] == val {
                    return i as i32;
                }
            }

            nums.len() as i32
        }
    }
}
