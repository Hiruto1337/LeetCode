fn main() {
    println!("{}", is_valid(String::from("{([[]])}")));
}

fn is_valid(s: String) -> bool {
    let mut remains: String = s.to_owned();

    let valid_combinations = ["()", "[]", "{}"];

    loop {
        let remains_length_before_trimming = remains.to_owned().len();

        for combination in valid_combinations {
            let remains_minus_combination = remains.split(combination).collect::<Vec<&str>>();
            remains = remains_minus_combination.join("");
        }

        let remains_length_after_trimming = remains.to_owned().len();

        if remains_length_before_trimming == remains_length_after_trimming {
            break
        }
    }

    if remains.len() == 0 {
        return true;
    } else {
        return false;
    }
}