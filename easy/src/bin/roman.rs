fn main() {
    println!("{}", roman_to_int(String::from("MCMXCIV")));
}

pub fn roman_to_int(s: String) -> i32 {
    let mut string_remains: String = s.to_owned();

    let mut result: i32 = 0;

    // Account for problematic numbers
    let sub_pairs: [(&str, i32); 6] = [
        ("CM", 900),
        ("CD", 400),
        ("XC", 90),
        ("XL", 40),
        ("IX", 9),
        ("IV", 4),
    ];

    for pair in sub_pairs {
        if string_remains.contains(pair.0) {
            result += pair.1;
            string_remains = string_remains.split(pair.0).collect();
        }
    }

    // Account for the rest
    let pairs: [(&str, i32); 7] = [
        ("I", 1),
        ("V", 5),
        ("X", 10),
        ("L", 50),
        ("C", 100),
        ("D", 500),
        ("M", 1000),
    ];

    for pair in pairs {
        if string_remains.contains(pair.0) {
            let amount = string_remains.matches(pair.0).count();

            for _i in 0..amount {
                result += pair.1;
            }
        }
    }

    result
}
