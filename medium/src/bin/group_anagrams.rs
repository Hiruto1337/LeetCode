fn main() {
    let strs: Vec<String> = vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ];

    println!("{:?}", group_anagrams(strs));
}

fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::HashMap;

    let mut map: HashMap<Vec<char>, Vec<String>> = HashMap::new();

    for str in strs {
        let mut str_vec: Vec<char> = str.chars().collect();
        str_vec.sort();

        map.entry(str_vec)
            .and_modify(|vec| (*vec).push(str.to_owned()))
            .or_insert(vec![str.to_owned()]);
    }

    let mut groups: Vec<Vec<String>> = vec![];

    map.iter().for_each(|(_, group)| groups.push(group.to_owned()));

    groups
}
