fn main() {
    let mut board: Vec<Vec<char>> = vec![
        vec!['X', 'X', 'X'],
        vec!['X', 'O', 'X'],
        vec!['O', 'X', 'X'],
    ];

    // for _ in 0..200 {
    //     let mut row: Vec<char> = vec![];
    //     for _ in 0..200 {
    //         row.push('O');
    //     }

    //     board.push(row);
    // }

    solve(&mut board);

    println!("{board:?}");
}

fn solve(board: &mut Vec<Vec<char>>) {
    let mut table: Vec<Vec<bool>> = vec![vec![false; board[0].len()]; board.len()];

    fn search(x: i32, y: i32, board: &mut Vec<Vec<char>>, table: &mut Vec<Vec<bool>>) {
        if 0 <= y && 0 <= x && x < board[0].len() as i32 && y < board.len() as i32 {
            if !(*table)[y as usize][x as usize] {
                if board[y as usize][x as usize] == 'O' {
                    (*table)[y as usize][x as usize] = true;

                    // Search up, left, right and down
                    search(x, y - 1, board, table);
                    search(x - 1, y, board, table);
                    search(x + 1, y, board, table);
                    search(x, y + 1, board, table);
                }
            }
        }
    }

    // Search top and bottom
    for i in 0..board[0].len() {
        search(i as i32, 0, board, &mut table);
        search(i as i32, board.len() as i32 - 1, board, &mut table);
    }

    // Search left and right
    for i in 0..board.len() {
        search(0, i as i32, board, &mut table);
        search(board[0].len() as i32 - 1, i as i32, board, &mut table);
    }

    for y in 0..table.len() {
        for x in 0..table[y].len() {
            match table[y][x] {
                true => (*board)[y][x] = 'O',
                false => (*board)[y][x] = 'X',
            }
        }
    }

    // Search outskirts of board (except for corners) ✅
    // When an 'O' is found, start recursive search in opposing direction (U, L, R, D) ✅
    // When finding an 'O', add the coordinate to "banned_list" ✅
    // Keep cycling until recursion comes to an end ✅
    // Convert all coordinates (within outskirts) not on the banned_list to 'X'
}
