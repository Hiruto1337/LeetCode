fn main() {
    println!("{:?}", letter_combinations(String::from("2379")).len());
}

fn letter_combinations(digits: String) -> Vec<String> {
    use std::collections::HashMap;

    let mut digits: Vec<&str> = digits.split("").collect();

    digits.retain(|&str| str != "");

    let mut map = HashMap::new();

    map.insert("2", vec!["a", "b", "c"]);
    map.insert("3", vec!["d", "e", "f"]);
    map.insert("4", vec!["g", "h", "i"]);
    map.insert("5", vec!["j", "k", "l"]);
    map.insert("6", vec!["m", "n", "o"]);
    map.insert("7", vec!["p", "q", "r", "s"]);
    map.insert("8", vec!["t", "u", "v"]);
    map.insert("9", vec!["w", "x", "y", "z"]);

    let mut combinations: Vec<String> = vec![];

    match digits.len() {
        1 => {
            for let1 in map[digits[0]].to_owned() {
                combinations.push(let1.to_string());
            }
        },
        2 => {
            for let1 in map[digits[0]].to_owned() {
                for let2 in map[digits[1]].to_owned() {
                    combinations.push(format!("{}{}", let1, let2));
                }
            }
        },
        3 => {
            for let1 in map[digits[0]].to_owned() {
                for let2 in map[digits[1]].to_owned() {
                    for let3 in map[digits[2]].to_owned() {
                        combinations.push(format!("{}{}{}", let1, let2, let3));
                    }
                }
            }
        },
        4 => {
            for let1 in map[digits[0]].to_owned() {
                for let2 in map[digits[1]].to_owned() {
                    for let3 in map[digits[2]].to_owned() {
                        for let4 in map[digits[3]].to_owned() {
                            combinations.push(format!("{}{}{}{}", let1, let2, let3, let4));
                        }
                    }
                }
            }
        },
        _ => {}
    }

    combinations
}