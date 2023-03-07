fn main() {
    let column_title: String = String::from("FXSHRXW");

    println!("{}", title_to_number(column_title));
}

fn title_to_number(column_title: String) -> i32 {
    use std::collections::HashMap;

    // Create a mapping
    let alphabet: Vec<&str> = vec![
        "A", "B", "C", "D", "E", "F", "G", "H", "I",
        "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z"
    ];

    let mut map: HashMap<&str, usize> = HashMap::new();

    for (i, &letter) in alphabet.iter().enumerate() {
        map.insert(letter, i + 1);
    }

    // Translate title to number
    let mut col_vec: Vec<&str> = column_title.split("").collect();
    col_vec.retain(|&str| str != "");

    let mut result = 0;

    while col_vec.len() > 0 {
        result += i32::pow(26, (col_vec.len() - 1) as u32) as usize * map.get(col_vec[0]).unwrap();
        col_vec.remove(0);
    }

    result as i32
}
