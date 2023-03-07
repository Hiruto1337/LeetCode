fn main() {
    let board = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    println!("{:?}", is_valid_sudoku(board));
}

fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if board[i][j] != '.' {
                let num = board[i][j];
                let mut matches = 0;

                // Check row for duplicates
                for k in 0..board[i].len() {
                    if board[i][k] == num {
                        matches += 1;
                    }
                }
                // Check column for duplicates
                for k in 0..board.len() {
                    if board[k][j] == num {
                        matches += 1;
                    }
                }

                // Get square
                let vertical: Vec<usize>;

                if i <= 2 {
                    vertical = vec![0, 1, 2];
                } else if 3 <= i && i <= 5 {
                    vertical = vec![3, 4, 5];
                } else {
                    vertical = vec![6, 7, 8];
                }

                let horizontal: Vec<usize>;

                if j <= 2 {
                    horizontal = vec![0, 1, 2];
                } else if 3 <= j && j <= 5 {
                    horizontal = vec![3, 4, 5];
                } else {
                    horizontal = vec![6, 7, 8];
                }

                for k in vertical.to_owned() {
                    for l in horizontal.to_owned() {
                        if board[k][l] == num {
                            matches += 1;
                        }
                    }
                }

                if matches != 3 {
                    return false;
                }
            }
        }
    }

    true
}
