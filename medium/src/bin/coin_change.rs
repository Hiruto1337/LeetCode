fn main(){
    // let coins = vec![411,412,413,414,415,416,417,418,419,420,421,422];
    // let amount = 9864;
    let coins = vec![186,419,83,408];
    let amount = 6249;

    println!("{}", coin_change(coins, amount));
}

fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut dp: Vec<i32> = vec![10001; (amount + 1) as usize];

    dp[0] = 0;

    for val in 1..=(amount as usize) {
        for coin in &coins {
            let result = val as i32 - coin;

            if 0 <= result {
                dp[val] = dp[val].min(1 + dp[result as usize]);
            }
        }
    }

    if dp[amount as usize] != 10001 {
        return dp[amount as usize];
    }

    -1
}

// Create a HashMap that maps remains to a minimum count
// Remains -> Count
