fn main() {
    let fruits: Vec<i32> = vec![1, 0, 1, 4, 1, 4, 1, 2, 3];

    println!("{}", total_fruit(fruits));
}

fn total_fruit(fruits: Vec<i32>) -> i32 {
    match fruits.len() {
        1 => 1,
        2 => 2,
        _ => {
            let mut basket = ((fruits[0], 1), (fruits[1], 1));

            let mut maximum = 0;

            for i in 2..fruits.len() {
                if (basket.0).0 == fruits[i] {
                    (basket.0).1 += 1;
                } else if (basket.1).0 == fruits[i] {
                    (basket.1).1 += 1;
                } else if (basket.0).0 != -1 && (basket.1).0 != -1 {
                    let (retain, overwrite) = if fruits[i - 1] == (basket.0).0 {
                        (&mut basket.0, &mut basket.1)
                    } else {
                        (&mut basket.1, &mut basket.0)
                    };

                    let mut ptr = i as i32 - 1;
                    let mut acc = 0;

                    while ptr >= 0 && fruits[ptr as usize] == (*retain).0 {
                        acc += 1;
                        ptr -= 1;
                    }

                    (*retain).1 = acc;
                    *overwrite = (fruits[i], 1);
                }

                maximum = maximum.max((basket.0).1 + (basket.1).1);
            }

            maximum
        }
    }
}

// Iterate through "fruits"
// Whenever a new number is found, start a new counter and end all counters with two numbers in them
