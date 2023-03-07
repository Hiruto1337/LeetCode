fn main() {
    let mut nums: Vec<i32> = vec![1, 2, 1, 2, 1, 2, 1, 2, 0];

    sort_colors(&mut nums);

    println!("{nums:?}");
}

fn sort_colors(nums: &mut Vec<i32>) {
    let mut left = 0;
    let mut right = nums.len() - 1;

    loop {
        while nums[left] == 0 && left < right {
            left += 1;
        }

        while nums[right] == 2 && left < right {
            right -= 1;
        }

        if nums[left] != nums[right] {
            let temp = nums[right];
            nums[right] = nums[left];
            nums[left] = temp;
        } else {
            let mut continue_loop = true;

            let diff = right - left;

            if 0 < diff {
                for i in 0..diff {
                    nums.insert(right, nums[left]);
                    nums.remove(left);
    
                    if nums[left] != nums[right] {
                        break;
                    }
    
                    if i == diff - 1 {
                        continue_loop = false;
                    }
                }
            } else {
                continue_loop = false;
            }

            if !continue_loop {
                break;
            }
        }
    }
}

// [2, 0, 2, 1, 1, 0]
