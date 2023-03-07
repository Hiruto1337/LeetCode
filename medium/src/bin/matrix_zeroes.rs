fn main() {
    let mut matrix = vec![];

    let mut row = vec![0;200];

    for _ in 0..200 {
        matrix.push(row.to_owned());
    }

    set_zeroes(&mut matrix);

    println!("{:?}", matrix);
}

fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    fn new_node(x: usize, y: usize, matrix: &mut Vec<Vec<i32>>) {
        let mut found = false;

        // Search rest of row
        for i in x + 1..matrix[y].len() {
            if matrix[y][i] == 0 {
                found = true;
                new_node(i, y, matrix);
                break;
            }
        }

        // Search rest of matrix
        for i in y + 1..matrix.len() {
            if !found {
                for j in 0..matrix[y].len() {
                    if matrix[i][j] == 0 {
                        found = true;
                        new_node(j, i, matrix);
                        break;
                    }
                }
            } else {
                break;
            }
        }

        // Convert row to 0
        for i in 0..matrix[y].len() {
            matrix[y][i] = 0;
        }

        // Convert column to 0
        for i in 0..matrix.len() {
            matrix[i][x] = 0;
        }
    }

    let mut found = false;

    for y in 0..matrix.len() {
        if !found {
            for x in 0..matrix[0].len() {
                if matrix[y][x] == 0 {
                    found = true;
                    new_node(x, y, matrix);
                    break;
                }
            }
        } else {
            break;
        }
    }
}
