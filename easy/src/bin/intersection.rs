fn main(){
    let nums1 = vec![];
    let nums2 = vec![];

    println!("{:?}", intersect(nums1, nums2));
}

fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;

    let mut map: HashMap<i32, u32> = HashMap::new();

    for num in nums1 {
        map.entry(num).and_modify(|val| *val += 1).or_insert(1);
    }

    let mut result: Vec<i32> = vec![];

    for num in nums2 {
        map.entry(num).and_modify(|val| if *val > 0 {
            result.push(num);
            *val -= 1;
        });
    }

    result
}