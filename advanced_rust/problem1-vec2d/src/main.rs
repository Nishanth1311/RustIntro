/**
   Given a matrix of 1s and 0s, return the number of "islands" in the matrix. A 1 represents land and 0 represents water, so an island is a group of 1s that are neighboring whose perimeter is surrounded by water.

   For example, this matrix has 4 islands.

   1 0 0 0 0
   0 0 1 1 0
   0 1 1 0 0
   0 0 0 0 0
   1 1 0 0 1
   1 1 0 0 1

   NOTE: Solve it using 2D vectors.
   The goal of this problem is for you to understand vectors better
*/

fn dfs(mut island_map: &mut Vec<Vec<i32>>, i: usize, j: usize, rows: usize, cols: usize) {

    if i<0 || j<0 || i>=rows || j>=cols || island_map[i][j]!=1  {
        return;
    }

    island_map[i][j] = 2;
    dfs(island_map, i+1, j, rows, cols);
    if i>0 {
        dfs(island_map, i-1, j, rows, cols);
    }
    dfs(island_map, i, j+1, rows, cols);
    if j>0 {
        dfs(island_map, i, j-1, rows, cols);
    }
}

fn solution(country: &str, mut island_map: Vec<Vec<i32>>, rows: usize, cols: usize) -> i32 {
    // Your code here
    if island_map.len() == 0 {
        return 0;
    }

    let mut no_of_islands = 0;

    for i in 0..rows {
        for j in 0..cols {
            if island_map[i][j] == 1 {
                dfs(&mut island_map, i, j, rows, cols);
                no_of_islands += 1;
            }
        }
    }

    println!("{} has {} islands", country, no_of_islands);
    return no_of_islands;

}

fn main() {
    let vec1: Vec<Vec<i32>> = vec![vec![1, 0, 0], vec![1, 0, 0], vec![0, 0, 1]];

    let vec2: Vec<Vec<i32>> = vec![vec![1, 0, 0], 
                                   vec![1, 1, 0], 
                                   vec![0, 0, 1]];

    let vec3: Vec<Vec<i32>> = vec![
        vec![1, 0, 0, 0, 0],
        vec![0, 0, 1, 1, 0],
        vec![0, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![1, 1, 0, 0, 1],
        vec![1, 1, 0, 0, 1],
    ];

    assert_eq!(solution("vec1", vec1, 3, 3), 2);
    assert_eq!(solution("vec2", vec2, 3, 3), 2);
    assert_eq!(solution("vec3", vec3, 6, 5), 4);
}
