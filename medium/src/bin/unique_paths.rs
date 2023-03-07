fn main(){
    println!("{}", unique_paths(3, 7));
}

fn unique_paths(m: i32, n: i32) -> i32 {
    fn make_row_from(array: &mut Vec<i32>) {
        let mut procedural_row = vec![1];

        for i in 1..array.len() {
            procedural_row.push(array[i] + procedural_row[i - 1]);
        }

        *array = procedural_row;
    }

    let mut array = vec![1;n as usize];

    for _ in 1..m {
        make_row_from(&mut array);
    }

    array[array.len() - 1]
}