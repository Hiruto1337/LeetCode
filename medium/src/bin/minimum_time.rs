fn main() {
    let time = vec![1,2,3];
    let total_trips = 5;

    println!("{}", minimum_time(time, total_trips));
}

fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
    let total_trips = total_trips as i64;
    let min = *time.iter().min().unwrap() as i64;

    let mut left = 1;
    let mut right = min * total_trips;

    while left < right {
        let mid_time = left + (right - left) / 2;

        let trips = time.iter().fold(0, |acc, &x| acc + mid_time / x as i64);

        if trips < total_trips {
            left = mid_time + 1;
        } else {
            right = mid_time;
        }
    }

    left
}
