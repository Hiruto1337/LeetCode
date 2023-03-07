fn main() {
    let num = vec![9,9,9];
    let k = 1;

    println!("{:?}", add_to_array_form(num, k));
}

fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
    let k: Vec<i32> = k.to_string().chars().map(|char| char.to_digit(10).unwrap() as i32).collect();

    let (mut short, long) = if num.len() > k.len() {(k, num)} else {(num, k)};

    while short.len() < long.len() {
        short.insert(0, 0);
    }

    let mut holder = false;

    for i in (0..short.len()).rev() {
        let mut sum = if holder {
            holder = false;
            short[i] + long[i] + 1
        } else {short[i] + long[i]};

        if sum > 9 {
            holder = true;
            sum -= 10;
        }

        short[i] = sum;
    }

    if holder {
        short.insert(0, 1);
    }

    short
}

// Convert k to a Vec<i32> (i32 -> String -> Vec<char> -> Vec<i32>)
// Keep adding zeros to the front of the shortest Vec<i32>
// Start adding the two Vec<i32> from the start
// If the sum of two integers is greater than 9, subtract 9 from it,
// ... place it into the result and set holder to true

