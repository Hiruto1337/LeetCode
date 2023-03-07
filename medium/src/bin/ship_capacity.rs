fn main(){
    let weights = vec![1,2,3,4,5,6,7,8,9,10];

    let days = 1;

    println!("{}", ship_within_days(weights, days));
}

fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
    let mut min_capacity = 0;
    let mut max_capacity = 0;

    for i in 0..weights.len() {
        max_capacity += weights[i];
        min_capacity = min_capacity.max(weights[i]);
    }

    while min_capacity < max_capacity {
        let capacity = (min_capacity + max_capacity) / 2;
        let mut weight = 0;
        let mut elapsed_days = 1;

        for i in 0..weights.len() {
            weight += weights[i];
            if capacity < weight {
                elapsed_days += 1;
                weight = weights[i];
            }
        }

        if elapsed_days < days {
            max_capacity = capacity;
        } else {
            min_capacity = capacity + 1;
        }
    }

    min_capacity
}
