fn main() {
    println!("{}", is_palindrome(123454321));
}

fn is_palindrome(x: i32) -> bool {
    let x_string: String = x.to_owned().to_string();

    let original_order: Vec<&str> = x_string.split("").collect();

    let mut reversed_order: Vec<&str> = vec![];

    for i in 0..original_order.len() {
        reversed_order.push(original_order[original_order.len() - 1 - i]);
    }

    original_order == reversed_order
}
