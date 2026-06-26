# Rust Tetris TUI — 详细实现方案

> 团队协作出品：Jasper (架构) / Bram (工程) / Atlas (研究) / Iris (视觉)

---

## 1. 技术栈与 Crate 选型

```toml
[dependencies]
ratatui = "0.29"       # TUI 渲染框架
crossterm = "0.28"     # 终端控制（ratatui 默认后端）
rand = "0.8"           # 随机方块生成（7-bag 算法）
```

选择 **ratatui + crossterm** 的理由：
- ratatui 提供 Layout / Block / Paragraph / Frame 等高层抽象，省掉大量手动终端绘制代码
- crossterm 做事件驱动的输入处理，天然支持 key event，跨平台兼容性好
- Tetris 的 UI 密度用 ratatui 完全够，不需要更重的东西

---

## 2. 架构模式：Game Loop + State Machine（不用 ECS）

Tetris 就 7 种方块 + 10×20 网格，ECS 是杀鸡用牛刀。经典 Game Loop + State Machine 足够：

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

## 3. 项目文件结构

```
tetris/
├── Cargo.toml
└── src/
    ├── main.rs          — 入口，初始化 terminal + 启动 event loop
    ├── app.rs           — App 根状态，持有 Game + 当前 Screen
    ├── game.rs          — 游戏核心逻辑（纯数据，不碰 TUI）
    ├── board.rs         — 10×20 网格，碰撞检测，消行
    ├── piece.rs         — 7 种方块 + SRS 旋转表
    ├── bag.rs           — 7-bag 随机生成器
    ├── ui.rs            — 所有 ratatui 渲染逻辑
    ├── input.rs         — crossterm 事件 → 游戏动作映射
    └── constants.rs     — 网格尺寸、颜色、tick 速度等常量
```

**核心分层**：`game / board / piece / bag` 是纯逻辑层，零 TUI 依赖；`ui / input / main` 是终端适配层。后续从 TUI 切 GUI，只需换 `ui` 和 `input` 两个文件。

---

## 4. 游戏状态机

```rust
enum Screen {
    Menu,
    Playing(Game),      // Game 持有 Board + 当前 Piece + 分数
    Paused(Game),       // 暂停时保留 Game 状态
    GameOver { score: u32 },
}
```

状态转换：

```
Menu ──[Enter]──→ Playing
Playing ──[Esc]──→ Paused
Paused ──[Esc]──→ Playing
Paused ──[Q]───→ Menu
Playing ──[块触顶]──→ GameOver
GameOver ──[Enter]──→ Menu
```

---

## 5. TUI 布局（60fps 渲染）

```
┌────────────┬──────────┐
│            │  NEXT    │
│            │ ┌──┐     │
│  10×20     │ │  │     │
│  游戏区    │ └──┘     │
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

ratatui Layout 分割：左侧 60% 游戏区，右侧 40% 信息面板，底部一行操作提示。

---

## 6. 渲染方案

每个 cell 渲染为 2 个空格字符，使终端显示为近似正方形（终端字符高度 ≈ 2×宽度）：

```rust
fn render_board(frame: &mut Frame, board: &Board, area: Rect) {
    for row in 0..BOARD_ROWS {
        for col in 0..BOARD_COLS {
            let cell = board.grid[row][col];
            let style = Style::default().bg(cell.color);
            let span = Span::styled("  ", style);
            // 渲染到对应位置
        }
    }
}
```

### Tetromino 配色（Iris 提供，40 年验证的标准配色）

| 方块 | 字符 | 色值 | 理由 |
|------|------|------|------|
| I | `I` | `#00F0F0` 青 | 长条，最亮的颜色，下落时显眼 |
| O | `O` | `#F0F000` 黄 | 方块，最温暖的颜色，稳定感 |
| T | `T` | `#A000F0` 紫 | T 型，皇家气质，最容易被记住 |
| S | `S` | `#00F000` 绿 | Z 的镜像，S 绿色记忆顺口 |
| Z | `Z` | `#F00000` 红 | Z 跟 red 押韵，英文区经典记忆法 |
| J | `J` | `#0000F0` 蓝 | J = blue 的视觉停顿感 |
| L | `L` | `#F0A000` 橙 | L 型唯一剩下的暖色，辨识度高 |

---

## 7. 输入处理（crossterm event loop）

```rust
// 将 key event 映射为游戏 Action
enum Action {
    MoveLeft,
    MoveRight,
    RotateCW,       // 顺时针旋转
    RotateCCW,      // 逆时针
    SoftDrop,       // 加速下落
    HardDrop,       // 直接落底
    Hold,           // 暂存方块（可选）
    Pause,
    Quit,
}
```

非阻塞输入循环：

```rust
loop {
    if event::poll(Duration::from_millis(16))? {  // ~60fps
        if let Event::Key(key) = event::read()? {
            // 映射 key → Action → game.handle(action)
        }
    }
    game.tick();   // 重力 + 锁定逻辑
    ui::render();  // 渲染一帧
}
```

**键盘映射：**

| 按键 | 动作 |
|------|------|
| ← → | 左右移动 |
| ↑ | 顺时针旋转 |
| Z | 逆时针旋转 |
| ↓ | 软降（加速） |
| 空格 | 硬降（直接落底） |
| C | Hold（暂存） |
| P | 暂停/继续 |
| Q | 退出 |

---

## 8. Game Loop 时序

```rust
const TICK_BASE: Duration = Duration::from_millis(800);   // Lv1
const TICK_MIN:  Duration = Duration::from_millis(50);     // Lv20+
const LOCK_DELAY: Duration = Duration::from_millis(500);   // 触底锁定延迟

fn tick(&mut self) {
    let now = Instant::now();
    // 1. 重力：达到 tick 间隔 → 方块下移一格
    // 2. 锁定：触底 + 超过 LOCK_DELAY → piece 写入 board
    // 3. 消行：检查完整行 → 移除 → 加分
    // 4. 新块：从 bag 取下一个
}
```

**Lock delay 是真手感的来源** — 方块触底后给 500ms，这期间玩家仍可左右移动和旋转。没有这个机制游戏体验会大幅下降。

---

## 9. SRS 旋转系统（Super Rotation System）

### 方块定义（piece.rs）

7 种方块，每种 4 个旋转状态，直接查表：

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
enum Tetromino { I, O, T, S, Z, J, L }

impl Tetromino {
    fn cells(&self, rotation: usize) -> [(i32, i32); 4] {
        match self {
            Tetromino::I => [
                [(0,1), (1,1), (2,1), (3,1)],  // state 0: 水平
                [(2,0), (2,1), (2,2), (2,3)],  // state 1
                [(0,2), (1,2), (2,2), (3,2)],  // state 2
                [(1,0), (1,1), (1,2), (1,3)],  // state 3
            ][rotation],
            // ... 其他 6 种方块
        }
    }
}
```

### Wall Kick 偏移表

旋转发生碰撞时，按 SRS 标准尝试 5 组偏移。这是旋转手感的核心：

```
旋转失败 → 尝试偏移1 → 尝试偏移2 → ... → 尝试偏移5 → 放弃旋转
```

参考 [tetris.wiki/Super_Rotation_System](https://tetris.wiki/Super_Rotation_System) 的通用 Wall Kick 表。

---

## 10. 7-Bag 随机生成器（bag.rs）

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

7-Bag 确保每 7 个方块内每种出现一次，消除极端随机的挫败感。

---

## 11. 碰撞检测（board.rs）

```rust
impl Board {
    fn collides(&self, piece: &Piece, offset_x: i32, offset_y: i32) -> bool {
        for (x, y) in piece.cells() {
            let nx = piece.x + x + offset_x;
            let ny = piece.y + y + offset_y;
            if nx < 0 || nx >= COLS || ny >= ROWS { return true; }
            if ny < 0 { continue; } // 允许在顶部之上
            if self.grid[ny as usize][nx as usize].occupied { return true; }
        }
        false
    }
}
```

---

## 12. 分阶段实施计划

| 步骤 | 内容 | 验证标准 |
|------|------|---------|
| **1** | `cargo init` + ratatui/crossterm 依赖 + alternate screen 启动 | 终端清屏，按 Q 退出 |
| **2** | `board.rs` + `piece.rs` — 网格数据结构 + 7 种方块定义 | `cargo test` 全部通过 |
| **3** | `ui.rs` — 渲染空网格 + 侧边栏布局 | 看到 10×20 空网格 + NEXT/SCORE |
| **4** | `game.rs` — Game Loop，方块下落 + 输入控制 | 能移动、旋转、自动下落 |
| **5** | 碰撞检测 + 锁定 + 消行 + 分数 | 能玩完整一局 |
| **6** | 状态机：Menu → Playing → Pause → GameOver + 启动画面 | 完整游戏循环 |

---

## 13. 关键架构决策

- **纯逻辑层与渲染层严格分离** — board / piece / game 里不 import ratatui
- **60fps event loop + 可变重力 tick** — event loop 固定 16ms，重力单独计时
- **SRS 查表实现** — 不自己推导旋转，直接参考 tetris.wiki 的偏移表
- **每个 cell 渲染为 2 个空格** — 终端方块比例的唯一可靠方案
- **Lock delay 500ms** — 触底不立刻锁定，允许最终调整

---

## 14. 启动画面（ASCII Art）

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

> 最后更新：2026-06-27
> 团队：Jasper (架构) / Bram (工程代码) / Atlas (研究调研) / Iris (视觉设计) / Nova (产品管理)
