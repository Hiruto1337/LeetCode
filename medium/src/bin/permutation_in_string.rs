fn main(){
    let s1 = "abcdefghi".to_string();
    let s2 = "afghbcdei".to_string();

    println!("{}", check_inclusion(s1, s2));
}

fn check_inclusion(s1: String, s2: String) -> bool {
    let mut signature_1 = [0 ; 26];
    let mut signature_2 = [0 ; 26];

    for char in s1.chars() {
        signature_1[(char as u8 - b'a') as usize] += 1;
    }

    let s2_signature = s2.as_bytes();

    for (i, char) in s2.chars().enumerate() {
        signature_2[(char as u8 - b'a') as usize] += 1;

        if i >= s1.len() {
            signature_2[(s2_signature[i - s1.len()] - b'a') as usize] -= 1;
        }

        if signature_1 == signature_2 {
            return true;
        }
    }

    false
}

// Create a hashmap over all letters in s1
// Make a mutable copy of this hashmap
// Iterate over s2 - if a letter exists in the hashmap, increment length by 1
// and remove entry from hashmap copy
// If a letter in s2 does not exist in the hashmap, reset the length to 0