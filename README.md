# Rata-Tetris

![demo animation](images/demo_image.gif)

This project implements a Tetris-like game written in Rust. Below is a brief overview of the structure, features, and functionality based on the provided code.

## Features

- **Classic Tetris Gameplay**: Includes features such as line clearing, scoring, and leveling.
- **Game Board and Block Types**: A grid-based board where players stack and clear blocks, with various tetrominoes that have unique shapes and rotations. Blocks change color according to level progression.
- **Level Progression**: Increases difficulty as the player clears more lines.
- **Scoring System**: Tracks scores based on the number of lines cleared and the level.
- **Statistics Tracking**: Records player performance during the game.
- **Pause and Resume**: Allows the player to pause and resume the game.
- **Slam Feature**: Players can instantly drop pieces to the bottom of the board.
- **Ghost Piece**: Displays a shadow of where the current piece will land.

### Dependencies

- **External Crates**:
  - `crossterm = "0.28.1"`: Terminal manipulation.
  - `dirs = "5.0.1"`: For handling directory paths, likely for saving game data.
  - `once_cell = "1.20.2"`: Single assignment cell for global data.
  - `rand = "0.8.5"`: Random number generation for piece placement.
  - `ratatui = "0.29.0"`: UI rendering for terminal applications.

## Getting Started

### Prerequisites

- Install [Rust](https://www.rust-lang.org/).
- Add the `cargo` package manager.

### Building the Project

1. Clone the repository:
   ```sh
   git clone <repository-url>
   ```
2. Navigate to the project directory:
   ```sh
   cd game_project
   ```
3. Build the project:
   ```sh
   cargo build
   ```

### Running the Game

```sh
cargo run
```

## Contributing

1. Fork the repository.
2. Create a feature branch: `git checkout -b feature-name`.
3. Commit your changes: `git commit -m 'Add feature'`.
4. Push to the branch: `git push origin feature-name`.
5. Open a pull request.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

## Acknowledgments

- This project was written as an educational exercise. No code from other versions of Tetris or similar games was referenced.
- [Rust Book](https://doc.rust-lang.org/book/) for excellent documentation.
- Contributors and open-source libraries used in this project.

