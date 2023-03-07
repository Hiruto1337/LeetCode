fn main() {
    println!("{}", my_atoi(String::from("-100000000000000000000000000000000000000000000000000")));
}

pub fn my_atoi(s: String) -> i32 {
    let mut s = s.trim_start();

    let (s, sign) = match s.strip_prefix('-') {
        Some(str) => (str, -1),
        None => (s.strip_prefix('+').unwrap_or(s), 1)
    };

    s.chars()
        .map(|char| char.to_digit(10))
        .take_while(Option::is_some)
        .flatten()
        .fold(0, |acc: i32, x| {
            acc.saturating_mul(10).saturating_add(sign * x as i32)
        })
}
