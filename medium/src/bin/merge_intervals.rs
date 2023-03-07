fn main() {
    let mut intervals: Vec<Vec<i32>> = vec![];

    for i in 0..10000 {
        intervals.push(vec![(10000 - i - 1), (10000 - i)]);
    }

    println!("{:?}", merge(intervals));
}

fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use std::cmp::{max, min};

    let mut intervals = intervals;

    intervals.sort_by(|a, b| a[0].cmp(&b[0]));

    let mut skip_indices: Vec<usize> = vec![];

    let mut merged: Vec<Vec<i32>> = vec![];

    for i in 0..intervals.len() {
        if skip_indices.contains(&i) {
            continue;
        } else {
            let mut interval = intervals[i].to_owned();

            for j in i + 1..intervals.len() {
                if skip_indices.contains(&j) {
                    continue;
                } else {
                    let interval_2 = intervals[j].to_owned();

                    if (interval_2[0] <= interval[0] && interval[0] <= interval_2[1])
                        || (interval_2[0] <= interval[1] && interval[1] <= interval_2[1])
                        || (interval[0] <= interval_2[0] && interval_2[0] <= interval[1])
                        || (interval[0] <= interval_2[1] && interval_2[1] <= interval[1])
                    {
                        let min = min(interval[0], interval_2[0]);
                        let max = max(interval[1], interval_2[1]);
                        interval = vec![min, max];
                        skip_indices.push(j);
                    }
                }
            }

            merged.push(interval);
        }
    }

    merged
}
