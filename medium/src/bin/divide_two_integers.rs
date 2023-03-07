fn main() {
    println!("{}", divide(-2147483648, -3));
}

fn divide(dividend: i32, divisor: i32) -> i32 {
    let mut dividend = dividend;
    let divisor = divisor;
    let mut sign = 1;

    let mut result = 0;

    // Save time by skipping divisions with one
    if divisor == 1 {
        return dividend;
    } else if divisor == -1 {
        if dividend == -2147483648 {
            return 2147483647;
        } else {
            return -dividend;
        }
    }

    // Save time by skipping divisions between equal numbers
    if dividend == divisor {
        return 1;
    }

    // Begin repeated subtraction
    if dividend < 0 && divisor < 0 {
        while dividend - divisor <= 0 {
            result += 1;
            dividend -= divisor;
        }
    } else if dividend < 0 {
        while dividend + divisor <= 0 {
            result += 1;
            dividend += divisor;
        }
        sign = -sign;
    } else if divisor < 0 {
        while dividend + divisor >= 0 {
            result += 1;
            dividend += divisor;
        }
        sign = -sign;
    } else {
        while dividend - divisor >= 0 {
            result += 1;
            dividend -= divisor;
        }
    }

    if sign < 0 {
        return -result;
    } else {
        return result;
    }
}
