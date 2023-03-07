fn main() {
    println!("{}", longest_palindrome(String::from("abcdcba")));
}

fn longest_palindrome(s: String) -> String {
    if s.len() == 1 {
        return s;
    } else if s.len() == 2 {
        let mut letters: Vec<&str> = s.split("").collect();
        letters.retain(|&str| str != "");

        if letters[0] == letters[1] {
            return s;
        } else {
            return String::from(letters[0]);
        }
    }

    let mut letters: Vec<&str> = s.split("").collect();
    letters.retain(|&str| str != "");

    let mut longest_uniform_chain: Vec<&str> = vec![];

    let mut start: usize = 0;
    let mut end: usize = letters.len();

    let mut palindromes: Vec<String> = vec![];

    // Scout letters for uniform chains
    for i in 0..letters.len() {
        let mut dist = 1;
        while i + dist < letters.len() {
            if letters[i] == letters[i + dist] {
                if letters[i..i+dist + 1].len() > longest_uniform_chain.len() {
                    longest_uniform_chain = letters[i..i+dist + 1].to_vec();
                    
                    palindromes.push(letters[i..i+dist + 1].to_vec().join(""));

                    if longest_uniform_chain.len() > letters.len()/2 {
                        start = i + dist/2;
                        end = start + 2;
                    }
                }

                dist += 1;
            } else {
                break;
            }
        }
    }

    for palindrome in palindromes.to_owned() {
        if palindrome.len() == s.to_owned().len() {
            return palindrome;
        }
    }

    // Get odd palindromes
    for i in start..end {
        let mut dist = 1;
        while 0 <= i as i32 - dist as i32 && i + dist < letters.len() {
            if letters[i - dist] == letters[i + dist] {
                palindromes.push(letters[(i - dist)..(i + dist + 1)].to_vec().join(""));
                dist += 1;
            } else {
                break;
            }
        }
    }

    for palindrome in palindromes.to_owned() {
        if palindrome.len() == s.to_owned().len() {
            return palindrome;
        }
    }

    // Get even palindromes
    for i in start..end - 1 {
        if letters[i] == letters[i + 1] {
            let mut dist = 0;
            while 0 <= i as i32 - dist as i32 && i+1 + dist < letters.len() {
                if letters[i - dist] == letters[i+1 + dist] {
                    palindromes.push(letters[(i - dist)..((i+1 + dist) + 1)].to_vec().join(""));
                    dist += 1;
                } else {
                    break;
                }
            }
        }
    }

    // If no palindromes exist, return the first letter
    if palindromes.len() == 0 {
        return String::from(letters[0]);
    }

    // Find and return the longest palindrome
    let mut longest_palindrome: String = String::new();

    for palindrome in palindromes {
        if palindrome.len() > longest_palindrome.len() {
            longest_palindrome = palindrome;
        }
    }

    longest_palindrome
}
