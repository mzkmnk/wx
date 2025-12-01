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
- [ ] 2.1 WorktreeManager のテストを作成

  - create_worktree() のテスト（既存ブランチ、新規ブランチ）
  - remove_worktree() のテスト
  - list_worktrees() のテスト
  - fetch() のテスト
  - get_remote_branches() のテスト
  - branch_exists() のテスト
  - テストが失敗することを確認（Red）
  - _要件: 1.1, 1.2, 1.3, 1.4, 3.1, 3.2, 4.4_

- [ ] 2.2 WorktreeManager の実装

  - src/workspace/worktree.rs を作成
  - WorktreeManager 構造体と new() メソッドを実装
  - create_worktree() を実装（git2 を使用）
  - remove_worktree() を実装（prune 含む）
  - list_worktrees() を実装
  - fetch() を実装
  - get_remote_branches() を実装
  - branch_exists() を実装
  - テストが通ることを確認（Green）
  - _要件: 1.1, 1.2, 1.3, 1.4, 3.1, 3.2, 4.4_

- [ ]\* 2.3 WorktreeManager のプロパティテストを作成

  - **プロパティ 1: worktree 作成後のディレクトリ存在**
  - **検証: 要件 1.1**

- [ ]\* 2.4 WorktreeManager のプロパティテストを作成

  - **プロパティ 2: worktree の親リポジトリ検証**
  - **検証: 要件 1.2**

- [ ]\* 2.5 WorktreeManager のプロパティテストを作成

  - **プロパティ 3: 既存ブランチの checkout**
  - **検証: 要件 1.3**

- [ ]\* 2.6 WorktreeManager のプロパティテストを作成

  - **プロパティ 4: 新規ブランチの作成**
  - **検証: 要件 1.4**

- [ ]\* 2.7 WorktreeManager のプロパティテストを作成

  - **プロパティ 5: 重複 worktree の拒否**
  - **検証: 要件 1.5**

- [ ]\* 2.8 WorktreeManager のプロパティテストを作成

  - **プロパティ 10: ブランチ一覧の完全性**
  - **検証: 要件 3.1**

- [ ]\* 2.9 WorktreeManager のプロパティテストを作成

  - **プロパティ 11: fetch の実行**
  - **検証: 要件 3.2**

- [ ]\* 2.10 WorktreeManager のプロパティテストを作成

  - **プロパティ 12: デフォルトブランチの優先**
  - **検証: 要件 3.3**

- [ ]\* 2.11 WorktreeManager のプロパティテストを作成

  - **プロパティ 15: worktree prune の実行**
  - **検証: 要件 4.4**

- [ ] 3. WorkspaceFileManager の実装（TDD）
- [ ] 3.1 WorkspaceFileManager のテストを作成

  - generate() のテスト（正常系）
  - read() のテスト（有効/無効な JSON）
  - delete() のテスト
  - exists() のテスト
  - テストが失敗することを確認（Red）
  - _要件: 2.1, 2.2, 2.3, 5.1, 5.2, 5.3, 5.4_

- [ ] 3.2 WorkspaceFileManager の実装

  - src/workspace/file.rs を作成
  - WorkspaceFileManager 構造体と new() メソッドを実装
  - generate() を実装（JSON 生成）
  - read() を実装（JSON 読み込みと検証）
  - delete() を実装
  - exists() を実装
  - テストが通ることを確認（Green）
  - _要件: 2.1, 2.2, 2.3, 5.1, 5.2, 5.3, 5.4_

- [ ]\* 3.3 WorkspaceFileManager のプロパティテストを作成

  - **プロパティ 6: workspace ファイル生成**
  - **検証: 要件 2.1**

- [ ]\* 3.4 WorkspaceFileManager のプロパティテストを作成

  - **プロパティ 7: folders 配列の完全性**
  - **検証: 要件 2.2, 5.2**

- [ ]\* 3.5 WorkspaceFileManager のプロパティテストを作成

  - **プロパティ 8: workspace ファイル名の正確性**
  - **検証: 要件 2.3**

- [ ]\* 3.6 WorkspaceFileManager のプロパティテストを作成

  - **プロパティ 9: 重複 workspace ファイルの拒否**
  - **検証: 要件 2.5**

- [ ]\* 3.7 WorkspaceFileManager のプロパティテストを作成

  - **プロパティ 17: JSON 形式の有効性**
  - **検証: 要件 5.1**

- [ ]\* 3.8 WorkspaceFileManager のプロパティテストを作成

  - **プロパティ 18: settings オブジェクトの存在**
  - **検証: 要件 5.3**

- [ ]\* 3.9 WorkspaceFileManager のプロパティテストを作成

  - **プロパティ 19: JSON 構造検証**
  - **検証: 要件 5.4**

- [ ] 4. チェックポイント - すべてのテストが合格することを確認

  - すべてのテストが合格することを確認し、問題があればユーザーに質問する

- [ ] 5. WorkspaceGenerationService の実装（TDD）
- [ ] 5.1 WorkspaceGenerationService のテストを作成

  - generate() のテスト（単一/複数 worktree）
  - clean() のテスト（指定/全体）
  - get_branches() のテスト
  - ロールバック動作のテスト
  - テストが失敗することを確認（Red）
  - _要件: 1.1, 1.2, 1.3, 1.4, 1.5, 2.1, 2.2, 2.3, 2.4, 2.5, 3.1, 3.2, 3.3, 3.4, 4.1, 4.2, 4.3, 4.4, 4.5, 4.6, 4.7, 6.5_

- [ ] 5.2 WorkspaceGenerationService の実装

  - src/workspace/service.rs を作成
  - WorkspaceGenerationService 構造体と new() メソッドを実装
  - generate() を実装（worktree 作成 + workspace 生成）
  - clean() を実装（指定/全体削除）
  - get_branches() を実装
  - ロールバック機能を実装
  - src/workspace/mod.rs を作成してモジュールをエクスポート
  - テストが通ることを確認（Green）
  - _要件: 1.1, 1.2, 1.3, 1.4, 1.5, 2.1, 2.2, 2.3, 2.4, 2.5, 3.1, 3.2, 3.3, 3.4, 4.1, 4.2, 4.3, 4.4, 4.5, 4.6, 4.7, 6.5_

- [ ]\* 5.3 WorkspaceGenerationService のプロパティテストを作成

  - **プロパティ 13: 指定 workspace のクリーンアップ**
  - **検証: 要件 4.1**

- [ ]\* 5.4 WorkspaceGenerationService のプロパティテストを作成

  - **プロパティ 14: 全体クリーンアップ**
  - **検証: 要件 4.2**

- [ ]\* 5.5 WorkspaceGenerationService のプロパティテストを作成

  - **プロパティ 16: 管理外ファイルの保護**
  - **検証: 要件 4.6**

- [ ]\* 5.6 WorkspaceGenerationService のプロパティテストを作成

  - **プロパティ 20: ロールバック動作**
  - **検証: 要件 6.5**

- [ ] 6. チェックポイント - すべてのテストが合格することを確認

  - すべてのテストが合格することを確認し、問題があればユーザーに質問する

- [ ] 7. clean コマンドの実装（TDD）
- [ ] 7.1 clean コマンドハンドラーのテストを作成

  - `wtx clean <workspace名>` のテスト
  - `wtx clean --all` のテスト
  - `wtx clean`（引数なし）のテスト
  - テストが失敗することを確認（Red）
  - _要件: 4.1, 4.2, 4.3_

- [ ] 7.2 clean コマンドハンドラーの実装

  - src/commands/clean.rs を作成
  - CleanArgs 構造体を定義（workspace 名、--all フラグ）
  - execute() 関数を実装
  - src/commands/mod.rs を更新
  - src/cli/mod.rs に Clean サブコマンドを追加
  - src/main.rs で clean コマンドを統合
  - テストが通ることを確認（Green）
  - _要件: 4.1, 4.2, 4.3_

- [ ]\* 7.3 clean コマンドの統合テストを作成

  - worktree 作成 → clean <workspace 名> → 検証フロー
  - worktree 作成 → clean --all → 検証フロー
  - _要件: 4.1, 4.2, 4.3, 4.4, 4.5, 4.6, 4.7_

- [ ] 8. エラーハンドリングの実装
- [ ] 8.1 エラーメッセージのテストを作成

  - 未登録リポジトリエラーのテスト
  - Git 操作エラーのテスト
  - ファイルシステムエラーのテスト
  - テストが失敗することを確認（Red）
  - _要件: 6.1, 6.2, 6.3_

- [ ] 8.2 エラーメッセージの実装

  - 各エラーケースに対する詳細なメッセージを実装
  - ユーザーフレンドリーなエラー表示を実装
  - テストが通ることを確認（Green）
  - _要件: 6.1, 6.2, 6.3_

- [ ] 9. 最終チェックポイント - すべてのテストが合格することを確認
  - すべてのテストが合格することを確認し、問題があればユーザーに質問する
