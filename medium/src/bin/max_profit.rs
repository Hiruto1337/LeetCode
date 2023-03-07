fn main(){
    // Output should be 16
    let prices: Vec<i32> = vec![8,5,3,7,5,7,3,6,5,7,8,9,7,4,7];

    println!("{}", max_profit(prices));
}

fn max_profit(prices: Vec<i32>) -> i32 {
    prices.windows(2).fold(0, |acc, window| acc + std::cmp::max(window[1] - window[0], 0))
}