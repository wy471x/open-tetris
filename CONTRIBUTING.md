# Contributing to Open Tetris

Thank you for your interest in Open Tetris!

## Code of Conduct

- Respect all contributors. Be kind and constructive.
- Stay focused on code and technical discussion.

## How to Contribute

### Reporting Bugs

1. Search existing GitHub Issues for the same problem
2. If not found, create a new Issue
3. Provide: steps to reproduce, expected behavior, actual behavior, terminal environment screenshot if possible

### Submitting Code

1. **Fork** this repository
2. Create a feature branch from `main`:
   ```bash
   git checkout -b feat/your-feature-name
   ```
3. Write code and ensure tests pass:
   ```bash
   cargo test
   cargo clippy -- -D warnings
   ```
4. Commit your changes:
   ```bash
   git commit -m "feat: describe your change"
   ```
5. Push to your fork:
   ```bash
   git push origin feat/your-feature-name
   ```
6. Create a Pull Request targeting `main`

### Commit Convention

Use [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` — new feature
- `fix:` — bug fix
- `refactor:` — code refactoring (no behavior change)
- `docs:` — documentation
- `test:` — add or update tests
- `chore:` — build, CI, dependencies, etc.

Examples:
```
feat: add hold piece functionality
fix: prevent wall kick through filled cells
refactor: extract SRS data into lookup tables
```

### Code Style

- Follow `cargo fmt` defaults
- Pass `cargo clippy -- -D warnings` with zero warnings
- Pure logic layer (`game.rs`, `board.rs`, `piece.rs`) must not import TUI dependencies
- Add brief comments to new public functions

### Branch Naming

| Type | Format |
|------|--------|
| Feature | `feat/description` |
| Bug fix | `fix/description` |
| Refactor | `refactor/description` |

### PR Review

- PR requires at least one maintainer approval
- Code must pass CI tests
- Keep PRs small and focused — one issue per PR

## Project Architecture

Strict separation between pure logic and rendering layers:

```
Logic Layer (zero TUI deps)    Terminal Adapter Layer
───────────────────────────    ─────────────────────
board.rs / piece.rs            ui.rs
bag.rs / game.rs               input.rs
constants.rs                   main.rs
```

Follow this layering principle when adding new features.

## Testing

```bash
# Run all tests
cargo test

# Run specific module tests
cargo test --lib board
cargo test --lib piece

# Code quality checks
cargo clippy -- -D warnings
cargo fmt --check
```

## License

This project is licensed under MIT. By contributing, you agree to release your code under this license.
