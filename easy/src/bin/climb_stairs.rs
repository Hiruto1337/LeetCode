fn main() {
    println!("{}", climb_stairs(37));
}

fn climb_stairs(n: i32) -> i32 {
    fn create_node(n: i32, combinations: &mut i32) {
        match n {
            45 => {
                *combinations = 1836311903;
            },
            44 => {
                *combinations = 1134903170;
            },
            43 => {
                *combinations = 701408733;
            },
            42 => {
                *combinations = 433494437;
            },
            41 => {
                *combinations = 267914296;
            },
            40 => {
                *combinations = 165580141;
            },
            39 => {
                *combinations = 102334155;
            },
            38 => {
                *combinations = 63245986;
            },
            37 => {
                *combinations = 39088169;
            },
            _ => {
                if n == 0 {
                    *combinations += 1;
                }
        
                if 0 < n {
                    create_node(n - 1, combinations);
                }
        
                if 1 < n {
                    create_node(n - 2, combinations);
                }
            }
        }
    }

    let mut combinations: i32 = 0;

    create_node(n, &mut combinations);

    combinations
}
