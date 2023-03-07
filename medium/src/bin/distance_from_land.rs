fn main() {
    let mut grid = vec![
        vec![1, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 1],
    ];

    // for _ in 0..100 {
    //     let mut row: Vec<i32> = vec![1];

    //     for _ in 0..99 {
    //         row.push(0);
    //     }

    //     grid.push(row);
    // }

    println!("{}", max_distance(grid));
}

fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashSet;

    let mut not_visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: HashSet<(usize, usize)> = HashSet::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let (mut land, mut water) = (false, false);

    for y in 0..grid.len() {
        for x in 0..grid.len() {
            if grid[y][x] == 1 {
                queue.insert((x, y));
                land = true;
            } else {
                not_visited.insert((x, y));
                water = true;
            }
        }
    }

    if !water || !land {
        return -1;
    }

    let mut maximum = 0;

    while not_visited.len() != 0 {
        let old_queue = queue.to_owned();
        queue.drain();

        for (x, y) in old_queue {
            if let None = visited.get(&(x, y)) {
                if y != 0 {
                    queue.insert((x, y - 1));
                    not_visited.remove(&(x, y - 1));
                }

                if y != grid.len() - 1 {
                    queue.insert((x, y + 1));
                    not_visited.remove(&(x, y + 1));
                }

                if x != 0 {
                    queue.insert((x - 1, y));
                    not_visited.remove(&(x - 1, y));
                }

                if x != grid.len() - 1 {
                    queue.insert((x + 1, y));
                    not_visited.remove(&(x + 1, y));
                }

                visited.insert((x, y));
            }
        }

        maximum += 1;
    }
    maximum
}
