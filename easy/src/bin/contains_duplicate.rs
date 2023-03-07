fn main(){}

fn contains_duplicate(nums: Vec<i32>) -> bool {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    !nums.iter().all(|num| set.insert(num))
}