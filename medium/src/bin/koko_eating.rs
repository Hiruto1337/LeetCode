fn main() {
    let piles = vec![1000000000];
    let h = 2;

    println!("{}", min_eating_speed(piles, h));
}

fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let max = piles.iter().max().unwrap().to_owned();

    if h as usize == piles.len() {
        return max;
    }

    let mut left = 1;
    let mut right = max;

    while left < right {
        let bph = left + (right - left) / 2;

        let spent = get_time(bph, piles.to_owned());

        if h < spent {
            left = bph + 1;
        } else {
            right = bph;
        }
    }

    left
}

fn get_time(bph: i32, piles: Vec<i32>) -> i32 {
    let mut time = 0;

    for pile in piles {
        time += (pile as f64 / bph as f64).ceil() as i32;
    }

    time
}
