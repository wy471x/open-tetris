# Rust Tetris TUI — 詳細実装計画

> チーム協業：Jasper (アーキテクチャ) / Bram (エンジニアリング) / Atlas (リサーチ) / Iris (ビジュアルデザイン)

---

## 1. 技術スタックとクレート選定

```toml
[dependencies]
ratatui = "0.29"       # TUI レンダリングフレームワーク
crossterm = "0.28"     # 端末制御（ratatui デフォルトバックエンド）
rand = "0.8"           # ランダムブロック生成（7-bag アルゴリズム）
```

**ratatui + crossterm** を選択した理由：
- ratatui は Layout / Block / Paragraph / Frame などの高レベル抽象を提供し、手動の端末描画コードを大幅に削減
- crossterm はイベント駆動の入力処理、ネイティブのキーイベント対応、優れたクロスプラットフォーム互換性
- Tetris の UI 密度には ratatui で十分 — より重いものは不要

---

## 2. アーキテクチャ：Game Loop + State Machine（ECS 不使用）

Tetris は 7 種のブロック + 10×20 グリッドのみ — ECS は過剰。古典的な Game Loop + State Machine で十分：

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

## 3. プロジェクトファイル構造

```
tetris/
├── Cargo.toml
└── src/
    ├── main.rs          — エントリポイント、端末初期化 + イベントループ
    ├── app.rs           — App ルート状態、Game + 現在の Screen を保持
    ├── game.rs          — コアゲームロジック（純粋データ、TUI 非依存）
    ├── board.rs         — 10×20 グリッド、衝突判定、ライン消去
    ├── piece.rs         — 7 種テトロミノ + SRS 回転テーブル
    ├── bag.rs           — 7-bag ランダムジェネレーター
    ├── ui.rs            — すべての ratatui レンダリングロジック
    ├── input.rs         — crossterm イベント → ゲームアクション マッピング
    └── constants.rs     — グリッドサイズ、色、tick 速度などの定数
```

**コアレイヤリング**：`game / board / piece / bag` は純粋ロジック層で TUI 依存ゼロ。`ui / input / main` は端末アダプター層。後日 TUI から GUI への切り替えは `ui` と `input` の 2 ファイルのみ交換すればよい。

---

## 4. ゲーム状態遷移

```rust
enum Screen {
    Menu,
    Playing(Game),      // Game が Board + 現在の Piece + スコアを保持
    Paused(Game),       // 一時停止時に Game 状態を保持
    GameOver { score: u32 },
}
```

状態遷移：

```
Menu ──[Enter]──→ Playing
Playing ──[Esc]──→ Paused
Paused ──[Esc]──→ Playing
Paused ──[Q]───→ Menu
Playing ──[ブロックが天井到達]──→ GameOver
GameOver ──[Enter]──→ Menu
```

---

## 5. TUI レイアウト（60fps レンダリング）

```
┌────────────┬──────────┐
│            │  NEXT    │
│            │ ┌──┐     │
│  10×20     │ │  │     │
│  プレイ    │ └──┘     │
│  フィールド│          │
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

ratatui Layout 分割：左 60% プレイフィールド、右 40% 情報パネル、下部 1 行操作ヒント。

---

## 6. レンダリング方式

各セルを 2 つの半角スペースでレンダリングし、端末セルを正方形に近づける（端末文字の高さ ≈ 幅の 2 倍）：

```rust
fn render_board(frame: &mut Frame, board: &Board, area: Rect) {
    for row in 0..BOARD_ROWS {
        for col in 0..BOARD_COLS {
            let cell = board.grid[row][col];
            let style = Style::default().bg(cell.color);
            let span = Span::styled("  ", style);
            // 対応する位置にレンダリング
        }
    }
}
```

### テトロミノ配色（Iris 提供、40 年の実績ある標準配色）

| ブロック | 文字 | 色値 | 理由 |
|------|------|------|------|
| I | `I` | `#00F0F0` シアン | 長尺、最も明るい色、落下時に目立つ |
| O | `O` | `#F0F000` イエロー | 正方形、最も温かい色、安定感 |
| T | `T` | `#A000F0` パープル | T 字型、高貴な印象、最も記憶に残る |
| S | `S` | `#00F000` グリーン | Z の鏡像、"S = 緑" の記憶法 |
| Z | `Z` | `#F00000` レッド | "Z は red と韻を踏む"、英語圏の古典的記憶法 |
| J | `J` | `#0000F0` ブルー | "J = blue" の視覚的静止感 |
| L | `L` | `#F0A000` オレンジ | 残された唯一の暖色、高い識別性 |

---

## 7. 入力処理（crossterm イベントループ）

```rust
// キーイベントをゲーム Action にマッピング
enum Action {
    MoveLeft,
    MoveRight,
    RotateCW,       // 時計回り回転
    RotateCCW,      // 反時計回り回転
    SoftDrop,       // 加速落下
    HardDrop,       // 直接底まで落下
    Hold,           // ブロック保持（オプション）
    Pause,
    Quit,
}
```

ノンブロッキング入力ループ：

```rust
loop {
    if event::poll(Duration::from_millis(16))? {  // ~60fps
        if let Event::Key(key) = event::read()? {
            // キー → Action → game.handle(action) にマッピング
        }
    }
    game.tick();   // 重力 + ロックロジック
    ui::render();  // 1 フレームレンダリング
}
```

**キーマッピング：**

| キー | アクション |
|------|------|
| ← → | 左右移動 |
| ↑ | 時計回り回転 |
| Z | 反時計回り回転 |
| ↓ | ソフトドロップ（加速） |
| Space | ハードドロップ（直接底まで） |
| C | Hold（ブロック保持） |
| P | 一時停止 / 再開 |
| Q | 終了 |

---

## 8. Game Loop タイミング

```rust
const TICK_BASE: Duration = Duration::from_millis(800);   // Lv1
const TICK_MIN:  Duration = Duration::from_millis(50);     // Lv20+
const LOCK_DELAY: Duration = Duration::from_millis(500);   // 接地ロック遅延

fn tick(&mut self) {
    let now = Instant::now();
    // 1. 重力：tick 間隔に達する → ブロックを 1 行下に移動
    // 2. ロック：接地 + LOCK_DELAY 超過 → ブロックを board に書き込み
    // 3. ライン消去：完全な行を確認 → 削除 → スコア加算
    // 4. 次のブロック：bag から取得
}
```

**ロック遅延こそが本物の操作感の源** — 接地後 500ms の猶予があり、その間プレイヤーは左右移動と回転が可能。この仕組みがないとゲーム体験が大幅に低下する。

---

## 9. SRS 回転システム（Super Rotation System）

### ブロック定義（piece.rs）

7 種のブロック、各 4 回転状態、直接テーブル参照：

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
            // ... 他の 6 種
        }
    }
}
```

### ウォールキックオフセットテーブル

回転時に衝突が発生した場合、SRS 標準に従い 5 組のオフセットを試行する。これが回転操作感の核心：

```
回転失敗 → オフセット1試行 → オフセット2試行 → ... → オフセット5試行 → 回転放棄
```

[tetris.wiki/Super_Rotation_System](https://tetris.wiki/Super_Rotation_System) の標準ウォールキックテーブルを参照。

---

## 10. 7-Bag ランダムジェネレーター（bag.rs）

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
        // Fisher-Yates シャッフル
        // ...
        self.queue = pieces;
    }
}
```

7-Bag により 7 ピースごとに全種類が 1 回ずつ出現することが保証され、極端なランダム性によるフラストレーションを解消する。

---

## 11. 衝突判定（board.rs）

```rust
impl Board {
    fn collides(&self, piece: &Piece, offset_x: i32, offset_y: i32) -> bool {
        for (x, y) in piece.cells() {
            let nx = piece.x + x + offset_x;
            let ny = piece.y + y + offset_y;
            if nx < 0 || nx >= COLS || ny >= ROWS { return true; }
            if ny < 0 { continue; } // 天井より上は許容
            if self.grid[ny as usize][nx as usize].occupied { return true; }
        }
        false
    }
}
```

---

## 12. 段階的実装計画

| ステップ | 内容 | 検証基準 |
|------|------|---------|
| **1** | `cargo init` + ratatui/crossterm 依存 + 代替画面起動 | 端末クリア、Q で終了 |
| **2** | `board.rs` + `piece.rs` — グリッドデータ構造 + 7 種ブロック定義 | `cargo test` 全通過 |
| **3** | `ui.rs` — 空グリッド + サイドバーレイアウト描画 | 10×20 空グリッド + NEXT/SCORE 表示 |
| **4** | `game.rs` — Game Loop、ブロック落下 + 入力制御 | 移動、回転、自動落下可能 |
| **5** | 衝突判定 + ロック + ライン消去 + スコア | 完全なゲームプレイ可能 |
| **6** | 状態遷移：Menu → Playing → Pause → GameOver + タイトル画面 | 完全なゲームループ |

---

## 13. 主要アーキテクチャ決定事項

- **純粋ロジック層とレンダリング層の厳格な分離** — board / piece / game は ratatui を import しない
- **60fps イベントループ + 可変重力 tick** — イベントループは 16ms 固定、重力は個別タイマー
- **SRS テーブル参照実装** — 回転計算を自作せず、tetris.wiki のオフセットテーブルを直接参照
- **各セルを 2 スペースでレンダリング** — 端末で正方形ブロックを実現する唯一の信頼できる方法
- **ロック遅延 500ms** — 接地しても即ロックせず、最終調整を許容

---

## 14. タイトル画面（ASCII Art）

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

> 最終更新：2026-06-27
> チーム：Jasper (アーキテクチャ) / Bram (エンジニアリング) / Atlas (リサーチ) / Iris (ビジュアルデザイン) / Nova (プロダクト管理)
