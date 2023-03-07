fn main() {
    let mut arr = vec![5,4,3];

    println!("{}", min_jumps(arr));
}
use std::collections::{HashMap, HashSet};
fn min_jumps(arr: Vec<i32>) -> i32 {
    let mut paths: HashMap<i32, Vec<usize>> = HashMap::new();

    for (i, &num) in arr.iter().enumerate() {
        paths.entry(num).or_insert(vec![]).push(i);
    }

    let mut priority = vec![(arr.len() - 1, 0)];

    let mut visited: HashSet<usize> = HashSet::new();

    loop {
        let (node, depth) = priority.remove(0);

        if node == 0 {return depth;}

        visited.insert(node);

        let mut origins = paths.get(&arr[node]).unwrap().to_owned();
        origins.push(node - (node != 0) as usize);
        origins.push(node + (node != arr.len() - 1) as usize);

        for origin in origins {
            if origin == 0 {
                return depth + 1;
            } else if let None = visited.get(&origin) {
                priority.push((origin, depth + 1));
            }
        }

        paths.entry(arr[node]).and_modify(|list| *list = vec![]);
    }
}

// [-76,3,66,-32,64,2,-19,-8,-5,-93,80,-5,-76,-78,64,2,16]
// The only way to 16 is by 2
// -> The only way to 2 is by 64 and -19
// ---> The only ways to 64 is by -32 and -78
// ---> The only way to -19 is by -8
// -----> The only way to -32 is by 66
// -----> The only way to -78 is by -76
// -----> You can get from -76 from the start!

//             16
//              2
//        64     -19
//   -32    -78     -8
//    66    -76        -5
//         Start!   80    -76
//                       Start!
