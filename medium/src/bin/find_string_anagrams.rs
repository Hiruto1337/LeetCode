fn main() {
    let mut s_vec: Vec<char> = vec![];
    let mut p_vec: Vec<char> = vec![];

    for i in 0..10000 {
        s_vec.push('a');
        p_vec.push('a');
    }

    s_vec.push('b');

    for i in 0..10000 {
        s_vec.push('a');
    }

    let s: String = s_vec.iter().collect();
    let p: String = p_vec.iter().collect();

    let s_sum = s.chars().into_iter().fold(0, |acc, x| acc + i32::pow(x as i32, 4));

    println!("{:?}", find_anagrams(s, p));
}

fn find_anagrams(s: String, p: String) -> Vec<i32> {
    let mut p_signature: [usize; 26] = [0; 26];

    for val in p.chars() {
        p_signature[(val as u8 - b'a') as usize] += 1;
    }

    let mut result: Vec<i32> = vec![];

    let mut s_signature: [usize; 26] = [0; 26];

    let s_bytes = s.as_bytes();

    for i in 0..s.len() {
        s_signature[(s_bytes[i] - b'a') as usize] += 1;

        if i >= p.len() {
            s_signature[(s_bytes[i - p.len()] - b'a') as usize] -= 1;
        }

        if s_signature == p_signature {
            result.push((i - (p.len() - 1)) as i32);
        }
    }

    result
}

// Get signature of p
// Create signature of slice of s
// Keep rolling up s
// If an s_signature is equal to a p_signature, add the index - p.len() to result vector
