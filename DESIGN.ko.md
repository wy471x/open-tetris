# Rust Tetris TUI — 상세 구현 계획

> 팀 협업: Jasper (아키텍처) / Bram (엔지니어링) / Atlas (리서치) / Iris (비주얼 디자인)

---

## 1. 기술 스택 및 크레이트 선정

```toml
[dependencies]
ratatui = "0.29"       # TUI 렌더링 프레임워크
crossterm = "0.28"     # 터미널 제어 (ratatui 기본 백엔드)
rand = "0.8"           # 랜덤 블록 생성 (7-bag 알고리즘)
```

**ratatui + crossterm** 선택 이유：
- ratatui는 Layout / Block / Paragraph / Frame 등 고수준 추상화를 제공하여 수동 터미널 그리기 코드를 대폭 절감
- crossterm은 이벤트 기반 입력 처리, 네이티브 키 이벤트 지원, 뛰어난 크로스 플랫폼 호환성
- Tetris 수준의 UI 밀도에는 ratatui로 충분 — 더 무거운 것은 불필요

---

## 2. 아키텍처: Game Loop + State Machine (ECS 미사용)

Tetris는 블록 7종 + 10×20 그리드만 존재 — ECS는 과잉 설계. 고전적인 Game Loop + State Machine으로 충분：

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

## 3. 프로젝트 파일 구조

```
tetris/
├── Cargo.toml
└── src/
    ├── main.rs          — 진입점, 터미널 초기화 + 이벤트 루프
    ├── app.rs           — App 루트 상태, Game + 현재 Screen 보유
    ├── game.rs          — 핵심 게임 로직 (순수 데이터, TUI 비의존)
    ├── board.rs         — 10×20 그리드, 충돌 감지, 라인 제거
    ├── piece.rs         — 7종 테트로미노 + SRS 회전 테이블
    ├── bag.rs           — 7-bag 랜덤 생성기
    ├── ui.rs            — 모든 ratatui 렌더링 로직
    ├── input.rs         — crossterm 이벤트 → 게임 액션 매핑
    └── constants.rs     — 그리드 크기, 색상, tick 속도 등 상수
```

**핵심 레이어링**：`game / board / piece / bag`은 순수 로직 레이어로 TUI 의존성 제로. `ui / input / main`은 터미널 어댑터 레이어. 추후 TUI에서 GUI로 전환 시 `ui`와 `input` 두 파일만 교체하면 됨.

---

## 4. 게임 상태 머신

```rust
enum Screen {
    Menu,
    Playing(Game),      // Game이 Board + 현재 Piece + 점수를 보유
    Paused(Game),       // 일시정지 시 Game 상태 보존
    GameOver { score: u32 },
}
```

상태 전이：

```
Menu ──[Enter]──→ Playing
Playing ──[Esc]──→ Paused
Paused ──[Esc]──→ Playing
Paused ──[Q]───→ Menu
Playing ──[블록 천장 도달]──→ GameOver
GameOver ──[Enter]──→ Menu
```

---

## 5. TUI 레이아웃 (60fps 렌더링)

```
┌────────────┬──────────┐
│            │  NEXT    │
│            │ ┌──┐     │
│  10×20     │ │  │     │
│  플레이    │ └──┘     │
│  필드      │          │
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

ratatui Layout 분할：왼쪽 60% 플레이 필드, 오른쪽 40% 정보 패널, 하단 1줄 조작 힌트.

---

## 6. 렌더링 방식

각 셀을 2개의 공백 문자로 렌더링하여 터미널 셀을 정사각형에 가깝게 만듦 (터미널 문자 높이 ≈ 너비의 2배)：

```rust
fn render_board(frame: &mut Frame, board: &Board, area: Rect) {
    for row in 0..BOARD_ROWS {
        for col in 0..BOARD_COLS {
            let cell = board.grid[row][col];
            let style = Style::default().bg(cell.color);
            let span = Span::styled("  ", style);
            // 해당 위치에 렌더링
        }
    }
}
```

### 테트로미노 색상 (Iris 제공, 40년 검증된 표준 색상)

| 블록 | 문자 | 색상값 | 이유 |
|------|------|------|------|
| I | `I` | `#00F0F0` 시안 | 긴 막대, 가장 밝은 색, 낙하 시 눈에 띔 |
| O | `O` | `#F0F000` 옐로 | 정사각형, 가장 따뜻한 색, 안정감 |
| T | `T` | `#A000F0` 퍼플 | T자형, 고귀한 느낌, 가장 기억에 남음 |
| S | `S` | `#00F000` 그린 | Z의 거울상, "S = 초록" 연상법 |
| Z | `Z` | `#F00000` 레드 | "Z는 red와 운율", 영문권 고전적 기억법 |
| J | `J` | `#0000F0` 블루 | "J = blue" 시각적 정지감 |
| L | `L` | `#F0A000` 오렌지 | 남은 유일한 난색, 높은 식별성 |

---

## 7. 입력 처리 (crossterm 이벤트 루프)

```rust
// 키 이벤트를 게임 Action에 매핑
enum Action {
    MoveLeft,
    MoveRight,
    RotateCW,       // 시계 방향 회전
    RotateCCW,      // 반시계 방향 회전
    SoftDrop,       // 가속 낙하
    HardDrop,       // 즉시 바닥까지 낙하
    Hold,           // 블록 보관 (선택 사항)
    Pause,
    Quit,
}
```

논블로킹 입력 루프：

```rust
loop {
    if event::poll(Duration::from_millis(16))? {  // ~60fps
        if let Event::Key(key) = event::read()? {
            // 키 → Action → game.handle(action) 매핑
        }
    }
    game.tick();   // 중력 + 락 로직
    ui::render();  // 1프레임 렌더링
}
```

**키 매핑：**

| 키 | 동작 |
|------|------|
| ← → | 좌우 이동 |
| ↑ | 시계 방향 회전 |
| Z | 반시계 방향 회전 |
| ↓ | 소프트 드롭 (가속) |
| Space | 하드 드롭 (즉시 바닥까지) |
| C | Hold (블록 보관) |
| P | 일시정지 / 재개 |
| Q | 종료 |

---

## 8. Game Loop 타이밍

```rust
const TICK_BASE: Duration = Duration::from_millis(800);   // Lv1
const TICK_MIN:  Duration = Duration::from_millis(50);     // Lv20+
const LOCK_DELAY: Duration = Duration::from_millis(500);   // 접지 락 지연

fn tick(&mut self) {
    let now = Instant::now();
    // 1. 중력: tick 간격 도달 → 블록 1행 아래로 이동
    // 2. 락: 접지 + LOCK_DELAY 초과 → 블록을 board에 기록
    // 3. 라인 제거: 완전한 행 확인 → 제거 → 점수 추가
    // 4. 다음 블록: bag에서 가져오기
}
```

**락 딜레이가 진정한 조작감의 원천** — 착지 후 500ms의 유예 시간 동안 플레이어는 여전히 좌우 이동 및 회전 가능. 이 메커니즘이 없으면 게임 체험이 크게 저하됨.

---

## 9. SRS 회전 시스템 (Super Rotation System)

### 블록 정의 (piece.rs)

7종 블록, 각 4개 회전 상태, 직접 테이블 참조：

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
enum Tetromino { I, O, T, S, Z, J, L }

impl Tetromino {
    fn cells(&self, rotation: usize) -> [(i32, i32); 4] {
        match self {
            Tetromino::I => [
                [(0,1), (1,1), (2,1), (3,1)],  // state 0: 수평
                [(2,0), (2,1), (2,2), (2,3)],  // state 1
                [(0,2), (1,2), (2,2), (3,2)],  // state 2
                [(1,0), (1,1), (1,2), (1,3)],  // state 3
            ][rotation],
            // ... 기타 6종
        }
    }
}
```

### 월킥 오프셋 테이블

회전 시 충돌이 발생하면 SRS 표준에 따라 5개 그룹의 오프셋을 시도. 이것이 회전 조작감의 핵심：

```
회전 실패 → 오프셋1 시도 → 오프셋2 시도 → ... → 오프셋5 시도 → 회전 포기
```

[tetris.wiki/Super_Rotation_System](https://tetris.wiki/Super_Rotation_System)의 표준 월킥 테이블 참조.

---

## 10. 7-Bag 랜덤 생성기 (bag.rs)

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
        // Fisher-Yates 셔플
        // ...
        self.queue = pieces;
    }
}
```

7-Bag은 7개 블록마다 모든 종류가 각 1회씩 출현함을 보장하여 극단적인 랜덤성의 좌절감을 해소한다.

---

## 11. 충돌 감지 (board.rs)

```rust
impl Board {
    fn collides(&self, piece: &Piece, offset_x: i32, offset_y: i32) -> bool {
        for (x, y) in piece.cells() {
            let nx = piece.x + x + offset_x;
            let ny = piece.y + y + offset_y;
            if nx < 0 || nx >= COLS || ny >= ROWS { return true; }
            if ny < 0 { continue; } // 천장 위는 허용
            if self.grid[ny as usize][nx as usize].occupied { return true; }
        }
        false
    }
}
```

---

## 12. 단계별 구현 계획

| 단계 | 내용 | 검증 기준 |
|------|------|---------|
| **1** | `cargo init` + ratatui/crossterm 의존성 + 대체 화면 기동 | 터미널 클리어, Q로 종료 |
| **2** | `board.rs` + `piece.rs` — 그리드 데이터 구조 + 7종 블록 정의 | `cargo test` 전부 통과 |
| **3** | `ui.rs` — 빈 그리드 + 사이드바 레이아웃 렌더링 | 10×20 빈 그리드 + NEXT/SCORE 표시 |
| **4** | `game.rs` — Game Loop, 블록 낙하 + 입력 제어 | 이동, 회전, 자동 낙하 가능 |
| **5** | 충돌 감지 + 락 + 라인 제거 + 점수 | 완전한 게임 플레이 가능 |
| **6** | 상태 머신: Menu → Playing → Pause → GameOver + 타이틀 화면 | 완전한 게임 루프 |

---

## 13. 주요 아키텍처 결정 사항

- **순수 로직 레이어와 렌더링 레이어의 엄격한 분리** — board / piece / game은 ratatui를 import하지 않음
- **60fps 이벤트 루프 + 가변 중력 tick** — 이벤트 루프는 16ms 고정, 중력은 별도 타이머
- **SRS 테이블 참조 구현** — 회전 계산을 직접 하지 않고 tetris.wiki의 오프셋 테이블 직접 참조
- **각 셀을 공백 2개로 렌더링** — 터미널 사각형 블록을 위한 유일한 신뢰 가능한 방법
- **락 딜레이 500ms** — 착지 즉시 락하지 않고 최종 조정 허용

---

## 14. 타이틀 화면 (ASCII Art)

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

> 최종 업데이트: 2026-06-27
> 팀: Jasper (아키텍처) / Bram (엔지니어링) / Atlas (리서치) / Iris (비주얼 디자인) / Nova (제품 관리)
