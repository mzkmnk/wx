# 設計ドキュメント

## 概要

Clone ベースワークスペース機能は、従来の bare リポジトリ + worktree 方式から、シンプルな逐次 clone 方式に変更します。これにより、状態管理を簡素化し、VSCode/Kiro と同等の Git 操作を実現します。

### 主な変更点

| 項目                 | 旧方式                    | 新方式             |
| -------------------- | ------------------------- | ------------------ |
| リポジトリ保存       | bare clone (~/.wx/\*.git) | URL のみ保存       |
| 作業ディレクトリ作成 | git worktree              | git clone          |
| ブランチ取得         | bare リポジトリから       | git ls-remote      |
| 同一ブランチ並列作業 | 不可（git 制約）          | 可能（独立 clone） |

## アーキテクチャ

```
┌───────────────────────────────────────────────────────────────────────────────┐
│                                 CLI Layer                                    │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌───────┐ ┌──────────┐ ┌──────────┐ │
│  │register │ │  list   │ │unregister│ │   wx    │ │ clean │ │list-ws   │ │ rm-ws    │ │
│  └────┬────┘ └────┬────┘ └────┬────┘ └────┬────┘ └───┬───┘ └────┬─────┘ └────┬─────┘ │
└───────┼──────────┼──────────┼──────────┼──────────┼──────────┼──────────┼───────────┘
        │          │          │          │          │
┌───────┴──────────┴──────────┴──────────┴──────────┴───────┐
│                      Service Layer                         │
│  ┌────────────────────┐  ┌─────────────────────────────┐  │
│  │ RepositoryService  │  │    WorkspaceService         │  │
│  │ - register()       │  │ - create_workspace()        │  │
│  │ - list()           │  │ - clone_repository()        │  │
│  │ - unregister()     │  │ - get_remote_branches()     │  │
│  │ - find_by_name()   │  │ - generate_workspace_file() │  │
│  └─────────┬──────────┘  │ - clean()                   │  │
│            │             └──────────────┬──────────────┘  │
└────────────┼────────────────────────────┼─────────────────┘
             │                            │
┌────────────┴────────────────────────────┴─────────────────┐
│                   Infrastructure Layer                     │
│  ┌─────────────────┐  ┌─────────────────────────────────┐ │
│  │  ConfigManager  │  │        GitCloneManager          │ │
│  │ - load()        │  │ - clone()                       │ │
│  │ - save()        │  │ - checkout_branch()             │ │
│  │ - validate()    │  │ - list_remote_branches()        │ │
│  └─────────────────┘  │ - set_upstream()                │ │
│                       └─────────────────────────────────┘ │
└───────────────────────────────────────────────────────────┘
```

## コンポーネントとインターフェース

### ConfigManager（既存を修正）

```rust
/// 設定ファイルの読み書きを管理
pub trait ConfigManager {
    fn load(&self) -> Result<Config, WxError>;
    fn save(&self, config: &Config) -> Result<(), WxError>;
    fn ensure_config_dir(&self) -> Result<PathBuf, WxError>;
}
```

### GitCloneManager（新規）

```rust
/// Git clone 操作を管理
pub trait GitCloneManager {
    /// リモートリポジトリを clone
    fn clone(&self, url: &str, target_path: &Path) -> Result<(), WxError>;

    /// 指定ブランチを checkout（存在しない場合は新規作成）
    fn checkout_branch(&self, repo_path: &Path, branch: &str) -> Result<BranchStatus, WxError>;

    /// リモートブランチ一覧を取得（git ls-remote）
    fn list_remote_branches(&self, url: &str) -> Result<Vec<String>, WxError>;

    /// upstream を設定
    fn set_upstream(&self, repo_path: &Path, branch: &str) -> Result<(), WxError>;
}

pub enum BranchStatus {
    CheckedOutExisting,  // リモートブランチを checkout
    CreatedNew,          // 新規ブランチを作成
}
```

### RepositoryService（既存を修正）

```rust
pub trait RepositoryService {
    fn register(&self, url: &str) -> Result<Repository, WxError>;
    fn list(&self) -> Result<Vec<Repository>, WxError>;
    fn unregister(&self, name: &str) -> Result<(), WxError>;
    fn find_by_name(&self, name: &str) -> Result<Option<Repository>, WxError>;
}
```

### WorkspaceService（既存を修正）

```rust
pub trait WorkspaceService {
    /// 複数リポジトリを clone して workspace を作成
    fn create_workspace(
        &self,
        repos: Vec<(String, String)>,  // (repo_name, branch)
        workspace_name: Option<String>,
    ) -> Result<WorkspaceResult, WxError>;

    /// 単一リポジトリを clone
    fn clone_repository(
        &self,
        repo_name: &str,
        branch: &str,
        target_dir: &Path,
    ) -> Result<PathBuf, WxError>;

    /// リモートブランチ一覧を取得
    fn get_remote_branches(&self, repo_name: &str) -> Result<Vec<String>, WxError>;

    /// workspace ファイルを生成
    fn generate_workspace_file(
        &self,
        folders: &[PathBuf],
        workspace_name: &str,
    ) -> Result<PathBuf, WxError>;

    /// 作成済み workspace 一覧を取得
    fn list_workspaces(&self) -> Result<Vec<ManagedWorkspace>, WxError>;

    /// workspace を削除（clone ディレクトリも削除）
    /// id は UUID または選択インデックス
    fn remove_workspace(&self, id: &str) -> Result<RemoveWorkspaceResult, WxError>;

    /// 作業ディレクトリをクリーンアップ（現在のディレクトリのみ）
    fn clean(&self, target_dir: &Path) -> Result<CleanResult, WxError>;
}
```

## データモデル

### Config（修正）

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub repositories: Vec<Repository>,
    pub workspaces: Vec<ManagedWorkspace>,  // 作成済み workspace の追跡
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub name: String,
    pub url: String,  // Git リモート URL（SSH または HTTPS）
}

/// wx で作成した workspace を追跡するための構造体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedWorkspace {
    pub id: String,                    // UUID（一意識別子）
    pub name: String,                  // workspace 名（表示用）
    pub workspace_file: PathBuf,       // .code-workspace ファイルの絶対パス
    pub repos: Vec<ManagedWorkspaceRepo>,  // 含まれるリポジトリ
    pub created_at: DateTime<Utc>,     // 作成日時
    pub updated_at: DateTime<Utc>,     // 更新日時
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedWorkspaceRepo {
    pub name: String,      // リポジトリ名
    pub branch: String,    // ブランチ名
    pub path: PathBuf,     // clone 先の絶対パス
}
```

### WorkspaceFile（既存）

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceFile {
    pub folders: Vec<WorkspaceFolder>,
    pub settings: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceFolder {
    pub path: String,
}
```

### 新規型

```rust
pub struct WorkspaceResult {
    pub workspace_file: PathBuf,
    pub cloned_repos: Vec<ClonedRepo>,
}

pub struct ClonedRepo {
    pub name: String,
    pub path: PathBuf,
    pub branch: String,
    pub status: BranchStatus,
}

pub struct CleanResult {
    pub deleted_dirs: Vec<PathBuf>,
    pub deleted_files: Vec<PathBuf>,
}

pub struct RemoveWorkspaceResult {
    pub workspace_name: String,
    pub deleted_workspace_file: PathBuf,
    pub deleted_repos: Vec<PathBuf>,
}
```

## 正当性プロパティ

_A property is a characteristic or behavior that should hold true across all valid executions of a system-essentially, a formal statement about what the system should do.
Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees._

### Property 1: Register/Unregister ラウンドトリップ

_For any_ 有効な Git URL に対して、register した後に unregister すると、config.json は元の状態に戻る

**Validates: Requirements 1.1, 3.1**

### Property 2: Register の冪等性拒否

_For any_ 登録済みリポジトリに対して、同じ URL で再度 register しようとするとエラーが返される

**Validates: Requirements 1.2**

### Property 3: List の完全性

_For any_ config.json に登録されたリポジトリリストに対して、list コマンドの出力にはすべてのリポジトリの名前と URL が含まれる

**Validates: Requirements 2.1, 2.2**

### Property 4: URL 検証の正確性

_For any_ 文字列に対して、SSH 形式（git@host:path）または HTTPS 形式（https://host/path）に一致する場合のみ有効と判定される

**Validates: Requirements 4.1, 4.2**

### Property 5: リポジトリ名抽出の一貫性

_For any_ 有効な Git URL に対して、抽出されるリポジトリ名は URL の最後のパスコンポーネントから .git を除いたものである

**Validates: Requirements 4.3**

### Property 6: 名前競合の検出

_For any_ 既存の登録済みリポジトリと同じ名前になる URL を登録しようとするとエラーが返される

**Validates: Requirements 4.4**

### Property 7: ディレクトリ存在時のエラー

_For any_ 既にディレクトリまたはファイルが存在するパスに対して clone を実行しようとするとエラーが返され、既存のファイルシステムエントリ（ディレクトリ、ファイル、シンボリックリンクなど）は変更されない

**Validates: Requirements 5.4**

### Property 8: Workspace ファイルの完全性

_For any_ フォルダリストに対して、生成された workspace ファイルの folders 配列にはすべてのフォルダへの相対パスが含まれる

**Validates: Requirements 6.2, 9.2**

### Property 9: Workspace ファイル名の正確性

_For any_ workspace 名に対して、生成されるファイル名は `<workspace名>.code-workspace` である

**Validates: Requirements 6.3**

### Property 10: Workspace ファイル存在時のエラー

_For any_ 既に同名の workspace ファイルが存在する場合、生成を実行しようとするとエラーが返され、既存ファイルは変更されない

**Validates: Requirements 6.5**

### Property 11: ブランチソートの正確性

_For any_ ブランチリストに対して、ソート後は main または master が先頭に配置される（存在する場合）

**Validates: Requirements 7.2**

### Property 12: Clean の選択性

_For any_ 作業ディレクトリに対して、clean 実行後は wx 管理の clone と workspace ファイルのみが削除され、それ以外のファイルは残る

**Validates: Requirements 8.1, 8.2**

### Property 13: Workspace JSON の有効性

_For any_ 入力に対して、生成された workspace ファイルは有効な JSON であり、folders 配列と settings オブジェクトを含む

**Validates: Requirements 9.1, 9.3**

### Property 14: Workspace メタデータの完全性

_For any_ 作成された workspace に対して、config.json には UUID、名前、パス、リポジトリ情報、created_at、updated_at が記録される

**Validates: Requirements 12.1, 12.2, 12.3, 12.4**

### Property 15: Workspace 削除の完全性

_For any_ 削除対象の workspace に対して、rm-workspace 実行後は workspace ファイル、clone ディレクトリ、config.json のエントリがすべて削除される

**Validates: Requirements 11.1, 11.2**

### Property 16: Workspace ID の一意性

_For any_ 作成された workspace に対して、その UUID は既存のすべての workspace の UUID と異なる

**Validates: Requirements 12.1**

## エラーハンドリング

### WxError（既存を拡張）

```rust
#[derive(Debug, thiserror::Error)]
pub enum WxError {
    #[error("Invalid Git URL: {0}")]
    InvalidUrl(String),

    #[error("Repository already registered: {0}")]
    AlreadyRegistered(String),

    #[error("Repository not found: {0}")]
    RepositoryNotFound(String),

    #[error("Name conflict: {0}")]
    NameConflict(String),

    #[error("Directory already exists: {0}")]
    DirectoryExists(String),

    #[error("Workspace file already exists: {0}")]
    WorkspaceFileExists(String),

    #[error("Git operation failed: {0}")]
    GitError(#[from] git2::Error),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("File system error: {0}")]
    FileSystemError(#[from] std::io::Error),

    #[error("Config parse error: {0}")]
    ConfigParseError(#[from] serde_json::Error),

    #[error("Invalid path: {0}")]
    InvalidPath(String),
}
```

## テスト戦略

### プロパティベーステスト

- **ライブラリ**: proptest
- **最小実行回数**: 100 回

テスト対象:

- URL 検証（Property 4）
- リポジトリ名抽出（Property 5）
- Workspace ファイル生成（Property 8, 9, 13）
- ブランチソート（Property 11）
- Register/Unregister ラウンドトリップ（Property 1）

### ユニットテスト

- ConfigManager の load/save
- GitCloneManager の各メソッド（モック使用）
- WorkspaceService のビジネスロジック

### 統合テスト

- 実際の Git リポジトリを使用した clone テスト
- ブランチ checkout のテスト
- 完全なワークフローテスト

### テストヘルパー

```rust
// テスト用の一時ディレクトリとモックリポジトリを作成
fn setup_test_environment() -> (TempDir, PathBuf);

// テスト用の Git リポジトリを作成
fn create_test_git_repo(path: &Path, name: &str) -> PathBuf;

// テスト用の config.json を作成
fn create_test_config(repos: Vec<Repository>) -> Config;
```
