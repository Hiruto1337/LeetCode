fn main() {
    println!("{}", length_of_longest_substring(String::from("asdfgghjkl")));
}

fn length_of_longest_substring(s: String) -> i32 {
    use std::collections::HashMap;

    let s: Vec<char> = s.chars().collect();

    let mut map: HashMap<char, usize> = HashMap::new();

    let mut longest = 0;

    let (mut bot, mut top) = (0, 0);

    while top < s.len() {
        match map.get(&s[top]) {
            Some(val) => {
                bot = *val + 1;
                map.entry(s[top]).and_modify(|value| *value = top);
                map.retain(|_,value| *value >= bot);
            },
            None => {
                map.insert(s[top], top);
            }
        }

        longest = longest.max(s[bot..=top].len());

        top += 1;
    }

    longest as i32
}

// As long as the hashset doesn't contain the char being evaluated,
// insert the char and update current longest
// If it does contain the char, keep going
// If it gets the same char twice in a row, clear the set and keep going

// ajsbasda