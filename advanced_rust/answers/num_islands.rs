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
fn solution(country: &str, island_map: Vec<Vec<i32>>, rows: usize, cols: usize) -> i32 {
    let mut queue: Vec<Vec<usize>> = Vec::new();
    let mut count: i32 = 0;
    let mut x: usize;
    let mut y: usize;
    let mut island_map: Vec<Vec<i32>> = island_map;

    for i in 0 .. island_map.len() {
        for j in 0 .. island_map[i].len() {
            if island_map[i][j] == 0 {
                continue;
            }
            island_map[i][j] = 0;
            queue.push(vec![i, j]);
            while queue.len() > 0 {
                // coordinates
                x = queue[0][0];
                y = queue[0][1];

                // mark the island as counted
                island_map[x][y] = 0;
                queue.remove(0);

                // x - 1, y - 1
                if (x as i32 - 1 >= 0) && (y as i32 - 1 >= 0) && island_map[x - 1][y - 1] == 1 {
                    queue.push(vec![x - 1, y - 1]);
                }

                // x - 1, y
                if (x as i32 - 1 >= 0) && island_map[x - 1][y] == 1 {
                    queue.push(vec![x - 1, y]);
                }

                // x - 1, y + 1
                if (x as i32 - 1 >= 0) && (y + 1 < cols) && island_map[x - 1][y + 1] == 1 {
                    queue.push(vec![x - 1, y + 1]);
                }

                // x, y - 1
                if (y as i32 - 1 >= 0) && island_map[x][y - 1] == 1 {
                    queue.push(vec![x, y - 1]);
                }

                // x, y + 1,
                if (y + 1 < cols) && island_map[x][y + 1] == 1 {
                    queue.push(vec![x, y + 1]);
                }

                // x + 1, y - 1,
                if (x + 1 < rows) && (y as i32 - 1 >= 0) && island_map[x + 1][y - 1] == 1 {
                    queue.push(vec![x + 1, y - 1]);
                }

                // x + 1, y
                if (x + 1 < rows) && island_map[x + 1][y] == 1 {
                    queue.push(vec![x + 1, y]);
                }

                // x + 1, y + 1
                if (x + 1 < rows) && (y + 1 < cols) && island_map[x + 1][y + 1] == 1 {
                    queue.push(vec![x + 1, y + 1]);
                }
            }

            count += 1;
        }
    }

    println!("Total number of islands at {country}: {count}");
    return count;
}

fn main() {
    let vec1: Vec<Vec<i32>> = vec![
        vec![1, 0, 0],
        vec![1, 0, 0],
        vec![0, 0, 1]
    ];

    let vec2: Vec<Vec<i32>> = vec![
        vec![1, 0, 0],
        vec![1, 1, 0],
        vec![0, 0, 1]
    ];

    let vec3: Vec<Vec<i32>> = vec![
        vec![1, 0, 0, 0, 0],
        vec![0, 0, 1, 1, 0],
        vec![0, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![1, 1, 0, 0, 1],
        vec![1, 1, 0, 0, 1]
    ];

    assert_eq!(solution("vec1", vec1, 3, 3), 2);
    assert_eq!(solution("vec2", vec2, 3, 3), 1);
    assert_eq!(solution("vec3", vec3, 6, 5), 4);
}