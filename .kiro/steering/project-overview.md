# wtx プロジェクト概要

## 背景と課題

### 問題

1. **VSCode/Kiro の workspace 管理が煩雑**

   - workspace ファイル（`.code-workspace`）の手動作成・管理が面倒
   - 複数リポジトリを扱うプロジェクトで特に顕著

2. **Git worktree との組み合わせが困難**

   - worktree は並列開発に最適だが、各 worktree に対する workspace 作成が手作業
   - 同じリポジトリの異なるブランチで並列作業する場合、worktree なしでは不可能

3. **典型的な課題シナリオ**
   ```
   # feature-auth と feature-payment を並列開発したい
   # 両方とも frontend + backend リポジトリが必要
   # → 手動で worktree 作成 + workspace 作成 = 時間がかかる
   ```

## ソリューション

`wtx` - Git worktree と VSCode workspace を統合管理する CLI ツール

### 名前の由来

- **w**ork**t**ree + workspace の **x**（拡張・統合）
- 短くて打ちやすい

### コンセプト

```bash
# 1. よく使うリポジトリを事前登録（bare clone される）
wtx register git@github.com:org/frontend.git
wtx register git@github.com:org/backend.git
# → ~/.wtx/frontend.git/, ~/.wtx/backend.git/ に bare clone

# 2. 作業ディレクトリで実行
cd ~/work/feature-auth
wtx

# 3. インタラクティブ UI で選択
#    - リポジトリを複数選択
#    - 各リポジトリのブランチを選択
#    - workspace 名を入力（省略可）

# 4. 自動生成される
~/work/feature-auth/
  ├── frontend/                  # worktree
  ├── backend/                   # worktree
  └── feature-auth.code-workspace
```

## 用語定義

| 用語                 | 定義                                                                                                   |
| -------------------- | ------------------------------------------------------------------------------------------------------ |
| **登録リポジトリ**   | `wtx register` で登録された Git リポジトリ。`~/.wtx/config.json` に保存                                |
| **worktree**         | Git の worktree 機能で作成された作業ディレクトリ。元リポジトリとは別の場所でブランチを checkout できる |
| **workspace**        | VSCode/Kiro の `.code-workspace` ファイル。複数フォルダを 1 つのウィンドウで開ける                     |
| **作業ディレクトリ** | `wtx` コマンドを実行するディレクトリ。ここに worktree と workspace が生成される                        |

## 機能一覧

### コア機能

| コマンド                | 説明                                                 |
| ----------------------- | ---------------------------------------------------- |
| `wtx register <url>`    | Git リポジトリを bare clone して登録                 |
| `wtx list`              | 登録済みリポジトリ一覧表示                           |
| `wtx unregister <name>` | 登録解除                                             |
| `wtx`                   | インタラクティブ UI 起動 → worktree + workspace 生成 |
| `wtx clean`             | 作業ディレクトリの worktree + workspace を削除       |

### インタラクティブ UI フロー

```
┌─ wtx ─────────────────────────────────────────────────┐
│ Select repositories (Space: toggle, Enter: confirm)  │
│                                                       │
│ ◉ frontend     [main]           ▼ (ブランチ選択)     │
│ ◉ backend      [develop]        ▼                    │
│ ○ shared-lib   [main]           ▼                    │
│                                                       │
│ Workspace name: feature-auth (optional)              │
│                                                       │
│ [ ] Open in VSCode/Kiro after creation               │
└───────────────────────────────────────────────────────┘
```

## 技術選択

### 言語: Rust

**選定理由:**

- ratatui によるリッチな TUI（テキストユーザーインターフェース）
- シングルバイナリで配布可能（依存関係なし）
- 高速な実行速度
- 型安全性とメモリ安全性

### 主要クレート

| クレート    | 用途                               |
| ----------- | ---------------------------------- |
| `ratatui`   | TUI フレームワーク                 |
| `crossterm` | クロスプラットフォーム端末操作     |
| `git2`      | Git 操作（libgit2 バインディング） |
| `clap`      | CLI パーサー                       |
| `serde`     | JSON シリアライズ/デシリアライズ   |
| `tokio`     | 非同期ランタイム（必要に応じて）   |

### ディレクトリ構造

```
~/.wtx/
  ├── config.json           # 登録リポジトリ一覧（メタデータ）
  ├── frontend.git/         # bare リポジトリ（clone）
  └── backend.git/          # bare リポジトリ（clone）
```

### 設定ファイル

```json
{
  "repositories": [
    {
      "name": "frontend",
      "remote": "git@github.com:org/frontend.git",
      "localPath": "~/.wtx/frontend.git"
    },
    {
      "name": "backend",
      "remote": "git@github.com:org/backend.git",
      "localPath": "~/.wtx/backend.git"
    }
  ]
}
```

### bare リポジトリを使う理由

- worktree 専用の親リポジトリとして最適
- 作業ディレクトリが不要でディスク効率が良い
- 複数の worktree を同時に作成可能
- 元のリポジトリの場所に依存しない

## Spec 分割方針

機能ごとに独立した spec として実装:

```
specs/
├── repo-registration/     # register / list / unregister
├── workspace-generation/  # worktree 作成 + workspace 生成
└── interactive-ui/        # インタラクティブ選択 UI
```

### 実装順序

1. **repo-registration** - 他機能の前提となる基盤
2. **workspace-generation** - コアロジック
3. **interactive-ui** - 1, 2 を統合する UI 層

## 非機能要件

- **対応 OS**: macOS, Linux
- **Rust バージョン**: 1.70 以上（Edition 2021）
- **配布方法**:
  - GitHub Releases（バイナリ）
  - Homebrew（macOS）
  - cargo install
- **エラーハンドリング**: ユーザーフレンドリーなエラーメッセージ
- **テスト**: 単体テスト + プロパティベーステスト（proptest）
