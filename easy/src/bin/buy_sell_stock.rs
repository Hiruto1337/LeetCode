fn main() {
    let prices = vec![2,1,4];
    println!("{}", max_profit(prices));
}

fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min = prices[0];
    let mut max_diff = 0;

    for i in 0..prices.len() {
        if prices[i] < min {
            min = prices[i];
        } else if min < prices[i] {
            if prices[i] - min > max_diff {
                max_diff = prices[i] - min;
            }
        }
    }

    max_diff
}