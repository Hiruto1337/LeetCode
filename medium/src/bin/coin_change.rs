fn main(){
    // let coins = vec![411,412,413,414,415,416,417,418,419,420,421,422];
    // let amount = 9864;
    let coins = vec![186,419,83,408];
    let amount = 6249;

    println!("{}", coin_change(coins, amount));
}

fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    use std::collections::HashMap;
    let mut coins = coins;
    coins.sort();

    let mut map: HashMap<i32, i32> = HashMap::new();

    let mut min_coins = -1;

    for coin in coins.iter().rev() {
        let mut accumulated = *coin;
        let mut count = 1;

        while accumulated <= amount {
            if accumulated == amount && min_coins == -1 {
                min_coins = count;
            }
            map.entry(accumulated).or_insert(count);
            accumulated += *coin;
            count += 1;
        }
    }

    for coin in coins.iter().rev() {
        let mut remains = amount;
        let mut count = 0;

        while remains >= 0 {
            if let Some(add_count) = map.get(&remains) {
                min_coins = min_coins.min(count + *add_count);
                break;
            }

            count += 1;
            remains -= *coin;
        }
    }

    println!("{map:?}");

    min_coins
}

// Create a HashMap that maps remains to a minimum count
// Remains -> Count
