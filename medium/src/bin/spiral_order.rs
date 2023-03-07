fn main() {
    let matrix: Vec<Vec<i32>> = vec![vec![1,2,3,4],vec![5,6,7,8],vec![9,10,11,12]];
    println!("{:?}", spiral_order(matrix));
}

fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut matrix = matrix;
    let mut order: Vec<i32> = vec![];

    loop {
        if matrix.len() != 0 && matrix[0].len() != 0 {
            // Read and delete top row
            for i in 0..matrix[0].len() {
                order.push(matrix[0][i]);
            }

            matrix.rotate_left(1);
            matrix.pop();
        } else {
            return order;
        }

        if matrix.len() != 0 && matrix[0].len() != 0 {
            // Read and delete right column
            for i in 0..matrix.len() {
                order.push(matrix[i][matrix[0].len() - 1]);
            }

            for i in 0..matrix.len() {
                matrix[i].pop();
            }
        } else {
            return order;
        }

        if matrix.len() != 0 && matrix[0].len() != 0 {
            // Read and delete bottom row
            for i in 0..matrix[0].len() {
                order.push(matrix[matrix.len() - 1][matrix[0].len() - i - 1]);
            }

            matrix.pop();
        } else {
            return order;
        }

        if matrix.len() != 0 && matrix[0].len() != 0 {
            // Read and delete left column
            for i in 0..matrix.len() {
                order.push(matrix[matrix.len() - i - 1][0]);
            }

            for i in 0..matrix.len() {
                matrix[i].rotate_left(1);
                matrix[i].pop();
            }
        } else {
            return order;
        }
    }
}
