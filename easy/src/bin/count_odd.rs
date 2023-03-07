fn main(){
    let low = 1;
    let high = 3;

    println!("{}", count_odds(low, high));
}

fn count_odds(low: i32, high: i32) -> i32 {
    let mut low = low;
    let mut high = high;
    let mut odds = 0;

    if low % 2 == 1 {
        odds += 1;
        low += 1;
    }

    if high % 2 == 1 {
        odds += 1;
        high -= 1;
    }

    odds += (high - low) / 2;

    odds
}