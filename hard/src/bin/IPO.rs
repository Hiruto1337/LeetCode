fn main() {
    let k = 3;
    let w = 0;
    let profits = vec![1,2,10,3];
    let capital = vec![0,1,2,2];

    println!("{}", find_maximized_capital(k, w, profits, capital));
}

fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut w = w;

        if capital.len() <= k as usize {
            let max_capital = *capital.iter().max().unwrap();

            if w >= max_capital {
                return w + profits.iter().sum::<i32>();
            }
        }

        use std::collections::{HashMap, HashSet};
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

        let mut unique_profits: HashSet<i32> = HashSet::new();

        for (i, profit) in profits.iter().enumerate() {
            if let None = unique_profits.get(profit) {
                unique_profits.insert(*profit);
            }

            map.entry(*profit)
                .and_modify(|list| list.push(capital[i]))
                .or_insert(vec![capital[i]]);
        }

        for (profit, _) in map.to_owned() {
            map.entry(profit).and_modify(|list| list.sort());
        }

        let mut profits: Vec<i32> = unique_profits.into_iter().collect();
        profits.sort();

        let mut projects = 0;

        while projects != k {
            let mut none = true;

            for profit in profits.iter().rev() {
                let capitals = map.get(profit).unwrap().to_owned();

                if capitals.len() != 0 && capitals[0] <= w {
                    w += *profit;
                    map.entry(*profit).and_modify(|list| {
                        list.remove(0);
                    });
                    projects += 1;
                    none = false;
                    break;
                }
            }

            if none {
                return w;
            }
        }

        w
}

// Create a hashmap that maps a profit to a sorted list of all capitals (via binary search)
// Create a sorted list of all profits
// Start iterating through the profits list from high to low
// -> Get a sorted list of all capitals
// -> Make a binary search for the current capital, w
// ---> If a capital smaller than or equal to w is found, add profits to w and remove "capitals" from the list
// ---> Repeat the whole process
// -> Go to the next profit
