fn main() {
    let digits: Vec<i32> = vec![4, 1, 9];
    // let digits: Vec<i32> = vec![9, 9, 9];

    println!("{:?}", plus_one(digits));
}

fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut original: Vec<i32> = digits.to_owned();

    original.reverse();

    for num in &mut original {
        if *num + 1 < 10 {
            *num += 1;
            break;
        } else {
            *num = 0;
        }
    }

    for num in &mut original {
        if *num != 0 {
            original.reverse();

            return original;
        }
    }
    
    original.push(1);

    original.reverse();

    original
}
