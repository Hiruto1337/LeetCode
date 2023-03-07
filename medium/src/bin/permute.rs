fn main() {
    println!("{:?}", permute(vec![1, 2, 3, 4, 5]).len());
}

fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn get_combination_tree(leftover: &mut Vec<i32>, log: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if leftover.len() == 0 {
            result.push(log.to_owned());
        } else {
            for i in 0..leftover.len() {
                let mut leftover_copy = leftover.to_owned();

                let mut log_copy = log.to_owned();

                log_copy.push(leftover_copy[i]);
                leftover_copy.remove(i);

                get_combination_tree(&mut leftover_copy, &mut log_copy, result);
            }
        }
    }

    let mut nums = nums;

    let mut result: Vec<Vec<i32>> = vec![];

    get_combination_tree(&mut nums, &mut vec![], &mut result);

    result
}