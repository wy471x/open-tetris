# Open Tetris へのコントリビューション

Open Tetris に興味をお持ちいただきありがとうございます！

## 行動規範

- すべてのコントリビューターを尊重し、親切かつ建設的なコミュニケーションを心がけてください
- コードと技術的な議論に集中し、関係のない話題は避けてください

## コントリビューション方法

### バグ報告

1. GitHub Issues で同じ問題が報告されていないか検索
2. 見つからない場合は新しい Issue を作成
3. 再現手順、期待される動作、実際の動作、端末環境のスクリーンショットを提供

### コードの提出

1. このリポジトリを **Fork**
2. `main` ブランチからフィーチャーブランチを作成：
   ```bash
   git checkout -b feat/your-feature-name
   ```
3. コードを書き、テストが通ることを確認：
   ```bash
   cargo test
   cargo clippy -- -D warnings
   ```
4. 変更をコミット：
   ```bash
   git commit -m "feat: describe your change"
   ```
5. フォークにプッシュ：
   ```bash
   git push origin feat/your-feature-name
   ```
6. `main` ブランチに対してプルリクエストを作成

### コミット規約

[Conventional Commits](https://www.conventionalcommits.org/) 形式を使用：

- `feat:` — 新機能
- `fix:` — バグ修正
- `refactor:` — リファクタリング（動作変更なし）
- `docs:` — ドキュメント
- `test:` — テストの追加・更新
- `chore:` — ビルド、CI、依存関係など

例：
```
feat: add hold piece functionality
fix: prevent wall kick through filled cells
refactor: extract SRS data into lookup tables
```

### コードスタイル

- `cargo fmt` のデフォルト設定に従う
- `cargo clippy -- -D warnings` をゼロ警告で通過
- 純粋ロジック層（`game.rs`、`board.rs`、`piece.rs`）は TUI 依存をインポートしない
- 新しい公開関数には簡潔なコメントを追加

### ブランチ命名

| 種類 | 形式 |
|------|------|
| 新機能 | `feat/説明` |
| バグ修正 | `fix/説明` |
| リファクタ | `refactor/説明` |

### PR レビュー

- PR には最低 1 人のメンテナー承認が必要
- コードは CI テストを通過すること
- PR は小さく焦点を絞り、1 つの PR で 1 つの課題のみ対応

## プロジェクトアーキテクチャ

純粋ロジック層とレンダリング層の厳密な分離：

```
ロジック層 (TUI 依存なし)      端末アダプター層
─────────────────────────    ────────────────
board.rs / piece.rs          ui.rs
bag.rs / game.rs             input.rs
constants.rs                 main.rs
```

新機能を追加する際はこのレイヤー原則に従ってください。

## テスト

```bash
# すべてのテストを実行
cargo test

# 特定モジュールのテストを実行
cargo test --lib board
cargo test --lib piece

# コード品質チェック
cargo clippy -- -D warnings
cargo fmt --check
```

## ライセンス

本プロジェクトは MIT ライセンスです。コードを提供することにより、このライセンスの下でコードを公開することに同意したものとみなします。
