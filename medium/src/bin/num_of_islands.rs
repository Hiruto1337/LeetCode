fn main(){
    let grid = vec![
        vec!['1','0','1','0','1'],
        vec!['0','1','0','1','0'],
        vec!['1','0','1','0','1'],
        vec!['0','1','0','1','0']
      ];

      println!("{}", num_islands(grid));
}

fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    use std::collections::HashSet;

    fn expand_island(x: usize, y:usize, grid: &Vec<Vec<char>>, hashset: &mut HashSet<(usize, usize)>) {
        if x < grid[0].len() && y < grid.len() && !(*hashset).contains(&(x,y)) && grid[y][x] == '1' {
            (*hashset).insert((x,y));
            if x != 0 {expand_island(x - 1, y, grid, hashset);}
            expand_island(x + 1, y, grid, hashset);
            if y != 0 {expand_island(x, y - 1, grid, hashset);}
            expand_island(x, y + 1, grid, hashset);
        }
    }

    let mut set: HashSet<(usize, usize)> = HashSet::new();
    let mut total = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '1' {
                if let None = set.get(&(x,y)) {
                    total += 1;
                    expand_island(x, y, &grid, &mut set);
                }
            }
        }
    }

    total
}

// Create a hashmap that maps (x,y) coordinates to a boolean value
// (indicating if they've already been traversed)
// Loop through each element in the matrix
// When finding 1 at a coordinate not already mapped, check up, down, left and right for more 1's
// and add them to the map
