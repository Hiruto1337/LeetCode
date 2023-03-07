fn main(){
    let n = 14348907;

    println!("{}", is_power_of_three(n));
}

fn is_power_of_three(n: i32) -> bool {
    let mut n = n;

    if n < 1 {
        return false;
    }

    while n % 3 == 0 {
        n /= 3;
    }

    if n == 1 {
        return true;
    } else {
        return false;
    }
}