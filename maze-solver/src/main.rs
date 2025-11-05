use maze_solver::*;

fn main() {
    let maze = vec![
        vec!['S', '.', '#', '#', '#'],
        vec!['#', '.', '#', '.', '.'],
        vec!['#', '.', '.', '.', '#'],
        vec!['#', '#', '#', '.', '#'],
        vec!['#', '#', '#', 'E', '#'],
    ];
    let start = (0, 0);
    let end = (4, 3);

    let path = solve_maze(maze, start, end);
    if path.is_empty() {
        println!("is empty")
    }
    path.iter().for_each(|&p| println!("{}, {}", p.0, p.1));
}
