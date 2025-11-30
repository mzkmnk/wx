# 設計ドキュメント

## 概要

ワークスペース生成機能は、wtx CLI ツールのコアロジックを提供します。この機能は、登録済みの bare リポジトリから Git worktree を作成し、それらを含む VSCode/Kiro の workspace ファイル（.code-workspace）を自動生成します。

主要な責務:

- 登録済み bare リポジトリからの worktree 作成
- 複数 worktree を含む workspace ファイルの生成
- リポジトリのブランチ一覧取得（fetch 後）
- worktree と workspace のクリーンアップ
- エラーハンドリングとロールバック

## アーキテクチャ

### レイヤー構造

```
┌─────────────────────────────────────┐
│         CLI Layer (clap)            │  コマンドパース、引数検証
├─────────────────────────────────────┤
│      Command Handler Layer          │  clean コマンド
├─────────────────────────────────────┤
│    Workspace Generation Service     │  ビジネスロジック
├─────────────────────────────────────┤
│  Worktree Manager │ Workspace File  │  worktree操作 │ ファイル生成
├─────────────────────────────────────┤
│      Config Manager (既存)          │  登録リポジトリ参照
└─────────────────────────────────────┘
```

### 依存関係

- **既存の依存関係**: clap, git2, serde, serde_json, anyhow, thiserror, dirs
- **追加の依存関係**: なし（既存のクレートで実装可能）

## コンポーネントとインターフェース

### 1. WorkspaceGenerationService

worktree 作成と workspace 生成のビジネスロジックを担当します。

```rust
pub struct WorkspaceGenerationService {
    config_manager: ConfigManager,
    worktree_manager: WorktreeManager,
    workspace_file_manager: WorkspaceFileManager,
}

impl WorkspaceGenerationService {
    pub fn new() -> Result<Self>;

    /// Create worktrees and generate workspace file
    pub fn generate(
        &self,
        working_dir: &Path,
        selections: Vec<WorktreeSelection>,
        workspace_name: Option<String>,
    ) -> Result<GenerationResult>;

    /// Clean up worktrees and workspace file
    pub fn clean(
        &self,
        working_dir: &Path,
        target: CleanTarget,
    ) -> Result<CleanResult>;

    /// Get available branches for a repository
    pub fn get_branches(&self, repo_name: &str) -> Result<Vec<BranchInfo>>;
}
```

### 2. WorktreeManager

Git worktree 操作を担当します。

```rust
pub struct WorktreeManager;

impl WorktreeManager {
    pub fn new() -> Self;

    /// Create a worktree from a bare repository
    pub fn create_worktree(
        &self,
        bare_repo_path: &Path,
        target_path: &Path,
        branch: &str,
    ) -> Result<()>;

    /// Remove a worktree and prune references
    pub fn remove_worktree(
        &self,
        bare_repo_path: &Path,
        worktree_path: &Path,
    ) -> Result<()>;

    /// List all worktrees for a bare repository
    pub fn list_worktrees(&self, bare_repo_path: &Path) -> Result<Vec<WorktreeInfo>>;

    /// Fetch latest changes from remote
    pub fn fetch(&self, bare_repo_path: &Path) -> Result<()>;

    /// Get all remote branches
    pub fn get_remote_branches(&self, bare_repo_path: &Path) -> Result<Vec<String>>;

    /// Check if a branch exists
    pub fn branch_exists(&self, bare_repo_path: &Path, branch: &str) -> Result<bool>;
}
```

### 3. WorkspaceFileManager

workspace ファイルの生成と管理を担当します。

```rust
pub struct WorkspaceFileManager;

impl WorkspaceFileManager {
    pub fn new() -> Self;

    /// Generate a workspace file
    pub fn generate(
        &self,
        working_dir: &Path,
        workspace_name: &str,
        folders: Vec<String>,
    ) -> Result<PathBuf>;

    /// Read and validate a workspace file
    pub fn read(&self, path: &Path) -> Result<WorkspaceFile>;

    /// Delete a workspace file
    pub fn delete(&self, path: &Path) -> Result<()>;

    /// Check if workspace file exists
    pub fn exists(&self, working_dir: &Path, workspace_name: &str) -> bool;
}
```

## データモデル

### WorktreeSelection

worktree 作成時の選択情報を表します。

```rust
#[derive(Debug, Clone)]
pub struct WorktreeSelection {
    pub repo_name: String,
    pub branch: String,
}
```

### WorktreeInfo

worktree の情報を表します。

```rust
#[derive(Debug, Clone)]
pub struct WorktreeInfo {
    pub path: PathBuf,
    pub branch: String,
    pub repo_name: String,
}
```

### BranchInfo

ブランチの情報を表します。

```rust
#[derive(Debug, Clone)]
pub struct BranchInfo {
    pub name: String,
    pub is_default: bool,
}
```

### WorkspaceFile

workspace ファイルの構造を表します。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceFile {
    pub folders: Vec<WorkspaceFolder>,
    pub settings: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceFolder {
    pub path: String,
}

impl WorkspaceFile {
    pub fn new(folders: Vec<String>) -> Self {
        Self {
            folders: folders.into_iter().map(|p| WorkspaceFolder { path: p }).collect(),
            settings: serde_json::json!({}),
        }
    }
}
```

### CleanTarget

クリーンアップ対象を表します。

```rust
#[derive(Debug, Clone)]
pub enum CleanTarget {
    /// Clean specific workspace by name
    Workspace(String),
    /// Clean all wtx-managed worktrees and workspaces
    All,
}
```

### GenerationResult

生成結果を表します。

```rust
#[derive(Debug)]
pub struct GenerationResult {
    pub worktrees: Vec<PathBuf>,
    pub workspace_file: PathBuf,
}
```

### CleanResult

クリーンアップ結果を表します。

```rust
#[derive(Debug)]
pub struct CleanResult {
    pub removed_worktrees: Vec<PathBuf>,
    pub removed_workspace_files: Vec<PathBuf>,
    pub warnings: Vec<String>,
}
```

### エラー型

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WorkspaceError {
    #[error("Repository '{0}' is not registered")]
    RepositoryNotFound(String),

    #[error("Worktree already exists at '{0}'")]
    WorktreeAlreadyExists(String),

    #[error("Workspace file already exists: '{0}'")]
    WorkspaceFileAlreadyExists(String),

    #[error("Branch '{0}' not found in repository '{1}'")]
    BranchNotFound(String, String),

    #[error("Git operation failed: {0}")]
    GitError(#[from] git2::Error),

    #[error("IO operation failed: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Rollback failed after error: {original_error}, rollback error: {rollback_error}")]
    RollbackFailed {
        original_error: String,
        rollback_error: String,
    },
}
```

## 正確性プロパティ

_プロパティとは、システムのすべての有効な実行において真であるべき特性または動作のことです。本質的には、システムが何をすべきかについての形式的な記述です。プロパティは、人間が読める仕様と機械で検証可能な正確性保証との橋渡しとなります。_

### プロパティ 1: worktree 作成後のディレクトリ存在

*任意の*有効なリポジトリ名とブランチ名に対して、worktree 作成が成功した場合、作業ディレクトリ内に `<リポジトリ名>/` ディレクトリが存在しなければならない
**検証: 要件 1.1**

### プロパティ 2: worktree の親リポジトリ検証

*任意の*作成された worktree に対して、その親リポジトリは登録済み bare リポジトリ（~/.wtx/<name>.git）でなければならない
**検証: 要件 1.2**

### プロパティ 3: 既存ブランチの checkout

*任意の*リモートに存在するブランチを指定した worktree 作成に対して、作成された worktree はそのブランチを checkout した状態でなければならない
**検証: 要件 1.3**

### プロパティ 4: 新規ブランチの作成

*任意の*リモートに存在しないブランチを指定した worktree 作成に対して、新しいブランチが作成され、worktree はそのブランチを checkout した状態でなければならない
**検証: 要件 1.4**

### プロパティ 5: 重複 worktree の拒否

*任意の*既に worktree が存在するパスに対して、再度 worktree を作成しようとした場合、エラーが返され、既存の worktree は変更されないままでなければならない
**検証: 要件 1.5**

### プロパティ 6: workspace ファイル生成

*任意の*workspace 生成要求に対して、成功した場合、作業ディレクトリに `.code-workspace` ファイルが存在しなければならない
**検証: 要件 2.1**

### プロパティ 7: folders 配列の完全性

*任意の*worktree リストを含む workspace 生成に対して、生成された workspace ファイルの `folders` 配列はすべての worktree への相対パスを含まなければならない
**検証: 要件 2.2, 5.2**

### プロパティ 8: workspace ファイル名の正確性

*任意の*workspace 名が指定された workspace 生成に対して、生成されるファイル名は `<workspace名>.code-workspace` でなければならない
**検証: 要件 2.3**

### プロパティ 9: 重複 workspace ファイルの拒否

*任意の*既に同名の workspace ファイルが存在する場合、再度生成しようとした場合、エラーが返され、既存のファイルは変更されないままでなければならない
**検証: 要件 2.5**

### プロパティ 10: ブランチ一覧の完全性

*任意の*登録済みリポジトリに対して、ブランチ一覧取得はすべてのリモートブランチを返さなければならない
**検証: 要件 3.1**

### プロパティ 11: fetch の実行

*任意の*ブランチ一覧取得に対して、取得前に bare リポジトリの fetch が実行されなければならない
**検証: 要件 3.2**

### プロパティ 12: デフォルトブランチの優先

*任意の*ブランチ一覧に対して、main または master ブランチが存在する場合、それが先頭に配置されなければならない
**検証: 要件 3.3**

### プロパティ 13: 指定 workspace のクリーンアップ

_任意の_`wtx clean <workspace名>` 実行に対して、指定された workspace に関連する worktree と workspace ファイルのみが削除されなければならない
**検証: 要件 4.1**

### プロパティ 14: 全体クリーンアップ

_任意の_`wtx clean --all` 実行に対して、作業ディレクトリ内のすべての wtx 管理 worktree と workspace ファイルが削除されなければならない
**検証: 要件 4.2**

### プロパティ 15: worktree prune の実行

*任意の*worktree 削除に対して、Git の worktree prune が実行され、親リポジトリの参照が更新されなければならない
**検証: 要件 4.4**

### プロパティ 16: 管理外ファイルの保護

*任意の*クリーンアップ操作に対して、wtx 管理外のファイルは削除されてはならない
**検証: 要件 4.6**

### プロパティ 17: JSON 形式の有効性

*任意の*workspace ファイル生成に対して、出力は有効な JSON 形式でなければならない
**検証: 要件 5.1**

### プロパティ 18: settings オブジェクトの存在

*任意の*workspace ファイル生成に対して、出力には空の `settings` オブジェクトが含まれなければならない
**検証: 要件 5.3**

### プロパティ 19: JSON 構造検証

*任意の*workspace ファイル読み込みに対して、JSON 構造が有効な場合のみ成功を返さなければならない
**検証: 要件 5.4**

### プロパティ 20: ロールバック動作

*任意の*複数 worktree 作成中にエラーが発生した場合、作成済みの worktree はすべて削除されなければならない
**検証: 要件 6.5**

## エラーハンドリング

### エラー戦略

1. **早期検証**: 操作を実行する前に入力を検証
2. **詳細なエラーメッセージ**: ユーザーが問題を理解し修正できるように
3. **ロールバック**: 複数 worktree 作成中のエラー時は作成済みを削除
4. **エラー伝播**: `Result<T, E>`を使用して明示的にエラーを伝播

### エラーシナリオと対応

| シナリオ               | 検出方法             | 対応                         |
| ---------------------- | -------------------- | ---------------------------- |
| 未登録リポジトリ       | config.json 内検索   | エラーメッセージで登録を促す |
| 重複 worktree          | パス存在チェック     | 既存パスを表示してエラー     |
| 重複 workspace         | ファイル存在チェック | 既存ファイルを表示してエラー |
| Git 操作失敗           | git2 エラーキャッチ  | Git エラー出力を含めて表示   |
| ファイルシステムエラー | IO エラーキャッチ    | 操作とパスを含めて表示       |
| 部分的作成後エラー     | 作成済みリスト追跡   | ロールバック実行             |

### ロールバックフロー

```
worktree作成開始
  ↓
作成済みリストを初期化
  ↓
各worktreeを順次作成
  ↓
成功? ─Yes→ 作成済みリストに追加 → 次のworktreeへ
  ↓
  No
  ↓
作成済みworktreeをすべて削除
  ↓
エラー表示 → 終了
```

## テスト戦略

### ユニットテスト

各コンポーネントの個別機能をテストします:

- **WorktreeManager**:

  - worktree 作成（既存/新規ブランチ）
  - worktree 削除と prune
  - ブランチ一覧取得
  - fetch 実行

- **WorkspaceFileManager**:

  - workspace ファイル生成
  - JSON 形式検証
  - ファイル読み込み

- **WorkspaceGenerationService**:
  - 複数 worktree + workspace 生成
  - クリーンアップ（指定/全体）
  - ロールバック動作

### プロパティベーステスト

**使用ライブラリ**: `proptest` (Rust 標準のプロパティベーステストライブラリ)

**設定**: 各プロパティテストは最低 100 回の反復を実行

**タグ付け規則**: 各プロパティベーステストには、設計ドキュメントの正確性プロパティを参照するコメントを付与

- フォーマット: `// Feature: workspace-generation, Property {number}: {property_text}`

**テスト対象プロパティ**:

1. **プロパティ 1-5**: worktree 作成の正確性

   - ランダムなリポジトリ名とブランチ名を生成
   - 作成後の状態を検証
   - 重複作成の拒否を検証

2. **プロパティ 6-9**: workspace ファイル生成の正確性

   - ランダムな worktree リストを生成
   - 生成されたファイルの内容を検証

3. **プロパティ 10-12**: ブランチ一覧取得の正確性

   - fetch 実行の検証
   - ソート順の検証

4. **プロパティ 13-16**: クリーンアップの正確性

   - 指定/全体削除の動作を検証
   - 管理外ファイルの保護を検証

5. **プロパティ 17-19**: JSON 処理の正確性

   - ラウンドトリップテスト
   - 構造検証

6. **プロパティ 20**: ロールバックの正確性
   - 途中エラー時の状態を検証

### 統合テスト

エンドツーエンドのシナリオをテストします:

- 完全な worktree 作成 → workspace 生成 → クリーンアップフロー
- エラーからの復旧シナリオ
- 複数リポジトリの同時管理

### テスト環境

- 各テストは独立した一時ディレクトリで実行
- テスト後のクリーンアップを保証
- モック Git リポジトリを使用（実際のネットワークアクセスなし）

## 実装の詳細

### ディレクトリ構造（追加分）

```
src/
├── commands/
│   └── clean.rs             # cleanコマンド実装
├── workspace/
│   ├── mod.rs
│   ├── service.rs           # WorkspaceGenerationService
│   ├── worktree.rs          # WorktreeManager
│   └── file.rs              # WorkspaceFileManager
└── models/
    └── workspace.rs         # workspace関連のデータモデル
```

### workspace ファイル形式

```json
{
  "folders": [{ "path": "frontend" }, { "path": "backend" }],
  "settings": {}
}
```

### wtx 管理ファイルの識別

wtx が管理する worktree と workspace を識別するため、以下の方法を使用:

1. **worktree**: Git の worktree 情報から親リポジトリが `~/.wtx/` 配下かを確認
2. **workspace ファイル**: `.code-workspace` 拡張子を持つファイル

### パフォーマンス考慮事項

- fetch は必要な場合のみ実行（ブランチ一覧取得時）
- 複数 worktree の作成は順次実行（並列化は将来的な拡張として検討）
- 大きなリポジトリの worktree 作成は時間がかかる可能性がある

### セキュリティ考慮事項

- ファイルパスのサニタイゼーション（パストラバーサル攻撃防止）
- 作業ディレクトリ外への書き込み防止
- 管理外ファイルの保護
