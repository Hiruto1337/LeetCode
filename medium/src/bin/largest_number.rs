fn main() {
    let nums = vec![432,43243];

    println!("{}", largest_number(nums));
}

fn largest_number(nums: Vec<i32>) -> String {
    let mut nums = nums;

    nums.sort_by(|a, b| {
        let a: Vec<char> = a.to_string().chars().collect();
        let b: Vec<char> = b.to_string().chars().collect();

        if a == b {
            return b.cmp(&a);
        }

        let mut ptr = 0;

        while a[ptr % a.len()] == b[ptr % b.len()] && ptr <= std::cmp::max(a.len(), b.len()) {
            ptr += 1;
        }

        b[ptr % b.len()].to_digit(10).unwrap().cmp(&(a[ptr % a.len()].to_digit(10).unwrap()))
    });

    let result = nums.iter().map(|&val| val.to_string()).collect::<String>();

    match result.parse::<i32>() {
        Ok(num) => {
            return num.to_string();
        },
        Err(_) => {
            return result;
        }
    }
}

// Create Vec<char> of both comparitors
// Get the shortest of the two
// Keep chipping off the longest while it is equal to shortest
// When the longest remains no longer contains the shortest, the remains are compared to the shortest
