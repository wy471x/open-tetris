# Rust Tetris TUI — Detailed Implementation Plan

> Team collaboration: Jasper (Architecture) / Bram (Engineering) / Atlas (Research) / Iris (Visual Design)

---

## 1. Tech Stack & Crate Selection

```toml
[dependencies]
ratatui = "0.29"       # TUI rendering framework
crossterm = "0.28"     # Terminal control (ratatui default backend)
rand = "0.8"           # Random piece generation (7-bag algorithm)
```

Why **ratatui + crossterm**:
- ratatui provides high-level abstractions like Layout / Block / Paragraph / Frame, eliminating extensive manual terminal drawing
- crossterm provides event-driven input handling with native key event support and cross-platform compatibility
- Tetris-level UI density is well within ratatui's capabilities — nothing heavier needed

---

## 2. Architecture: Game Loop + State Machine (no ECS)

Tetris has only 7 pieces + a 10×20 grid — ECS would be overkill. A classic Game Loop + State Machine suffices:

```
┌─────────────────────────────────┐
│         Game Loop (60fps)        │
│                                  │
│  Process Input → Update State   │
│       ↓              ↓          │
│  Check Collisions  Render       │
└─────────────────────────────────┘
```

---

## 3. Project File Structure

```
tetris/
├── Cargo.toml
└── src/
    ├── main.rs          — Entry point, terminal init + event loop
    ├── app.rs           — App root state, holds Game + current Screen
    ├── game.rs          — Core game logic (pure data, zero TUI deps)
    ├── board.rs         — 10×20 grid, collision detection, line clear
    ├── piece.rs         — 7 tetrominoes + SRS rotation tables
    ├── bag.rs           — 7-bag random generator
    ├── ui.rs            — All ratatui rendering logic
    ├── input.rs         — crossterm events → game action mapping
    └── constants.rs     — Grid size, colors, tick speed, etc.
```

**Core layering**: `game / board / piece / bag` is the pure logic layer with zero TUI dependencies; `ui / input / main` is the terminal adapter layer. Switching from TUI to GUI later requires replacing only `ui` and `input`.

---

## 4. Game State Machine

```rust
enum Screen {
    Menu,
    Playing(Game),      // Game holds Board + current Piece + score
    Paused(Game),       // Preserves Game state while paused
    GameOver { score: u32 },
}
```

State transitions:

```
Menu ──[Enter]──→ Playing
Playing ──[Esc]──→ Paused
Paused ──[Esc]──→ Playing
Paused ──[Q]───→ Menu
Playing ──[piece tops out]──→ GameOver
GameOver ──[Enter]──→ Menu
```

---

## 5. TUI Layout (60fps rendering)

```
┌────────────┬──────────┐
│            │  NEXT    │
│            │ ┌──┐     │
│  10×20     │ │  │     │
│  playfield │ └──┘     │
│            │          │
│            │ SCORE    │
│            │ 1200     │
│            │          │
│            │ LEVEL    │
│            │   5      │
└────────────┴──────────┘
│      CONTROLS          │
│ ← → move  ↑ rotate    │
│ ↓ soft drop  Space HD  │
│  P pause   Q quit      │
└────────────────────────┘
```

ratatui Layout split: 60% left for playfield, 40% right for info panel, bottom row for controls hint.

---

## 6. Rendering Approach

Each cell is rendered as 2 space characters, making terminal cells appear approximately square (terminal char height ≈ 2× width):

```rust
fn render_board(frame: &mut Frame, board: &Board, area: Rect) {
    for row in 0..BOARD_ROWS {
        for col in 0..BOARD_COLS {
            let cell = board.grid[row][col];
            let style = Style::default().bg(cell.color);
            let span = Span::styled("  ", style);
            // render at corresponding position
        }
    }
}
```

### Tetromino Color Scheme (by Iris, battle-tested over 40 years)

| Piece | Char | Hex | Rationale |
|------|------|------|------|
| I | `I` | `#00F0F0` Cyan | Long bar, brightest color, stands out while falling |
| O | `O` | `#F0F000` Yellow | Square block, warmest color, feels stable |
| T | `T` | `#A000F0` Purple | T-shape, royal character, most memorable |
| S | `S` | `#00F000` Green | Z's mirror, "S = green" mnemonic |
| Z | `Z` | `#F00000` Red | "Z rhymes with red", classic English mnemonic |
| J | `J` | `#0000F0` Blue | "J = blue" visual pause sensation |
| L | `L` | `#F0A000` Orange | Only remaining warm color, high distinctiveness |

---

## 7. Input Handling (crossterm event loop)

```rust
// Map key events to game Actions
enum Action {
    MoveLeft,
    MoveRight,
    RotateCW,       // clockwise
    RotateCCW,      // counter-clockwise
    SoftDrop,       // accelerated fall
    HardDrop,       // instant drop to bottom
    Hold,           // hold piece (optional)
    Pause,
    Quit,
}
```

Non-blocking input loop:

```rust
loop {
    if event::poll(Duration::from_millis(16))? {  // ~60fps
        if let Event::Key(key) = event::read()? {
            // map key → Action → game.handle(action)
        }
    }
    game.tick();   // gravity + lock logic
    ui::render();  // render a frame
}
```

**Key mapping:**

| Key | Action |
|------|------|
| ← → | Move left / right |
| ↑ | Rotate clockwise |
| Z | Rotate counter-clockwise |
| ↓ | Soft drop (accelerate) |
| Space | Hard drop (instant bottom) |
| C | Hold (store piece) |
| P | Pause / Resume |
| Q | Quit |

---

## 8. Game Loop Timing

```rust
const TICK_BASE: Duration = Duration::from_millis(800);   // Lv1
const TICK_MIN:  Duration = Duration::from_millis(50);     // Lv20+
const LOCK_DELAY: Duration = Duration::from_millis(500);   // ground lock delay

fn tick(&mut self) {
    let now = Instant::now();
    // 1. Gravity: tick interval reached → piece moves down one row
    // 2. Lock: grounded + LOCK_DELAY exceeded → write piece to board
    // 3. Line clear: check full rows → remove → add score
    // 4. Next piece: draw from bag
}
```

**Lock delay is the source of authentic game feel** — after touching down, a 500ms window allows the player to still slide and rotate. Without this mechanism, gameplay quality drops dramatically.

---

## 9. SRS Rotation System (Super Rotation System)

### Piece Definition (piece.rs)

7 pieces, 4 rotation states each, direct lookup table:

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
enum Tetromino { I, O, T, S, Z, J, L }

impl Tetromino {
    fn cells(&self, rotation: usize) -> [(i32, i32); 4] {
        match self {
            Tetromino::I => [
                [(0,1), (1,1), (2,1), (3,1)],  // state 0: horizontal
                [(2,0), (2,1), (2,2), (2,3)],  // state 1
                [(0,2), (1,2), (2,2), (3,2)],  // state 2
                [(1,0), (1,1), (1,2), (1,3)],  // state 3
            ][rotation],
            // ... 6 more pieces
        }
    }
}
```

### Wall Kick Offset Tables

When rotation causes collision, try 5 offset groups per the SRS standard. This is the core of rotation feel:

```
Rotation fails → try offset 1 → try offset 2 → ... → try offset 5 → give up
```

Reference the standard Wall Kick tables at [tetris.wiki/Super_Rotation_System](https://tetris.wiki/Super_Rotation_System).

---

## 10. 7-Bag Random Generator (bag.rs)

```rust
struct Bag {
    queue: Vec<Tetromino>,
}

impl Bag {
    fn next(&mut self) -> Tetromino {
        if self.queue.is_empty() {
            self.refill();
        }
        self.queue.pop().unwrap()
    }

    fn refill(&mut self) {
        let mut pieces = vec![I,O,T,S,Z,J,L];
        // Fisher-Yates shuffle
        // ...
        self.queue = pieces;
    }
}
```

The 7-Bag guarantees each of the 7 piece types appears exactly once every 7 pieces, eliminating the frustration of extreme randomness.

---

## 11. Collision Detection (board.rs)

```rust
impl Board {
    fn collides(&self, piece: &Piece, offset_x: i32, offset_y: i32) -> bool {
        for (x, y) in piece.cells() {
            let nx = piece.x + x + offset_x;
            let ny = piece.y + y + offset_y;
            if nx < 0 || nx >= COLS || ny >= ROWS { return true; }
            if ny < 0 { continue; } // allow above the top
            if self.grid[ny as usize][nx as usize].occupied { return true; }
        }
        false
    }
}
```

---

## 12. Phased Implementation Plan

| Step | Content | Acceptance Criteria |
|------|------|---------|
| **1** | `cargo init` + ratatui/crossterm deps + alternate screen startup | Terminal clears, Q to quit |
| **2** | `board.rs` + `piece.rs` — grid data structure + 7 piece definitions | `cargo test` all pass |
| **3** | `ui.rs` — render empty grid + sidebar layout | See 10×20 empty grid + NEXT/SCORE |
| **4** | `game.rs` — Game Loop, piece falling + input control | Can move, rotate, auto-drop |
| **5** | Collision detection + locking + line clear + scoring | Playable full game |
| **6** | State machine: Menu → Playing → Pause → GameOver + title screen | Complete game loop |

---

## 13. Key Architecture Decisions

- **Strict separation of pure logic and rendering layers** — board / piece / game do not import ratatui
- **60fps event loop + variable gravity tick** — event loop fixed at 16ms, gravity independently timed
- **SRS lookup table implementation** — no self-derived rotation math, directly reference tetris.wiki offset tables
- **Each cell rendered as 2 spaces** — the only reliable approach for square terminal blocks
- **Lock delay 500ms** — don't lock instantly on touchdown, allow final adjustments

---

## 14. Title Screen (ASCII Art)

```
╔══════════════════════════════════════╗
║                                      ║
║      ██╗ █████╗ ██╗   ██╗            ║
║      ██║██╔══██╗╚██╗ ██╔╝            ║
║      ██║███████║ ╚████╔╝             ║
║ ██   ██║██╔══██║  ╚██╔╝              ║
║ ╚█████╔╝██║  ██║   ██║               ║
║  ╚════╝ ╚═╝  ╚═╝   ╚═╝               ║
║                                      ║
║      Rust TUI Tetris                 ║
║                                      ║
║      Press ENTER to start            ║
║      Press Q to quit                 ║
║                                      ║
╚══════════════════════════════════════╝
```

---

> Last updated: 2026-06-27
> Team: Jasper (Architecture) / Bram (Engineering) / Atlas (Research) / Iris (Visual Design) / Nova (Product Management)
