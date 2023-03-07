fn main() {
    // println!("{}", my_pow(2.0, 10));
    // println!("{}", my_pow(1.00001, 12));
    // println!("{}", my_pow(1.00001, 2147483647));
    // println!("{}", my_pow(1.00001, 20));
    println!("{}", my_pow(1.00001, -2147483648));
}

fn my_pow(x: f64, n: i32) -> f64 {
    let (mut x, mut n) = (x, n);
    let div = n < 0;
    let mut rest = 1.0;

    if n == 0 {
        return 1.0;
    } else if n == -2147483648 {
        rest *= x;
        n += 1;
    }

    if div {
        n *= -1;
    }

    while n != 1 {
        while n % 2 == 0 {
            x *= x;
            n /= 2;
        }

        if n != 1 {
            n -= 1;
            rest *= x;
        }
    }

    if div {
        1.0 / (x * rest)
    } else {
        x * rest
    }
}

// 2^10
// (2^2)^(10/2) -> 4^5 -> (4^4) * 4
// (4^2)^(4/2) -> 16^2
// (16^2)^(2/2) -> 32^1
