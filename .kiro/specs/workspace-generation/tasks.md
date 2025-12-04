# 実装計画

- [ ] 1. workspace 関連のデータモデル定義
- [x] 1.1 worktree 関連のデータモデル定義

  - src/models/workspace.rs を作成
  - WorktreeSelection（リポジトリ名、ブランチ名）を定義
  - WorktreeInfo（パス、ブランチ、リポジトリ名）を定義
  - BranchInfo（名前、デフォルトフラグ）を定義
  - _要件: 1.1, 3.1_

- [x] 1.2 workspace ファイル関連のデータモデル定義

  - WorkspaceFile（folders、settings）を定義
  - WorkspaceFolder（path）を定義
  - Serialize/Deserialize を実装
  - _要件: 2.1, 5.1_

- [x] 1.3 操作結果のデータモデル定義

  - CleanTarget（Workspace、All）を定義
  - GenerationResult（worktrees、workspace_file）を定義
  - CleanResult（removed_worktrees、removed_workspace_files、warnings）を定義
  - _要件: 4.1_

- [x] 1.4 WorkspaceError エラー型の定義

  - src/models/error.rs に WorkspaceError を追加
  - RepositoryNotFound、WorktreeAlreadyExists、WorkspaceFileAlreadyExists 等を定義
  - src/models/mod.rs を更新してモジュールをエクスポート
  - _要件: 6.1, 6.2, 6.3_

- [ ] 2. WorktreeManager の実装（TDD）
- [x] 2.1 WorktreeManager 構造体と new() の実装

  - src/workspace/worktree.rs を作成
  - WorktreeManager 構造体と new() メソッドを実装
  - src/workspace/mod.rs を作成してモジュールをエクスポート
  - _要件: 1.1_

- [x] 2.2 fetch() のテストと実装

  - fetch() のテストを作成（Red）
  - fetch() を実装（git2 を使用）
  - テストが通ることを確認（Green）
  - _要件: 3.2_

- [x] 2.3 get_remote_branches() のテストと実装

  - get_remote_branches() のテストを作成（Red）
  - get_remote_branches() を実装
  - テストが通ることを確認（Green）
  - _要件: 3.1_

- [x] 2.4 branch_exists() のテストと実装

  - branch_exists() のテストを作成（Red）
  - branch_exists() を実装
  - テストが通ることを確認（Green）
  - _要件: 1.3, 1.4_

- [x] 2.5 create_worktree() のテストと実装（既存ブランチ）

  - create_worktree() の既存ブランチテストを作成（Red）
  - create_worktree() を実装（既存ブランチの checkout）
  - テストが通ることを確認（Green）
  - _要件: 1.1, 1.2, 1.3_

- [ ] 2.6 create_worktree() のテストと実装（新規ブランチ）

  - create_worktree() の新規ブランチテストを作成（Red）
  - create_worktree() を拡張（新規ブランチの作成）
  - テストが通ることを確認（Green）
  - _要件: 1.4_

- [ ] 2.7 create_worktree() の重複エラーテストと実装

  - 重複 worktree 作成時のエラーテストを作成（Red）
  - 重複チェックを実装
  - テストが通ることを確認（Green）
  - _要件: 1.5_

- [x] 2.8 list_worktrees() のテストと実装

  - list_worktrees() のテストを作成（Red）
  - list_worktrees() を実装
  - テストが通ることを確認（Green）
  - _要件: 4.1_

- [ ] 2.9 remove_worktree() のテストと実装

  - remove_worktree() のテストを作成（Red）
  - remove_worktree() を実装（prune 含む）
  - テストが通ることを確認（Green）
  - _要件: 4.4_

- [ ]\* 2.10 WorktreeManager のプロパティテストを作成

  - **プロパティ 1: worktree 作成後のディレクトリ存在**
  - **検証: 要件 1.1**

- [ ]\* 2.11 WorktreeManager のプロパティテストを作成

  - **プロパティ 2: worktree の親リポジトリ検証**
  - **検証: 要件 1.2**

- [ ]\* 2.12 WorktreeManager のプロパティテストを作成

  - **プロパティ 3: 既存ブランチの checkout**
  - **検証: 要件 1.3**

- [ ]\* 2.13 WorktreeManager のプロパティテストを作成

  - **プロパティ 4: 新規ブランチの作成**
  - **検証: 要件 1.4**

- [ ]\* 2.14 WorktreeManager のプロパティテストを作成

  - **プロパティ 5: 重複 worktree の拒否**
  - **検証: 要件 1.5**

- [ ]\* 2.15 WorktreeManager のプロパティテストを作成

  - **プロパティ 10: ブランチ一覧の完全性**
  - **検証: 要件 3.1**

- [ ]\* 2.16 WorktreeManager のプロパティテストを作成

  - **プロパティ 11: fetch の実行**
  - **検証: 要件 3.2**

- [ ]\* 2.17 WorktreeManager のプロパティテストを作成

  - **プロパティ 12: デフォルトブランチの優先**
  - **検証: 要件 3.3**

- [ ]\* 2.18 WorktreeManager のプロパティテストを作成

  - **プロパティ 15: worktree prune の実行**
  - **検証: 要件 4.4**

- [ ] 3. WorkspaceFileManager の実装（TDD）
- [ ] 3.1 WorkspaceFileManager 構造体と new() の実装

  - src/workspace/file.rs を作成
  - WorkspaceFileManager 構造体と new() メソッドを実装
  - src/workspace/mod.rs を更新してモジュールをエクスポート
  - _要件: 2.1_

- [ ] 3.2 generate() のテストと実装

  - generate() のテストを作成（正常系）（Red）
  - generate() を実装（JSON 生成）
  - テストが通ることを確認（Green）
  - _要件: 2.1, 2.2, 2.3, 5.1, 5.2, 5.3_

- [ ] 3.3 generate() の重複エラーテストと実装

  - 重複 workspace ファイル作成時のエラーテストを作成（Red）
  - 重複チェックを実装
  - テストが通ることを確認（Green）
  - _要件: 2.5_

- [ ] 3.4 read() のテストと実装

  - read() のテストを作成（有効/無効な JSON）（Red）
  - read() を実装（JSON 読み込みと検証）
  - テストが通ることを確認（Green）
  - _要件: 5.4_

- [ ] 3.5 exists() のテストと実装

  - exists() のテストを作成（Red）
  - exists() を実装
  - テストが通ることを確認（Green）
  - _要件: 2.5_

- [ ] 3.6 delete() のテストと実装

  - delete() のテストを作成（Red）
  - delete() を実装
  - テストが通ることを確認（Green）
  - _要件: 4.5_

- [ ]\* 3.7 WorkspaceFileManager のプロパティテストを作成

  - **プロパティ 6: workspace ファイル生成**
  - **検証: 要件 2.1**

- [ ]\* 3.8 WorkspaceFileManager のプロパティテストを作成

  - **プロパティ 7: folders 配列の完全性**
  - **検証: 要件 2.2, 5.2**

- [ ]\* 3.9 WorkspaceFileManager のプロパティテストを作成

  - **プロパティ 8: workspace ファイル名の正確性**
  - **検証: 要件 2.3**

- [ ]\* 3.10 WorkspaceFileManager のプロパティテストを作成

  - **プロパティ 9: 重複 workspace ファイルの拒否**
  - **検証: 要件 2.5**

- [ ]\* 3.11 WorkspaceFileManager のプロパティテストを作成

  - **プロパティ 17: JSON 形式の有効性**
  - **検証: 要件 5.1**

- [ ]\* 3.12 WorkspaceFileManager のプロパティテストを作成

  - **プロパティ 18: settings オブジェクトの存在**
  - **検証: 要件 5.3**

- [ ]\* 3.13 WorkspaceFileManager のプロパティテストを作成

  - **プロパティ 19: JSON 構造検証**
  - **検証: 要件 5.4**

- [ ] 4. チェックポイント - すべてのテストが合格することを確認

  - すべてのテストが合格することを確認し、問題があればユーザーに質問する

- [ ] 5. WorkspaceGenerationService の実装（TDD）
- [ ] 5.1 WorkspaceGenerationService 構造体と new() の実装

  - src/workspace/service.rs を作成
  - WorkspaceGenerationService 構造体と new() メソッドを実装
  - src/workspace/mod.rs を更新してモジュールをエクスポート
  - _要件: 1.1_

- [ ] 5.2 get_branches() のテストと実装

  - get_branches() のテストを作成（Red）
  - get_branches() を実装（fetch + ブランチ一覧取得）
  - テストが通ることを確認（Green）
  - _要件: 3.1, 3.2, 3.3, 3.4_

- [ ] 5.3 generate() のテストと実装（単一 worktree）

  - generate() の単一 worktree テストを作成（Red）
  - generate() を実装（worktree 作成 + workspace 生成）
  - テストが通ることを確認（Green）
  - _要件: 1.1, 1.2, 1.3, 2.1, 2.2, 2.3, 2.4_

- [ ] 5.4 generate() のテストと実装（複数 worktree）

  - generate() の複数 worktree テストを作成（Red）
  - generate() を拡張（複数 worktree 対応）
  - テストが通ることを確認（Green）
  - _要件: 1.1, 1.2, 2.2_

- [ ] 5.5 generate() のロールバックテストと実装

  - 部分的作成後エラー時のロールバックテストを作成（Red）
  - ロールバック機能を実装
  - テストが通ることを確認（Green）
  - _要件: 6.5_

- [ ] 5.6 clean() のテストと実装（指定 workspace）

  - clean() の指定 workspace テストを作成（Red）
  - clean() を実装（指定 workspace 削除）
  - テストが通ることを確認（Green）
  - _要件: 4.1, 4.4, 4.5_

- [ ] 5.7 clean() のテストと実装（全体削除）

  - clean() の全体削除テストを作成（Red）
  - clean() を拡張（全体削除対応）
  - テストが通ることを確認（Green）
  - _要件: 4.2, 4.6, 4.7_

- [ ]\* 5.8 WorkspaceGenerationService のプロパティテストを作成

  - **プロパティ 13: 指定 workspace のクリーンアップ**
  - **検証: 要件 4.1**

- [ ]\* 5.9 WorkspaceGenerationService のプロパティテストを作成

  - **プロパティ 14: 全体クリーンアップ**
  - **検証: 要件 4.2**

- [ ]\* 5.10 WorkspaceGenerationService のプロパティテストを作成

  - **プロパティ 16: 管理外ファイルの保護**
  - **検証: 要件 4.6**

- [ ]\* 5.11 WorkspaceGenerationService のプロパティテストを作成

  - **プロパティ 20: ロールバック動作**
  - **検証: 要件 6.5**

- [ ] 6. チェックポイント - すべてのテストが合格することを確認

  - すべてのテストが合格することを確認し、問題があればユーザーに質問する

- [ ] 7. clean コマンドの実装（TDD）
- [ ] 7.1 CleanArgs 構造体と CLI 定義

  - src/commands/clean.rs を作成
  - CleanArgs 構造体を定義（workspace 名、--all フラグ）
  - src/cli/mod.rs に Clean サブコマンドを追加
  - _要件: 4.1, 4.2, 4.3_

- [ ] 7.2 clean コマンド（引数なし）のテストと実装

  - `wtx clean`（引数なし）のテストを作成（Red）
  - 使用方法表示を実装
  - テストが通ることを確認（Green）
  - _要件: 4.3_

- [ ] 7.3 clean コマンド（指定 workspace）のテストと実装

  - `wtx clean <workspace名>` のテストを作成（Red）
  - 指定 workspace 削除を実装
  - テストが通ることを確認（Green）
  - _要件: 4.1_

- [ ] 7.4 clean コマンド（--all）のテストと実装

  - `wtx clean --all` のテストを作成（Red）
  - 全体削除を実装
  - src/commands/mod.rs を更新
  - src/main.rs で clean コマンドを統合
  - テストが通ることを確認（Green）
  - _要件: 4.2_

- [ ]\* 7.5 clean コマンドの統合テストを作成

  - worktree 作成 → clean <workspace 名> → 検証フロー
  - worktree 作成 → clean --all → 検証フロー
  - _要件: 4.1, 4.2, 4.3, 4.4, 4.5, 4.6, 4.7_

- [ ] 8. エラーハンドリングの実装
- [ ] 8.1 未登録リポジトリエラーのテストと実装

  - 未登録リポジトリエラーのテストを作成（Red）
  - エラーメッセージを実装
  - テストが通ることを確認（Green）
  - _要件: 6.1_

- [ ] 8.2 Git 操作エラーのテストと実装

  - Git 操作エラーのテストを作成（Red）
  - エラーメッセージを実装
  - テストが通ることを確認（Green）
  - _要件: 6.2_

- [ ] 8.3 ファイルシステムエラーのテストと実装

  - ファイルシステムエラーのテストを作成（Red）
  - エラーメッセージを実装
  - テストが通ることを確認（Green）
  - _要件: 6.3_

- [ ] 9. 最終チェックポイント - すべてのテストが合格することを確認
  - すべてのテストが合格することを確認し、問題があればユーザーに質問する
