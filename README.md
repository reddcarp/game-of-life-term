<p align="center">
  <img src="screenshots/Icon.png" width="200" alt="Game of Life Logo">
</p>

<h1 align="center">Game of Life</h1>

<p align="center">
  <strong>Conway's game of life in Rust in the terminal.</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/language-rust-orange.svg" alt="Language">
  <img src="https://img.shields.io/badge/platform-term-blue.svg" alt="Platform">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="License">
</p>

## Screenshots

| Neighbours | Trail | Heatmap |
| :---: | :---: | :---: |
| <img src="screenshots/neighbour_count.png" width="250"> | <img src="screenshots/trail.png" width="250"> | <img src="screenshots/density.png" width="250"> |

| Default | Trail & Neighbours |
| :---: | :---: |
| <img src="screenshots/default.png" width="250"> | <img src="screenshots/trail_and_neighbour.png" width="250"> |

## Features

- [x] Mouse click to draw cells
- [x] Neighbour count visualization
- [x] Heatmap visualization
- [x] Trail visualization
- [x] Original view
- [x] Simulation control (Pause/Play/Restart/Clear)

## Usage

1. **Clone the repository**

    ```bash
    git clone https://github.com/reddcarp/game-of-life-term.git
    ```

2. **Build binary**

    ```bash
    cd game-of-life-term
    cargo build --release
    ```

3. **Run binary**

    ```bash
    ./target/release/game_of_life
    ```
