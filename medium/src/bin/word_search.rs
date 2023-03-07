fn main() {
    let board: Vec<Vec<char>> = vec![
        vec!['A', 'A', 'A', 'A', 'A', 'A'],
        vec!['A', 'A', 'A', 'A', 'A', 'A'],
        vec!['A', 'A', 'A', 'A', 'A', 'A'],
        vec!['A', 'A', 'A', 'A', 'A', 'A'],
        vec!['A', 'A', 'A', 'A', 'A', 'A'],
        vec!['A', 'A', 'A', 'A', 'A', 'A'],
    ];

    let word: String = String::from("AAAAAAAAAAAAAAa");

    println!("{}", exist(board, word.to_string()));
}

fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let mut unique_symbols: Vec<char> = vec![];

    for char in word.chars().collect::<Vec<char>>() {
        if !unique_symbols.contains(&char) {
            unique_symbols.push(char);
        }
    }

    let mut possible = true;

    for symbol in unique_symbols {
        let mut symbol_exists = false;

        for row in board.iter() {
            if row.contains(&symbol) {
                symbol_exists = true;
                break;
            }
        }

        if !symbol_exists {
            possible = false;
            break;
        }
    }

    if !possible {
        return false;
    }

    fn check_neighbors(
        x: usize,
        y: usize,
        board: Vec<Vec<char>>,
        remains: Vec<char>,
        banned_pos: Vec<(usize, usize)>,
        exists: &mut bool,
    ) {
        if remains.len() == 0 {
            *exists = true;
        } else {
            let up = y as i32 - 1;
            let down = y as i32 + 1;
            let left = x as i32 - 1;
            let right = x as i32 + 1;

            // Check above
            if 0 <= up {
                if board[up as usize][x] == remains[0] {
                    // Make sure position hasn't been used
                    let used = check_available(x, up as usize, banned_pos.to_owned());

                    if !used {
                        let mut banned = banned_pos.to_owned();
                        banned.push((x, up as usize));
                        check_neighbors(
                            x,
                            up as usize,
                            board.to_owned(),
                            remains[1..remains.len()].to_vec(),
                            banned,
                            exists,
                        )
                    }
                }
            }

            // Check below
            if down as usize <= board.len() - 1 {
                if board[down as usize][x] == remains[0] {
                    // Make sure position hasn't been used
                    let used = check_available(x, down as usize, banned_pos.to_owned());

                    if !used {
                        let mut banned = banned_pos.to_owned();
                        banned.push((x, down as usize));
                        check_neighbors(
                            x,
                            down as usize,
                            board.to_owned(),
                            remains[1..remains.len()].to_vec(),
                            banned,
                            exists,
                        )
                    }
                }
            }

            // Check left
            if 0 <= left {
                if board[y][left as usize] == remains[0] {
                    // Make sure position hasn't been used
                    let used = check_available(left as usize, y as usize, banned_pos.to_owned());

                    if !used {
                        let mut banned = banned_pos.to_owned();
                        banned.push((left as usize, y));
                        check_neighbors(
                            left as usize,
                            y,
                            board.to_owned(),
                            remains[1..remains.len()].to_vec(),
                            banned,
                            exists,
                        )
                    }
                }
            }

            // Check right
            if right as usize <= board[0].len() - 1 {
                if board[y][right as usize] == remains[0] {
                    // Make sure position hasn't been used
                    let used = check_available(right as usize, y as usize, banned_pos.to_owned());

                    if !used {
                        let mut banned = banned_pos.to_owned();
                        banned.push((right as usize, y));
                        check_neighbors(
                            right as usize,
                            y,
                            board.to_owned(),
                            remains[1..remains.len()].to_vec(),
                            banned,
                            exists,
                        )
                    }
                }
            }
        }
    }

    fn check_available(x: usize, y: usize, banned_pos: Vec<(usize, usize)>) -> bool {
        let mut used = false;

        for pos in banned_pos {
            if pos == (x, y) {
                used = true;
            }
        }

        used
    }

    let word_vec: Vec<char> = word.chars().collect();

    let mut exists = false;

    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == word_vec[0] && exists == false {
                check_neighbors(
                    j,
                    i,
                    board.to_owned(),
                    word_vec[1..word_vec.len()].to_vec(),
                    vec![(j, i)],
                    &mut exists,
                );
            }
        }
    }

    exists
}
