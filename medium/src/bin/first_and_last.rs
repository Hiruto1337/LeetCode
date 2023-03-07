fn main() {
    let nums = vec![1];

    println!("{:?}", search_range(nums, 0));
}

fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.len() == 0{
        return vec![-1, -1];
    }
    let mut indices: Vec<i32> = vec![];
    // Do a binary search for target
    let mut left: i32 = 0;
    let mut right: i32 = nums.len() as i32 - 1;

    // Find lower index
    while left <= right {
        let mid = (left + right) / 2;

        if nums[mid as usize] == target {
            if 0 <= mid as i32 - 1 {
                if nums[mid as usize - 1] != target {
                    indices.push(mid as i32);
                    break;
                } else {
                    right = mid - 1;
                }
            } else {
                indices.push(mid as i32);
                break;
            }
        } else if nums[mid as usize] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    // Find upper index
    left = 0;
    right = nums.len() as i32 - 1;

    while left <= right {
        let mid = (left + right) / 2;

        if nums[mid as usize] == target {
            if mid + 1 <= nums.len() as i32 - 1 {
                if nums[mid as usize + 1] != target {
                    indices.push(mid as i32);
                    break;
                } else {
                    left = mid + 1;
                }
            } else {
                indices.push(mid as i32);
                break;
            }
        } else if nums[mid as usize] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    if indices.len() != 0 {
        indices
    } else {
        vec![-1, -1]
    }
}
