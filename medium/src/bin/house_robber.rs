fn main() {
    // let nums = vec![10, 0, 0, 10, 0, 0, 10, 0, 0, 10, 0, 0, 10];
    let nums = vec![9,1,8,2,7,3,6,4,5,5,4,6,3,7,2,8,1,9];

    println!("{}", rob(nums));
}

fn rob(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }

    use std::collections::HashMap;

    fn traverse(slice: &[i32], hashmap: &mut HashMap<Vec<i32>, i32>) -> i32 {
        match (*hashmap).get(slice) {
            Some(result) => {
                return *result;
            }
            None => {
                let num_1 = if slice.len() > 2 {
                    traverse(&slice[2..], hashmap)
                } else {0};

                let num_2 = if slice.len() > 3 {
                    traverse(&slice[3..], hashmap)
                } else {0};
                
                (*hashmap).insert(slice.to_vec(), slice[0] + num_1.max(num_2));

                slice[0] + num_1.max(num_2)
            }
        }
    }

    let mut map: HashMap<Vec<i32>, i32> = HashMap::new();

    let num_1 = traverse(&nums, &mut map);
    let num_2 = traverse(&nums[1..], &mut map);

    num_1.max(num_2)
}

// Recursively test every possible path
// Store a path and a the highest possible number for path in a hashmap
