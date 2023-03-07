fn main() {
    let indices: Vec<i32> = two_sum(vec![3, 3], 6);

    println!("{indices:?}");
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut map: HashMap<i32, usize> = HashMap::new();

    // Gradually build hashmap and check for new_target while building
    for (i, &num) in nums.iter().enumerate() {
        let new_target = target - nums[i];

        match map.get(&new_target) {
            Some(&index) => {
                return vec![index as i32, i as i32];
            }
            None => {
                map.insert(num, i);
            }
        }
    }

    vec![]
}
