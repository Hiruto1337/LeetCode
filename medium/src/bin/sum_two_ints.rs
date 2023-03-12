fn main() {
    println!("{}", get_sum(-1, 1));
}

fn get_sum(a: i32, b: i32) -> i32 {
    let (a, b) = (format!("{a:b}"), format!("{b:b}"));
    let (a, b) = (a.chars().collect::<Vec<char>>(), b.chars().collect::<Vec<char>>());
    let (mut short, long) = if a.len() < b.len() {(a, b)} else {(b, a)};

    println!("{short:?}");
    println!("{long:?}\n");
    
    while short.len() != long.len() {
        short.insert(0, '0');
    }

    let mut stock = false;

    for i in (0..short.len()).rev() {
        if (short[i] == '1' && long[i] == '0') || (short[i] == '0' && long[i] == '1') {
            if stock {
                short[i] = '0';
            } else {
                short[i] = '1';
            }
        } else if short[i] == '0' && long[i] == '0' {
            if stock {
                short[i] = '1';
                stock = false;
            }
        } else if short[i] == '1' && long[i] == '1' {
            if stock {
                short[i] = '1';
                stock = true;
            } else {
                stock = true;
                short[i] = '0';
            }
        }
    }

    if stock {
        short.insert(0, '1');
    }

    println!("{short:?}");

    i32::from_str_radix(&(short.iter().collect::<String>()), 2).unwrap()
}

