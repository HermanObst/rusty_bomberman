# Bomberman R Game README

Welcome to the Bomberman R project, a Rust-based implementation inspired by the classic Bomberman game, where strategy and quick decision-making lead to victory. This version is designed as part of the FIUBA Rust Workshop, focusing on the development of a robust and efficient solution to manage game state transformations based on bomb detonations in a maze-like environment.

## Features

- Dynamic Maze Environment: Simulate bomb explosions within a grid, observing how different elements (enemies, walls, bombs) interact.
- Complex Game Mechanics: Incorporate various objects within the game, including enemies with different health points, different types of bombs, and unique maze elements like rocks and detours.
- Input/Output Processing: Read game state from a file, execute the simulation based on specified bomb detonations, and output the final state to a file.
- Error Handling: Robust error handling to manage unexpected input scenarios without crashing or panicking.

## Getting Started

### Prerequisites

- Rust 1.72
- Basic familiarity with command-line operations in Unix/Linux environments.

### Installation and Running

1. Clone the project repository.
2. Navigate to the project directory and compile using `cargo build`.
3. Run the program with `cargo run -- <input_file> <output_dir> <x_coord> <y_coord>` to simulate a bomb detonation.

## Input Format

The input is a file representing the game's grid and an initial bomb detonation point. Each object in the grid is encoded with specific symbols for ease of parsing.

## Output Format

The output is a file showing the game grid's final state after simulating the bomb detonation, adhering to the same encoding as the input.

## Acknowledgements

This project is based on the guidelines provided by FIUBA's Rust Workshop, particularly the individual project assignment. A special thank you to FIUBA for their comprehensive resources and support in Rust language learning and application.

## Contributing

Contributions are encouraged, especially in areas such as feature enhancement, bug fixes, and performance optimization. Please follow the standard GitHub flow for submitting contributions.

## License

This project is licensed under the MIT License.

## Contact

For more information and support, please open an issue on the project's GitHub repository.
