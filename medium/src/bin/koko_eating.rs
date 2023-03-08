fn main() {
    let piles = vec![1000000000];
    let h = 2;

    println!("{}", min_eating_speed(piles, h));
}

fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let mut right = piles.iter().max().unwrap().to_owned();

    if h as usize == piles.len() {
        return right;
    }

    let mut left = 1;

    while left < right {
        let bph = left + (right - left) / 2;

        let mut spent = 0;

        for pile in &piles {
            spent += (*pile as f64 / bph as f64).ceil() as i32;
        }

        if h < spent {
            left = bph + 1;
        } else {
            right = bph;
        }
    }

    left
}
