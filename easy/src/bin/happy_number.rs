fn main() {
    let num = 1;

    println!("{}", is_happy(num));
}

fn is_happy(n: i32) -> bool {
    if n == 1 {
        return true;
    }
    fn get_next(num: i32) -> i32 {
        let num_string = num.to_owned().to_string();
        let num_vec: Vec<char> = num_string.chars().collect();

        let mut new_num = 0;

        for number in num_vec.to_owned() {
            new_num += i32::pow(number.to_digit(10).unwrap() as i32, 2);
        }

        new_num
    }

    let mut tortoise = n;

    let mut hare = get_next(n);

    while hare != tortoise {
        let hare_single = get_next(hare);

        let hare_double = get_next(hare_single);

        if hare_single == 1 || hare_double == 1 {
            return true;
        }

        tortoise = get_next(tortoise);

        hare = hare_double;
    }

    false
}