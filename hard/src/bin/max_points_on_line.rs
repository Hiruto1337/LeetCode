fn main() {
    let mut points: Vec<Vec<i32>> = vec![];

    for i in 0..300 {
        let rand_x = i32::pow(i, i as u32 % 4);
        let rand_y = i32::pow(i / 3, i as u32 % 4);

        points.push(vec![rand_x, rand_y]);
    }

    println!("{}", max_points(points));
}

fn max_points(points: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;

    if points.len() == 1 {
        return 1;
    }

    let mut max = 1;

    let mut hashmaps: Vec<HashMap<String, i32>> = vec![];

    // Create a hashmap for every point and add 1 to every repeating slope
    for (i, point) in points.iter().enumerate() {
        let mut points_copy = points.to_owned();
        points_copy.remove(i);

        // Create a map of slopes and their recurrences
        let mut map: HashMap<String, i32> = HashMap::new();

        for o_point in points_copy {
            let rise = o_point[1] - point[1];
            let run = o_point[0] - point[0];

            if run == 0 {
                map.entry(20001.to_string())
                    .and_modify(|slope_val| *slope_val += 1)
                    .or_insert(2);
            } else {
                let slope = rise as f32 / run as f32;

                map.entry(slope.to_string())
                    .and_modify(|slope_val| *slope_val += 1)
                    .or_insert(2);
            }
        }

        hashmaps.push(map);
    }

    for hashmap in hashmaps {
        hashmap
            .iter()
            .for_each(|(_, &slope_count)| max = std::cmp::max(max, slope_count));
            println!("{hashmap:?}");
    }
    max
}
