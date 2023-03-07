fn main() {
    println!("{:?}", generate_parenthesis(4));
}

fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut combinations: Vec<String> = vec![];
    let mut iteration_results: Vec<Vec<String>> = vec![vec![String::from("()")]];
    
    for _i in 0..n - 1 {
        let mut new_combinations: Vec<String> = vec![];

        for combination in iteration_results[iteration_results.len() - 1].to_owned() {

            let mut combination_vec: Vec<&str> = combination.split("").collect();
            combination_vec.retain(|&str| str != "");

            for i in 0..combination_vec.len() {
                let mut vec_copy = combination_vec.to_owned();
                vec_copy.insert(i, "()");
                let vec_copy_string = vec_copy.join("");

                if !new_combinations.contains(&vec_copy_string) {
                    new_combinations.push(vec_copy_string);
                }
            }
        }

        iteration_results.push(new_combinations);
    }

    for result in iteration_results[n as usize - 1].to_owned() {
        combinations.push(result);
    }

    combinations
}