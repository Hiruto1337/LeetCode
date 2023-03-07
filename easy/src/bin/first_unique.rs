fn main(){
    println!("{}", first_uniq_char(String::from("leetcode")));
}

fn first_uniq_char(s: String) -> i32 {
    use std::collections::HashMap;

    let mut once: HashMap<char, usize> = HashMap::new();

    let mut twice: HashMap<char, usize> = HashMap::new();

    let vec: Vec<char> = s.chars().collect();

    for i in 0..vec.len() {
        let key = vec[i];

        let mut del_key = false;

        if !twice.contains_key(&key) {
            once.entry(key).and_modify(|_| {
                twice.entry(key).or_insert(1);
                del_key = true;
            }).or_insert(i);
        }

        if del_key {
            once.remove(&key);
        }
    }

    let mut min_index = 100001;

    for pair in once.iter() {
        min_index = std::cmp::min(min_index, *pair.1);
    }

    if min_index == 100001 {
        return -1;
    }

    return min_index as i32;
}