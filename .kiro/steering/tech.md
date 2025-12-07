# wtx - 技術スタック

## 言語・ランタイム

- **Rust** (Edition 2021, 1.70+)
- シングルバイナリ配布

## 主要クレート

| クレート     | 用途                               |
| ------------ | ---------------------------------- |
| `clap`       | CLI パーサー（derive マクロ使用）  |
| `ratatui`    | TUI フレームワーク                 |
| `crossterm`  | クロスプラットフォーム端末操作     |
| `git2`       | Git 操作（libgit2 バインディング） |
| `serde`      | JSON シリアライズ                  |
| `serde_json` | JSON パース                        |
| `thiserror`  | エラー型定義                       |
| `anyhow`     | エラーハンドリング                 |
| `color-eyre` | エラーレポート                     |
| `dirs`       | ホームディレクトリ取得             |
| `console`    | ターミナル出力スタイリング         |

## テスト用クレート

| クレート   | 用途                   |
| ---------- | ---------------------- |
| `tempfile` | 一時ディレクトリ       |
| `proptest` | プロパティベーステスト |
| `rstest`   | パラメータ化テスト     |
| `mockall`  | モック生成             |

## よく使うコマンド

```bash
# ビルド
cargo build
cargo build --release

# テスト
cargo test
cargo test -- --nocapture  # 出力表示

# 実行
cargo run -- register <url>
cargo run -- list

# リント・フォーマット
cargo fmt
cargo clippy

# ドキュメント生成
cargo doc --open
```

## リリースビルド最適化

```toml
[profile.release]
codegen-units = 1
lto = true
opt-level = "s"
strip = true
```
