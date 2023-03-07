fn main() {
    let mut a = "1111".to_string();
    let mut b = "1111".to_string();

    println!("{}", add_binary(a, b));
}

fn add_binary(a: String, b: String) -> String {
    let (short, long) = if a.len() > b.len() { (b, a) } else { (a, b) };

    let mut short: Vec<char> = short.chars().collect();
    let long: Vec<char> = long.chars().collect();

    while long.len() > short.len() {
        short.insert(0, '0');
    }

    let mut sum: Vec<char> = vec![];

    let mut holder = false;

    for i in (0..long.len()).rev() {
        if short[i] == '1' && long[i] == '1' {
            if holder {
                sum.push('1');
            } else {
                sum.push('0');
                holder = true;
            }
        } else if short[i] == '1' || long[i] == '1' {
            if holder {
                sum.push('0');
            } else {
                sum.push('1');
            }
        } else {
            if holder {
                sum.push('1');
                holder = false;
            } else {
                sum.push('0');
            }
        }
    }

    if holder {
        sum.push('1');
    }

    sum.reverse();

    sum.iter().collect::<String>()
}
