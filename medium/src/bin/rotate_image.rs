fn main() {
    let mut matrix: Vec<Vec<i32>> = vec![
        vec![1,2,3],
        vec![4,5,6],
        vec![7,8,9]
    ];

    rotate(&mut matrix);

    println!("{:?}", matrix);
}

fn rotate(matrix: &mut Vec<Vec<i32>>) {
    for i in 0..matrix.len() {
        for j in (i + 1)..matrix.len() {
            let holder = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = holder;
        }
        matrix[i].reverse();
    }
}