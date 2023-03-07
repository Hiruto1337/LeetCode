fn main() {
    println!(
        "{}",
        longest_common_prefix(vec![
            String::from("aca"),
            String::from("cba"),
        ])
    );
}

fn longest_common_prefix(strs: Vec<String>) -> String {
    // Find the longest word in strs
    let mut longest_word: (String, usize) = (String::new(), 0);

    for word in strs.to_owned() {
        if word.len() > longest_word.1 {
            longest_word = (word.to_owned(), word.to_owned().len());
        }
    }

    // Find the shortest word in strs
    let mut shortest_word: (String, usize) = (String::new(), 10000);

    for word in strs.to_owned() {
        if word.len() < shortest_word.1 {
            shortest_word = (word.to_owned(), word.to_owned().len());
        }
    }

    let longest_string: String = longest_word.0;

    for i in 0..longest_word.1 {
        // Split the longest string at the length of the shortest string - i
        let remains = longest_string.split_at(shortest_word.1.to_owned() - i).0;

        let mut valid: bool = true;

        for word in strs.to_owned() {
            if !word.split_at(remains.len()).0.contains(remains) {
                valid = false;
                break;
            }
        }

        if valid {
            return remains.to_string();
        }
    }

    String::from("")
}
