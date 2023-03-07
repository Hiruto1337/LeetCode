fn main() {
    let s = "A".to_string();

    println!("{}", convert(s, 1));
}

fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    } else {
        let s: Vec<char> = s.chars().collect();

        let mut array: Vec<Vec<char>> = vec![];

        for _ in 0..num_rows {
            array.push(vec![]);
        }

        let mut ptr = 0;
        let mut prog = 0;

        while prog < s.len() {
            while ptr < num_rows as usize - 1 && prog < s.len() {
                array[ptr].push(s[prog]);
                ptr += 1;
                prog += 1;
            }

            while 0 < ptr && prog < s.len() {
                array[ptr].push(s[prog]);
                ptr -= 1;
                prog += 1;
            }
        }

        return array.into_iter().flatten().collect::<String>();
    }
}

// Create an array, Vec<&str>
// Iterate through s
// Add &str at every index until index == num_rows - 1

// 0   0
//  1 1 1
//   2   2
