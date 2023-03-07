fn main(){}

fn is_anagram(s: String, t: String) -> bool {
    let mut s_vec: Vec<char> = s.chars().collect();
    let mut t_vec: Vec<char> = t.chars().collect();

    s_vec.sort();
    t_vec.sort();

    println!("s_vec: {s_vec:?}");
    println!("t_vec: {t_vec:?}");

    s_vec == t_vec
}