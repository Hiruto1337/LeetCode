fn main() {
    println!("{}", str_str(String::from("llboinfibapsadisobfpiebfpibpdasndpw"), String::from("sad")));
}

fn str_str(haystack: String, needle: String) -> i32 {
    if let Some(index) = haystack.find(&needle) {
        return index as i32;
    }

    -1
}