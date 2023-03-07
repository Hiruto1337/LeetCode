fn main() {
    println!("{:?}", fizz_buzz(1));
}

fn fizz_buzz(n: i32) -> Vec<String> {
    let mut array: Vec<String> = vec![];
    for i in 1..n + 1 {
        if i % 3 == 0 && i % 5 == 0 {
            array.push(String::from("FizzBuzz"));
        } else if i % 3 == 0 {
            array.push(String::from("Fizz"));
        } else if i % 5 == 0 {
            array.push(String::from("Buzz"));
        } else {
            array.push(String::from(i.to_string()));
        }
    }

    array
}