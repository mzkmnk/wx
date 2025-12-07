# wtx - プロジェクト構造

## アーキテクチャ

レイヤードアーキテクチャを採用:

```
src/
├── main.rs              # エントリーポイント
├── cli/                 # CLI 定義（clap）
├── commands/            # コマンドハンドラ（薄いレイヤー）
├── services/            # ビジネスロジック
├── infrastructure/      # 外部システムとの接続
│   ├── config/          # 設定ファイル管理
│   ├── filesystem/      # ファイルシステム操作
│   └── git/             # Git 操作
├── models/              # ドメインモデル・エラー型
├── tui/                 # TUI アプリケーション
└── utils/               # ユーティリティ・テストヘルパー
```

## レイヤー責務

| レイヤー         | 責務                                      |
| ---------------- | ----------------------------------------- |
| `cli`            | コマンドライン引数の定義                  |
| `commands`       | CLI → Service の橋渡し（薄く保つ）        |
| `services`       | ビジネスロジックの実装                    |
| `infrastructure` | 外部リソース（ファイル、Git）へのアクセス |
| `models`         | データ構造、エラー型の定義                |
| `tui`            | インタラクティブ UI                       |
| `utils`          | 共通ユーティリティ、テストヘルパー        |

## 主要な型

- `Config` - 設定ファイル構造（repositories リスト）
- `Repository` - 登録リポジトリ情報（name, remote, local_path）
- `WtxError` - 統一エラー型（thiserror 使用）
- `RepositoryService` - リポジトリ登録・解除のビジネスロジック
- `ConfigManager` - 設定ファイルの読み書き・バックアップ

## 依存関係の方向

```
cli → commands → services → infrastructure
                    ↓
                 models
```

- `models` は他のレイヤーに依存しない
- `infrastructure` は `models` のみに依存
- `services` は `infrastructure` と `models` に依存

## テスト構成

- 各モジュール内に `#[cfg(test)] mod tests` でユニットテスト
- `utils/test_helpers.rs` に共通テストヘルパー関数
- `tempfile` で一時ディレクトリを使用した統合テスト
