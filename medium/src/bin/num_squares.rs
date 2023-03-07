fn main(){
    let n = 43;
    // 25 + 9 + 9
    
    println!("{}", num_squares(n));
}

fn num_squares(n: i32) -> i32 {
    fn get_min(result: i32, count: i32, min: &mut i32, squares: Vec<i32>) {
        if result == 0 {
            *min = count;
            return;
        }

        if count + 1 < *min {
            for i in (0..squares.len()).rev() {
                if squares[i] <= result  {
                    get_min(result - squares[i], count + 1, min, squares[0..=i].to_vec());
                }
            }
        }
    }

    let mut squares: Vec<i32> = vec![];

    let mut root = 1;

    while i32::pow(root, 2) <= n {
        squares.push(i32::pow(root, 2));
        root += 1;
    }

    let mut min = i32::MAX;

    get_min(n, 0, &mut min, squares);

    min
}

// Fill an array with all squares that are smaller than n
// Keep subtracting the largest square, until n becomes smaller than the largest square
// Find the largest square that is smaller than n
// Repeat