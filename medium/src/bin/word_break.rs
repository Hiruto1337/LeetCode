fn main() {
    let s = String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab");

    let word_dict = vec![
        String::from("a"),
        String::from("aa"),
        String::from("aaa"),
        String::from("aaaa"),
        String::from("aaaaa"),
        String::from("aaaaaa"),
        String::from("aaaaaaa"),
        String::from("aaaaaaaa"),
        String::from("aaaaaaaaa"),
        String::from("aaaaaaaaaa"),
    ];

    // let mut word_dict_valid = vec![];

    // for word in word_dict {
    //     word_dict_valid.push(word.to_string());
    // }

    println!("{}", word_break(s, word_dict));
}

fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let mut match_at: Vec<bool> = vec![false;s.len() + 1];

    match_at[0] = true;

    for index in 1..=s.len() {
        for word in &word_dict {
            if index < word.len() {
                continue;
            }

            let slice = &s[index - word.len()..index];
            if match_at[index - word.len()] && slice == word {
                match_at[index] = true;
            }
        }
    }

    match_at[s.len()]
}

// Look at the remains of the string
// If the remains contain a word from the word_dict, remove that word and repeat process
// If the remains ever get deleted completely, return true
// If the remains never get deleted completely, return false
