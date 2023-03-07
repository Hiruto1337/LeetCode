fn main() {
    let str1 = "ABABABAB".to_string();
    let str2 = "ABAB".to_string();

    println!("{}", gcd_of_strings(str1, str2));
}

fn gcd_of_strings(str1: String, str2: String) -> String {
    let mut divisor = if str1.len() < str2.len() {
        str1.to_owned()
    } else {
        str2.to_owned()
    };

    while divisor.len() != 0 {
        if str1.len() % divisor.len() != 0 || str2.len() % divisor.len() != 0 {
            divisor.pop();
            continue;
        }

        let mut str1_copy = str1.to_owned();

        while str1_copy.len() >= divisor.len() && str1_copy[(str1_copy.len() - divisor.len())..].to_string() == divisor {
            str1_copy = str1_copy[..(str1_copy.len() - divisor.len())].to_string();
        }

        let mut str2_copy = str2.to_owned();

        while str2_copy.len() >= divisor.len() && str2_copy[(str2_copy.len() - divisor.len())..].to_string() == divisor {
            str2_copy = str2_copy[..(str2_copy.len() - divisor.len())].to_string();
        }

        if str1_copy.len() == 0 && str2_copy.len() == 0 {
            return divisor;
        } else {
            divisor.pop();
        }
    }

    "".to_string()
}

// Get the shortest string and store it as "divisor"
// If str1.len() % divisor.len() != 0 || str2.len() % divisor.len() != 0, remove a letter from divisor
// Store all matches in an array
// Try reducing both str1 and str2 to length 0 with every match in array until it's complete
// Start with the longest
