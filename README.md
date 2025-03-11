# Dragon Maze

Dragon Maze is a text-based maze game where the player attempts to navigate the maze while avoiding a dragon. The original Integer BASIC source code was provided with the Apple II computer, printed in "The Redbook," and is included here in the `/reference/` directory. This project is an implementation in Rust. Some of the visual features are as faithfully reproduced as possible, such as instruction text and visual maze representation. However, some features have been reimagined to be more readable or user-friendly, such as maze generation and input commands.

## Features

- **Maze Generation**: The maze is generated with all walls initially present. Paths are created by removing walls between cells.
- **Player Movement**: The player can move up, down, left, or right using the `w`, `a`, `s`, and `d` keys, respectively.
- **Dragon Movement**: The dragon moves randomly within the maze.
- **Win/Lose Conditions**: The player wins by reaching the exit and loses if caught by the dragon.
- **Rendering**: The maze, player, and dragon are rendered in the terminal using Unicode characters.

## Missing Features

- **Intelligent Dragon Movement**: Currently, the dragon's movements are random. The feature to make the dragon move intelligently towards the player is not yet implemented.
- **Dragon Climbing Over Walls**: The feature where the dragon can get "angry" and climb over walls after visiting a cell multiple times is not yet implemented.
- **Score**: The original game tracked how many moves it took to reach the finish and tallied a score at the end. This feature is not yet implemented.

## How to Play

1. Run the game, read the instructions, and type "go[ENTER]" to begin.
2. Watch the game generate the maze, then clear out all of the walls.
3. Use the `w`, `a`, `s`, and `d` keys to move the player up, left, down, and right, respectively.
4. Avoid the dragon and reach the exit to win the game.
5. If the dragon catches you, you lose the game.

## Controls

- `w`: Move up
- `a`: Move left
- `s`: Move down
- `d`: Move right
- `Esc`: Exit the game

## Installation

1. Ensure you have Rust installed on your system.
2. Clone the repository.
3. Navigate to the project directory.
4. Run the game using `cargo run`.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests for new features, bug fixes, or improvements.

## License

This project is licensed under the MIT License. See LICENSE file in the root of the project for details.

## Attribution

The original Integer BASIC source code was provided with the Apple II computer and printed in "The Redbook." The original author is credited in the game output.

## Disclaimer

The original BASIC code is included for educational and reference purposes only. If you have any concerns about the inclusion of this code, please contact us.
