fn main() {
    println!("{:?}", generate(5));
}

fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut rows: Vec<Vec<i32>> = vec![vec![1], vec![1, 1]];
    
    if num_rows == 1 {
        return vec![vec![1]];
    } else if num_rows == 2 {
        return rows;
    }

    for i in 1..num_rows - 1 {
        let mut container: Vec<i32> = vec![];
        for j in 0..rows[i as usize].len() - 1 {
            container.push(rows[i as usize][j] + rows[i as usize][j + 1]);
        }
        container.push(1);
        container.push(1);
        container.rotate_right(1);

        rows.push(container);
    }

    rows
}