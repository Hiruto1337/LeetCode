fn main() {
    let word1 = "horse".to_string();
    let word2 = "ros".to_string();

    println!("{}", min_distance(word1, word2));
}

fn min_distance(word1: String, word2: String) -> i32 {
    let mut row: Vec<i32> = vec![];

    for len in 0..=word2.len() {
        row.push(len as i32);
    }

    let mut prev_row = row.to_owned();

    row.pop().unwrap()
}


// dinitrophenylhydrazine
// benzalphenylhydrazone

//         "" "r" "ro" "ros"
//      "" 0   1    2    3
//     "h" 1   1    2    3
//    "ho" 2   2    1    2
//   "hor" 3   2    2    2
//  "hors" 4   3    3    2
// "horse" 5   4    4    3