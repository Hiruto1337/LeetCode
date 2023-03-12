fn main() {
    let ideas = vec![
        "coffee".to_string(),
        "donuts".to_string(),
        "time".to_string(),
        "toffee".to_string(),
    ];
    // Output: 6

    // let ideas = vec![
    //     "aaa".to_string(),
    //     "baa".to_string(),
    //     "caa".to_string(),
    //     "bbb".to_string(),
    //     "cbb".to_string(),
    //     "dbb".to_string(),
    // ];
    // Output: 2

    println!("{}", distinct_names(ideas.iter().map(|str| str.to_string()).collect::<Vec<String>>()));
}

fn distinct_names(ideas: Vec<String>) -> i64 {
    use std::collections::{HashMap, HashSet};

    let mut map: HashMap<String, HashSet<String>> = HashMap::new();

    for idea in &ideas {
        let comps = idea.split_at(1);

        map.entry(comps.0.to_string())
            .and_modify(|set| {
                (*set).insert(comps.1.to_string());
            })
            .or_insert(HashSet::from([comps.1.to_string()]));
    }

    let mut result = 0;

    for (letter_1, set_1) in map.iter() {
        for (letter_2, set_2) in map.iter() {
            if letter_1 != letter_2 {
                let mut mutual: Vec<String> = vec![];

                for suffix in set_1 {
                    if let Some(_) = set_2.get(suffix) {
                        mutual.push(suffix.to_owned());
                    }
                }

                result += ((set_1.len() - mutual.len()) * (set_2.len() - mutual.len())) as i64;
            }
        }
    }

    result
}
