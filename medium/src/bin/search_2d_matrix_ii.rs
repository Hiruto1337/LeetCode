fn main() {
    let matrix = vec![
        vec![1, 4, 7, 11, 15],
        vec![2, 5, 8, 12, 19],
        vec![3, 6, 9, 16, 22],
        vec![10, 13, 14, 17, 24],
        vec![18, 21, 23, 26, 30],
    ];

    let target = 5;

    println!("{}", search_matrix(matrix, target));
}

fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let (mut left, mut top, mut right, mut bottom) = (0, 0, matrix[0].len() - 1, matrix.len() - 1);

    while target < matrix[0][right] && right != 0 {
        right -= 1;
    }

    while target < matrix[bottom][0] && bottom != 0 {
        bottom -= 1;
    }

    while matrix[top][right] < target && top != matrix.len() - 1 {
        top += 1;
    }

    while matrix[bottom][left] < target && left != matrix[0].len() - 1 {
        left += 1;
    }

    for y in top..=bottom {
        for x in left..=right {
            if matrix[y][x] == target {
                return true;
            }
        }
    }

    false
}
