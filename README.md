# Open Tetris

A classic Tetris game built in Rust for the terminal, powered by [ratatui](https://github.com/ratatui/ratatui) and [crossterm](https://github.com/crossterm-rs/crossterm).

## Features

- **SRS rotation system** — Super Rotation System with full wall kick tables for all 7 tetrominoes
- **7-bag randomizer** — Fisher-Yates shuffle ensures fair piece distribution
- **Lock delay** — 500ms lock delay with up to 15 move/reset extensions
- **Ghost piece** — translucent preview of where the piece will land
- **Scoring** — Single/Double/Triple/Tetris (100/300/500/800 × level), soft drop (1pt), hard drop (2pt per row)
- **Level progression** — Speed increases exponentially from 800ms to 50ms per tick
- **State machine** — Menu → Playing → Paused → GameOver

## Quick Start

```bash
cargo run
```

## Controls

| Key | Action |
|-----|--------|
| ← → | Move left / right |
| ↑ | Rotate clockwise |
| Z | Rotate counter-clockwise |
| ↓ | Soft drop |
| Space | Hard drop |
| P | Pause / Resume |
| Q | Quit |
| Enter | Start / Confirm |

## Tech Stack

- **Rust** — systems language
- **ratatui** 0.29 — terminal UI rendering
- **crossterm** 0.28 — terminal input & control
- **rand** 0.8 — random number generation

## Project Structure

```
src/
├── main.rs       — terminal init + 60fps event loop
├── app.rs        — state machine (Menu / Playing / Paused / GameOver)
├── game.rs       — core game logic: gravity, lock delay, scoring
├── board.rs      — 10×20 grid, collision detection, line clear
├── piece.rs      — 7 tetrominoes, SRS rotation states, wall kick tables
├── bag.rs        — 7-bag randomizer
├── ui.rs         — ratatui rendering: board, side panel, menus, overlays
├── input.rs      — crossterm key → action mapping
└── constants.rs  — grid size, timing, scoring constants
```

## License

MIT
