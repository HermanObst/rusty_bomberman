use maze_types::Maze;
mod maze_types;
mod errors;
fn main() {
    let maze = Maze::new("src/input.txt").unwrap();
    let _matrix = maze.matrix;
}
