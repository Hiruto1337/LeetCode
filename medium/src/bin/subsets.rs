fn main() {
    let nums: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    println!("{:?}", subsets(nums).len());
}

fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn get_subsets(array: Vec<i32>, current: Vec<i32>, result: &mut Vec<Vec<i32>>) {
        let mut current = current;

        current.push(array[0]);
        for i in 1..array.len() {
            get_subsets(array[i..array.len()].to_vec(), current.to_owned(), result);
        }

        result.push(current);
    }

    let mut result: Vec<Vec<i32>> = vec![vec![]];

    for i in 0..nums.len() {
        get_subsets(nums[i..nums.len()].to_vec(), Vec::new(), &mut result);
    }

    result
}