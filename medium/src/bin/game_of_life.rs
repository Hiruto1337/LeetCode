fn main(){
    let mut board = vec![vec![0,1,0],vec![0,0,1],vec![1,1,1],vec![0,0,0]];

    game_of_life(&mut board);

    println!("{board:?}");
}

fn game_of_life(board: &mut Vec<Vec<i32>>) {
    fn evaluate(x: usize, y: usize, board: &Vec<Vec<i32>>, new_board: &mut Vec<Vec<i32>>) {
        let mut neighbors = 0;
        let top = if y == 0 {0} else {y - 1};
        let bottom = if y == board.len() - 1 {board.len() - 1} else {y + 1};
        let left = if x == 0 {0} else {x - 1};
        let right = if x == board[0].len() - 1 {board[0].len() - 1} else {x + 1};

        for y_local in top..=bottom {
            for x_local in left..=right {
                if board[y_local][x_local] == 1 {
                    neighbors += 1;
                }
            }
        }

        if board[y][x] == 1 {
            neighbors -= 1;
            if neighbors < 2 || 3 < neighbors {
                new_board[y][x] = 0;
            }
        } else {
            if neighbors == 3 {
                new_board[y][x] = 1;
            }
        }
    }

    let mut new_board: Vec<Vec<i32>> = board.to_owned();

    for y in 0..board.len() {
        for x in 0..board[0].len() {
            evaluate(x, y, board, &mut new_board);
        }
    }

    *board = new_board;
}

// Go through every single node and check all eight neighboring nodes

// 0 or 1 neighbors -> Death
// 2 or 3 neighbors -> Stay alive
// More than three -> Death
// A 0 has three -> Resurrection