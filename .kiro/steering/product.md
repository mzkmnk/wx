# wx - プロダクト概要

## 概要

wx は Git worktree と VSCode/Kiro workspace を統合管理する CLI ツール。

## 解決する課題

- VSCode workspace ファイル（`.code-workspace`）の手動管理が煩雑
- Git worktree との組み合わせが困難
- 複数リポジトリを扱う並列開発でのセットアップ時間

## コアコンセプト

```bash
# 1. リポジトリを事前登録（bare clone）
wx register git@github.com:org/frontend.git

# 2. 作業ディレクトリで実行
cd ~/work/feature-auth
wx

# 3. インタラクティブ UI でリポジトリ・ブランチを選択
# 4. worktree + workspace が自動生成される
```

## 主要コマンド

| コマンド               | 説明                                            |
| ---------------------- | ----------------------------------------------- |
| `wx register <url>`    | Git リポジトリを bare clone して登録            |
| `wx list`              | 登録済みリポジトリ一覧表示                      |
| `wx unregister <name>` | 登録解除                                        |
| `wx`                   | インタラクティブ UI → worktree + workspace 生成 |
| `wx clean`             | worktree + workspace を削除                     |

## データ保存先

- 設定ファイル: `~/.wx/config.json`
- bare リポジトリ: `~/.wx/<repo-name>.git/`
