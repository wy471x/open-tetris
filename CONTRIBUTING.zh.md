# Contributing to Open Tetris

感谢你对 Open Tetris 的关注！

## 行为准则

- 尊重所有贡献者，保持友善和建设性的交流
- 专注于代码和技术讨论，避免无关话题

## 如何贡献

### 报告 Bug

1. 在 GitHub Issues 中搜索是否已有相同问题
2. 如果没有，创建一个新的 Issue
3. 尽可能提供：复现步骤、预期行为、实际行为、终端环境截图

### 提交代码

1. **Fork** 本仓库
2. 从 `main` 分支创建你的 feature 分支：
   ```bash
   git checkout -b feat/your-feature-name
   ```
3. 编写代码，并确保测试通过：
   ```bash
   cargo test
   cargo clippy -- -D warnings
   ```
4. 提交你的改动：
   ```bash
   git commit -m "feat: describe your change"
   ```
5. 推送到你的 Fork：
   ```bash
   git push origin feat/your-feature-name
   ```
6. 创建 Pull Request 到 `main` 分支

### Commit 规范

使用 [Conventional Commits](https://www.conventionalcommits.org/) 格式：

- `feat:` — 新功能
- `fix:` — Bug 修复
- `refactor:` — 代码重构（不改变功能）
- `docs:` — 文档变更
- `test:` — 添加或修改测试
- `chore:` — 构建、CI、依赖等杂项

示例：
```
feat: add hold piece functionality
fix: prevent wall kick through filled cells
refactor: extract SRS data into lookup tables
```

### 代码风格

- 遵循 `cargo fmt` 默认格式
- 通过 `cargo clippy -- -D warnings` 零警告
- 纯逻辑层（`game.rs`、`board.rs`、`piece.rs`）不引入 TUI 依赖
- 新增公开函数添加简要注释说明用途

### 分支命名

| 类型 | 格式 |
|------|------|
| 新功能 | `feat/功能描述` |
| Bug 修复 | `fix/问题描述` |
| 重构 | `refactor/重构描述` |

### PR 审查

- PR 至少需要一位维护者审查通过
- 代码必须通过 CI 测试
- 保持 PR 小而聚焦，一个 PR 只解决一个问题

## 项目架构

纯逻辑层与渲染层严格分离：

```
逻辑层 (零 TUI 依赖)      终端适配层
─────────────────────    ────────────
board.rs / piece.rs      ui.rs
bag.rs / game.rs         input.rs
constants.rs             main.rs
```

添加新功能时请遵循这一分层原则。

## 测试

```bash
# 运行所有测试
cargo test

# 运行特定模块测试
cargo test --lib board
cargo test --lib piece

# 检查代码质量
cargo clippy -- -D warnings
cargo fmt --check
```

## 许可证

本项目采用 MIT 许可证。贡献代码即视为你同意在此许可证下发布你的代码。
