# Open Tetris

Rust로 제작된 클래식 터미널 테트리스 게임. [ratatui](https://github.com/ratatui/ratatui)와 [crossterm](https://github.com/crossterm-rs/crossterm) 기반.

## 기능

- **SRS 회전 시스템** — 7가지 테트로미노 전체에 대한 월킥 테이블 적용
- **7-bag 랜덤 알고리즘** — Fisher-Yates 셔플로 공정한 블록 분배
- **락 딜레이** — 500ms 락 딜레이, 최대 15회 이동 리셋
- **고스트 피스** — 블록 착지 위치를 반투명 미리보기로 표시
- **점수 체계** — Single/Double/Triple/Tetris (100/300/500/800 × 레벨), 소프트 드롭 1점, 하드 드롭 행당 2점
- **레벨 진행** — 속도가 800ms에서 50ms까지 지수적으로 증가
- **상태 머신** — 메뉴 → 플레이 → 일시정지 → 게임 오버

## 빠른 시작

```bash
cargo run
```

## 조작법

| 키 | 동작 |
|------|------|
| ← → | 좌우 이동 |
| ↑ | 시계 방향 회전 |
| Z | 반시계 방향 회전 |
| ↓ | 소프트 드롭 |
| Space | 하드 드롭 |
| P | 일시정지 / 재개 |
| Q | 종료 |
| Enter | 시작 / 확인 |

## 기술 스택

- **Rust** — 시스템 프로그래밍 언어
- **ratatui** 0.29 — 터미널 UI 렌더링
- **crossterm** 0.28 — 터미널 입력 및 제어
- **rand** 0.8 — 난수 생성

## 프로젝트 구조

```
src/
├── main.rs       — 터미널 초기화 + 60fps 이벤트 루프
├── app.rs        — 상태 머신 (메뉴/플레이/일시정지/게임오버)
├── game.rs       — 핵심 로직: 중력, 락 딜레이, 점수
├── board.rs      — 10×20 그리드, 충돌 감지, 라인 제거
├── piece.rs      — 7가지 블록, SRS 회전 상태, 월킥 테이블
├── bag.rs        — 7-bag 랜덤 생성기
├── ui.rs         — ratatui 렌더링: 보드, 사이드 패널, 메뉴, 오버레이
├── input.rs      — crossterm 키 → 액션 매핑
└── constants.rs  — 그리드 크기, 타이밍, 점수 상수
```

## 라이선스

MIT
