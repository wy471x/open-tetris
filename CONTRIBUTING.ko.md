# Open Tetris 기여 가이드

Open Tetris에 관심을 가져주셔서 감사합니다!

## 행동 강령

- 모든 기여자를 존중하고 친절하며 건설적인 대화를 나눠주세요
- 코드와 기술 토론에 집중하고 관련 없는 주제는 피해주세요

## 기여 방법

### 버그 신고

1. GitHub Issues에서 동일한 문제가 이미 보고되었는지 검색
2. 없으면 새로운 Issue 생성
3. 재현 단계, 예상 동작, 실제 동작, 터미널 환경 스크린샷을 최대한 제공

### 코드 제출

1. 본 저장소를 **Fork**
2. `main` 브랜치에서 기능 브랜치 생성:
   ```bash
   git checkout -b feat/your-feature-name
   ```
3. 코드 작성 후 테스트 통과 확인:
   ```bash
   cargo test
   cargo clippy -- -D warnings
   ```
4. 변경사항 커밋:
   ```bash
   git commit -m "feat: describe your change"
   ```
5. Fork에 푸시:
   ```bash
   git push origin feat/your-feature-name
   ```
6. `main` 브랜치로 Pull Request 생성

### 커밋 규칙

[Conventional Commits](https://www.conventionalcommits.org/) 형식을 사용합니다:

- `feat:` — 새로운 기능
- `fix:` — 버그 수정
- `refactor:` — 코드 리팩터링 (동작 변경 없음)
- `docs:` — 문서 변경
- `test:` — 테스트 추가/수정
- `chore:` — 빌드, CI, 의존성 등

예시:
```
feat: add hold piece functionality
fix: prevent wall kick through filled cells
refactor: extract SRS data into lookup tables
```

### 코드 스타일

- `cargo fmt` 기본 설정 준수
- `cargo clippy -- -D warnings` 경고 제로 통과
- 순수 로직 레이어(`game.rs`, `board.rs`, `piece.rs`)는 TUI 의존성을 import 하지 않음
- 새로운 공개 함수에는 간결한 주석 추가

### 브랜치 명명

| 유형 | 형식 |
|------|------|
| 새 기능 | `feat/설명` |
| 버그 수정 | `fix/설명` |
| 리팩터링 | `refactor/설명` |

### PR 리뷰

- PR은 최소 1명의 메인테이너 승인이 필요
- 코드는 CI 테스트를 통과해야 함
- PR은 작고 집중적으로 유지 — 하나의 PR에 하나의 이슈만

## 프로젝트 아키텍처

순수 로직 레이어와 렌더링 레이어의 엄격한 분리:

```
로직 레이어 (TUI 의존성 제로)    터미널 어댑터 레이어
────────────────────────────    ──────────────────
board.rs / piece.rs             ui.rs
bag.rs / game.rs                input.rs
constants.rs                    main.rs
```

새 기능을 추가할 때 이 레이어 원칙을 따라주세요.

## 테스트

```bash
# 모든 테스트 실행
cargo test

# 특정 모듈 테스트 실행
cargo test --lib board
cargo test --lib piece

# 코드 품질 검사
cargo clippy -- -D warnings
cargo fmt --check
```

## 라이선스

본 프로젝트는 MIT 라이선스입니다. 코드를 기여함으로써 이 라이선스 하에 코드를 배포하는 데 동의하는 것으로 간주합니다.
