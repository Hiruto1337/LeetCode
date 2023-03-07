fn main() {
    let roads: Vec<Vec<i32>> = vec![
        vec![1, 0],
        vec![1, 2],
        vec![0, 3],
        vec![4, 3],
        vec![5, 2],
        vec![4, 6],
        vec![1, 7],
        vec![8, 6],
        vec![9, 6],
        vec![1, 10],
        vec![6, 11],
        vec![1, 12],
        vec![13, 9],
        vec![4, 14],
        vec![3, 15],
        vec![2, 16],
        vec![5, 17],
        vec![3, 18],
        vec![6, 19],
        vec![6, 20],
        vec![21, 16],
        vec![18, 22],
        vec![0, 23],
        vec![24, 1],
        vec![25, 12],
        vec![26, 24],
        vec![9, 27],
        vec![28, 23],
        vec![29, 25],
        vec![6, 30],
        vec![31, 21],
        vec![32, 21],
        vec![33, 23],
        vec![19, 34],
        vec![5, 35],
        vec![36, 7],
        vec![25, 37],
        vec![0, 38],
        vec![1, 39],
        vec![6, 40],
        vec![41, 3],
    ];

    // let roads: Vec<Vec<i32>> = vec![vec![0,1], vec![0,2], vec![1,3], vec![1,4]];

    let seats = 5;

    println!("{}", minimum_fuel_cost(roads, seats));
}

fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
    let nodes = roads.len() + 1;
    let mut neighbors: Vec<Vec<i32>> = vec![vec![]; nodes];
    let mut degree: Vec<i32> = vec![0; nodes];

    for road in roads {
        neighbors[road[0] as usize].push(road[1]);
        neighbors[road[1] as usize].push(road[0]);

        degree[road[0] as usize] += 1;
        degree[road[1] as usize] += 1;
    }

    let mut queue: Vec<i32> = vec![];

    // Push the leaves to the queue
    for i in 1..degree.len() {
        if degree[i] == 1 {
            queue.push(i as i32);
        }
    }

    let mut people: Vec<i32> = vec![1; nodes];

    let mut result = 0;

    while queue.len() != 0 {
        let node = queue.remove(0);

        result += (people[node as usize] as f64 / seats as f64).ceil() as i64;

        for neighbor in &neighbors[node as usize] {
            degree[*neighbor as usize] -= 1;
            people[*neighbor as usize] += people[node as usize];

            if degree[*neighbor as usize] == 1 && *neighbor != 0 {
                queue.push(*neighbor);
            }
        }
    }

    result
}

// Start at the outermost points
// Make every car at the outskirts move inward by one node
// Reevaluate passenger capacity
// Make every car at the outskirts move inward by one node
