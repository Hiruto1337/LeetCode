fn main() {
    let s: String = String::from("taco cat!");

    println!("{}", is_palindrome(s));
}

fn is_palindrome(s: String) -> bool {
    let mut s = s.to_lowercase();

    s.retain(|c: char| c.is_alphanumeric());

    let s_copy: String = s.to_owned();

    let mut s_copy_vec: Vec<&str> = s_copy.split("").collect();

    s_copy_vec.reverse();

    let s_reverse: String = s_copy_vec.join("");

    s == s_reverse
}