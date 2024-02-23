use maze_types::Maze;

mod maze_types;
mod errors;
fn main() {
    Maze::new("src/input.txt");
}
