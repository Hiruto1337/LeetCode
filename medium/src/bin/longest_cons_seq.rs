fn main(){
    let mut nums = vec![];

    for i in (0..100000).rev() {
        nums.push(i);
    }

    println!("{}", longest_consecutive(nums));
}

fn longest_consecutive(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    let mut set: HashSet<i32> = HashSet::new();

    for num in &nums {
        set.insert(*num);
    }

    let mut global = 0;

    let len_half = nums.len() / 2;

    for num in nums {
        if let true = set.remove(&(num)) {

            let mut local = 1;
            let mut offset = 1;

            while let true = set.remove(&(num + offset)) {
                local += 1;
                offset += 1;
            }

            offset = 1;

            while let true = set.remove(&(num - offset)) {
                local += 1;
                offset += 1;
            }

            global = std::cmp::max(global, local);

            if global as usize > len_half {
                break;
            }
        }
    }

    global
}