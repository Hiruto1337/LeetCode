fn main() {
    println!("{}", reverse(-696969));
}

fn reverse(x: i32) -> i32 {
    let x_string: String = x.to_string();

    let mut x_vec: Vec<&str> = x_string.split("").collect();

    x_vec.retain(|&str| str != "");

    if x_vec[0] == "-" {
        x_vec.rotate_left(1);
        x_vec.pop();
        x_vec.reverse();
        x_vec.push("-");
        x_vec.rotate_right(1);
    } else {
        x_vec.reverse();
    }

    let rev_string: String = x_vec.join("");

    let rev_x = rev_string.parse::<i32>();

    match rev_x {
        Ok(num) => num,
        Err(_err) => 0
    }
}