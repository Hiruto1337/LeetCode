fn main() {
    let mut chars = vec![
        'a','a','a','a','a','a','a','a','a','a',
    ];

    compress(&mut chars);

    println!("{chars:?}");
}

fn compress(chars: &mut Vec<char>) -> i32 {
    let mut ptr = 0;

    while ptr < chars.len() {
        let char = chars[ptr];
        let mut length = 1;
        let mut extend = ptr;

        while extend + 1 < chars.len() && chars[extend + 1] == char {
            length += 1;
            extend += 1;
        }

        if length != 1 {
            let digits_original = get_digits(length) as usize;

            while ptr + digits_original < extend {
                chars.remove(extend);
                extend -= 1;
            }

            extend = ptr + 1;
            while length != 0 {
                let digits = get_digits(length);
                let mut digit = 0;

                while get_digits(length) == digits {
                    length -= i32::pow(10, (digits - 1) as u32);
                    digit += 1;
                }

                chars[extend] = char::from_digit(digit, 10).unwrap();

                extend += 1;
            }

            while extend <= ptr + digits_original {
                chars[extend] = '0';
                extend += 1;
            }

            ptr = extend - 1;
        }

        ptr += 1;
    }

    chars.len() as i32
}

fn get_digits(num: i32) -> i32 {
    match num {
        x if x > 999 => 4,
        x if x > 99 => 3,
        x if x > 9 => 2,
        x if x > 0 => 1,
        _ => 0
    }
}
