fn main(){
    let n = 4;
    let red_edges: Vec<Vec<i32>> = vec![
        vec![1,3],
        vec![2,1],
    ];
    let blue_edges: Vec<Vec<i32>> = vec![
        vec![0,1],
        vec![1,2],
        vec![3,2],
    ];

    println!("{:?}", shortest_alternating_paths(n, red_edges, blue_edges));
}

fn shortest_alternating_paths(n: i32, red_edges: Vec<Vec<i32>>, blue_edges: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::{HashMap, HashSet};

    // Create a map of red edges
    let mut red_map: HashMap<i32, HashSet<i32>> = HashMap::new();

    for pair in red_edges {
        red_map.entry(pair[0]).and_modify(|set| {(*set).insert(pair[1]);}).or_insert(HashSet::from([pair[1]]));
    }

    // Create a map of blue edges
    let mut blue_map: HashMap<i32, HashSet<i32>> = HashMap::new();

    for pair in blue_edges {
        blue_map.entry(pair[0]).and_modify(|set| {(*set).insert(pair[1]);}).or_insert(HashSet::from([pair[1]]));
    }

    // Set all results to -1
    let mut result: Vec<i32> = vec![-1 ; n as usize];

    // Map every node to a set of the nodes neighbors
    // The set contains (node, color)
    let mut adj_list: HashMap<i32, HashSet<(i32, i32)>> = HashMap::new();

    // Loop through every node, get its neighbors and add them to adj_list
    for i in 0..n as usize {
        if let Some(red_nbs) = red_map.get(&(i as i32)) {
            for nbr in red_nbs {
                adj_list.entry(i as i32).and_modify(|set| {(*set).insert((*nbr, 0));}).or_insert(HashSet::from([(*nbr, 0)]));
            }
        }

        if let Some(blue_nbs) = blue_map.get(&(i as i32)) {
            for nbr in blue_nbs {
                adj_list.entry(i as i32).and_modify(|set| {(*set).insert((*nbr, 1));}).or_insert(HashSet::from([(*nbr, 1)]));
            }
        }
    }

    // Keep track of all visited nodes and their colors
    let mut visit: Vec<Vec<bool>> = vec![vec![false, false]; n as usize];

    // Create a queue consisting of (node, steps, color)
    let mut queue: Vec<(i32, i32, i32)> = vec![(0,0,-1)];

    // Set 0's red and blue visited to true
    visit[0][0] = true;
    visit[0][1] = true;

    // Keep going as long as the queue isn't empty
    while queue.len() != 0 {
        let node = queue.remove(0);

        // If the evaluated node has neighbors, loop through the neighbors
        if let Some(nbrs) = adj_list.get(&node.0) {
            for nbr in nbrs {
                // If the neighbor hasn't been visited and 
                // the evaluated color isn't the same as the neighbors color
                if visit[nbr.0 as usize][nbr.1 as usize] == false && nbr.1 != node.2 {
                    // Add neighbor to the queue with a step incrementation
                    queue.push((nbr.0, node.1 + 1, nbr.1));
                    // Log the neighbor as visited
                    visit[nbr.0 as usize][nbr.1 as usize] = true;

                    // Update the result of the neighbor
                    if result[nbr.0 as usize] == -1 {
                        result[nbr.0 as usize] = node.1 + 1;
                    }
                }
            }
        }
    }

    result[0] = 0;

    result
}