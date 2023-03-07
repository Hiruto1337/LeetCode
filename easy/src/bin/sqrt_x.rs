fn main() {
    println!("{}", my_sqrt(4));
}

fn my_sqrt(x: i32) -> i32 {
    let x: u32 = x as u32;
    let mut x_root: u32 = 0;

    loop {
        let x_root_squared = x_root * x_root;

        if x as u32 == x_root_squared {
            return x_root as i32;
        } else if x < x_root_squared {
            return (x_root - 1) as i32;
        } else {
            x_root += 1;
        }
    }
}