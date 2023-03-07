fn main() {
    let nums: Vec<i32> = vec![-69, 33, 420, 5000000, -69, 33, 5000000];

    println!("{}", single_number(nums));
}

fn single_number(nums: Vec<i32>) -> i32 {
    let mut logged: Vec<i32> = vec![];

    for num in nums {
        if logged.contains(&num) {
            let num_index = logged.iter().position(|&number| number == num).unwrap();
            logged.remove(num_index);
        } else {
            logged.push(num);
        }
    }

    logged[0]
}