fn main() {
    let n = 10000;

    println!("{}", count_primes(n));
}

fn count_primes(n: i32) -> i32 {
    use std::collections::HashSet;

    if n > 2 {
        let mut set: HashSet<i32> = HashSet::from([2]);

        let mut total = 1;

        for i in (3..n).step_by(2) {
            if let None = set.get(&i) {
                total += 1;
                let mut acc = i;

                while acc + i < n {
                    acc += i;
                    set.insert(acc);
                }
            }
        }

        return total;
    }

    0
}
