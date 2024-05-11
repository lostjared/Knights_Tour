# Knights Tour Simulation

![board](https://github.com/lostjared/Knights_Tour/blob/main/screens/tour.jpg)


This project implements a Knights Tour simulation using Rust and SDL2. The goal of the simulation is to move a knight around a chessboard, touching each square exactly once.

## Features

- Displays an 8x8 chessboard
- Moves the knight to each square on the board exactly once
- Shows the current state of the tour and indicates when the tour is complete
- Allows users to restart the tour by pressing `Enter`

## Requirements

- Rust
- SDL2
- SDL2_ttf

## Installation

1. Ensure you have Rust installed. If not, you can install it from [rust-lang.org](https://www.rust-lang.org/).
2. Install SDL2 and SDL2_ttf on your system. For installation instructions, refer to [SDL2 Installation Guide](https://github.com/Rust-SDL2/rust-sdl2#user-content-requirements).
3. Clone this repository:

```sh
git clone https://github.com/lostjared/Knights_Tour.git
cd Knights_Tour/knights_tour
```

4. Build the project using Cargo:

```sh
cargo build --release
```

5. Run the executable:

```sh
cargo run --release
```

## Usage

- Press `Space` to move the knight to the next position.
- Press `Enter` to restart the tour.
- Press `Escape` to exit the simulation.

## Code Structure

- **Position**: Represents the knight's position on the board.
- **drawboard**: Draws the chessboard on the SDL2 canvas.
- **drawknight**: Draws the knight at its current position on the board.
- **clearboard**: Clears the board for a new tour.
- **nextmove**: Calculates the knight's next move using Warnsdorff's rule.

## File Structure

- `src/main.rs`: The main file containing the simulation logic.
- `data/`: Directory containing required assets such as fonts and images.

## Assets

- `data/font.ttf`: The font used for rendering text.
- `data/knight.bmp`: The knight image used for the simulation.

## Contributing

If you would like to contribute to this project, please fork the repository and submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Acknowledgments

This project uses the following libraries and resources:

- [SDL2](https://www.libsdl.org/)
- [Rust-SDL2](https://github.com/Rust-SDL2/rust-sdl2)
