fn main() {
    let nums = vec![];

    println!("{}", majority_element(nums));
}

fn majority_element(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::new();

    nums.iter().for_each(|&num| {
        map.entry(num).and_modify(|val| *val += 1).or_insert(1);
    });

    let mut key_log = nums[0];
    let mut val_log = 0;

    map.iter().for_each(|(&key, &val)|{
        if val > val_log {
            key_log = key;
            val_log = val;
        }
    });

    key_log
}