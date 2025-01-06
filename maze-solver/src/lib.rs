pub fn solve_maze(
    maze: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    // Your code here

    let mut visited = vec![vec![false; maze[0].len()]; maze.len()];
    let mut path = Vec::new();
    let mut best_path = Vec::new();
    search_path(&maze, &mut visited, start, end, &mut path, &mut best_path);

    best_path
}

fn search_path(
    maze: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    position: (usize, usize),
    end: (usize, usize),
    path: &mut Vec<(usize, usize)>,
    best_path: &mut Vec<(usize, usize)>,
) {
    if position.0 >= maze.len()
        || position.1 >= maze[0].len()
        || maze[position.0][position.1] == '#'
        || visited[position.0][position.1]
    {
        return;
    }

    path.push(position);
    visited[position.0][position.1] = true;

    if position == end {
        if best_path.is_empty() || path.len() < best_path.len() {
            best_path.clear();
            best_path.extend_from_slice(path);
        }
    } else {
        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        for direction in directions {
            let new_position = (
                position.0 as i32 + direction.0,
                position.1 as i32 + direction.1,
            );

            if new_position.0 >= 0 && new_position.1 >= 0 {
                search_path(
                    maze,
                    visited,
                    (new_position.0 as usize, new_position.1 as usize),
                    end,
                    path,
                    best_path,
                );
            }
        }
    }

    path.pop();
    visited[position.0][position.1] = false;
}
