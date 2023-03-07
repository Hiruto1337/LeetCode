fn main() {
    let words = vec!["word".to_string(), "world".to_string(), "row".to_string()];
    let order = "worldabcefghijkmnpqstuvxyz".to_string();

    println!("{}", is_alien_sorted(words, order));
}

fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
    use std::collections::HashMap;

    let mut map: HashMap<char, usize> = HashMap::new();

    for (i, char) in order.chars().enumerate() {
        map.insert(char, i);
    }

    for i in 0..words.len() - 1 {
        let vec1: Vec<char> = words[i].chars().collect();
        let vec2: Vec<char> = words[i + 1].chars().collect();

        if vec1 == vec2 || map.get(&vec1[0]).unwrap() < map.get(&vec2[0]).unwrap() {
            continue;
        }

        let mut ptr = 0;

        while ptr < std::cmp::min(vec1.len(), vec2.len()) && vec1[ptr] == vec2[ptr] {
            ptr += 1;
        }

        if (ptr < vec1.len() && ptr < vec2.len() && map.get(&vec1[ptr]).unwrap() > map.get(&vec2[ptr]).unwrap())
        || (ptr == vec2.len() && ptr != vec1.len()) {
            return false;
        }
    }

    true
}
