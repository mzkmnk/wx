# 設計ドキュメント

## 概要

リポジトリ登録機能は、wtx CLI ツールの基盤となるコンポーネントです。この機能は、Git リポジトリを bare クローンとして一元管理し、後続の worktree 作成機能のためのソースリポジトリを提供します。

主要な責務:

- Git リポジトリの登録（bare クローン作成）
- 登録済みリポジトリの一覧表示
- リポジトリの登録解除（bare クローンとメタデータの削除）
- 設定ファイル（config.json）の管理
- エラーハンドリングとユーザーフィードバック

## アーキテクチャ

### レイヤー構造

```
┌─────────────────────────────────────┐
│         CLI Layer (clap)            │  コマンドパース、引数検証
├─────────────────────────────────────┤
│      Command Handler Layer          │  register/list/unregister
├─────────────────────────────────────┤
│      Repository Service Layer       │  ビジネスロジック
├─────────────────────────────────────┤
│   Config Manager │ Git Operations   │  永続化 │ Git操作
└─────────────────────────────────────┘
```

### 依存関係

- **clap**: CLI パーサー（v4 系）
- **git2**: libgit2 バインディング（Git 操作）
- **serde**: JSON シリアライズ/デシリアライズ
- **serde_json**: JSON 処理
- **anyhow**: エラーハンドリング
- **thiserror**: カスタムエラー型定義
- **dirs**: ホームディレクトリパス解決（`~/.wtx` 用）

## コンポーネントとインターフェース

### 1. CLI Layer

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "wtx")]
#[command(about = "Git worktree and workspace manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Register a Git repository
    Register {
        /// Git repository URL (SSH or HTTPS)
        url: String,
    },
    /// List all registered repositories
    List,
    /// Unregister a repository
    Unregister {
        /// Repository name
        name: String,
    },
}
```

### 2. Repository Service

リポジトリ管理のビジネスロジックを担当します。

```rust
pub struct RepositoryService {
    config_manager: ConfigManager,
    git_ops: GitOperations,
}

impl RepositoryService {
    pub fn new() -> Result<Self>;

    /// Register a new repository
    pub fn register(&mut self, url: &str) -> Result<()>;

    /// List all registered repositories
    pub fn list(&self) -> Result<Vec<Repository>>;

    /// Unregister a repository
    pub fn unregister(&mut self, name: &str) -> Result<()>;
}
```

### 3. Config Manager

config.json の読み書きとバックアップ管理を担当します。

```rust
pub struct ConfigManager {
    config_path: PathBuf,
    backup_path: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Result<Self>;

    /// Load configuration from disk
    pub fn load(&self) -> Result<Config>;

    /// Save configuration to disk
    pub fn save(&self, config: &Config) -> Result<()>;

    /// Create backup before risky operations
    pub fn create_backup(&self) -> Result<()>;

    /// Restore from backup
    pub fn restore_backup(&self) -> Result<()>;

    /// Delete backup after successful operation
    pub fn delete_backup(&self) -> Result<()>;
}
```

### 4. Git Operations

Git 操作（bare クローン、検証）を担当します。

```rust
pub struct GitOperations;

impl GitOperations {
    pub fn new() -> Self;

    /// Create a bare clone of a repository
    pub fn bare_clone(&self, url: &str, target_path: &Path) -> Result<()>;

    /// Validate Git URL format
    pub fn validate_url(&self, url: &str) -> Result<()>;

    /// Extract repository name from URL
    pub fn extract_repo_name(&self, url: &str) -> Result<String>;
}
```

## データモデル

### Config

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub repositories: Vec<Repository>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            repositories: Vec::new(),
        }
    }

    pub fn add_repository(&mut self, repo: Repository) -> Result<()> {
        // 重複チェック
        if self.repositories.iter().any(|r| r.name == repo.name) {
            return Err(anyhow!("Repository '{}' is already registered", repo.name));
        }
        self.repositories.push(repo);
        Ok(())
    }

    pub fn remove_repository(&mut self, name: &str) -> Result<Repository> {
        let index = self.repositories
            .iter()
            .position(|r| r.name == name)
            .ok_or_else(|| anyhow!("Repository '{}' not found", name))?;
        Ok(self.repositories.remove(index))
    }

    pub fn find_repository(&self, name: &str) -> Option<&Repository> {
        self.repositories.iter().find(|r| r.name == name)
    }
}
```

### Repository

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub name: String,
    pub remote: String,
    pub local_path: String,
}

impl Repository {
    pub fn new(name: String, remote: String, local_path: String) -> Self {
        Self {
            name,
            remote,
            local_path,
        }
    }
}
```

### エラー型

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RegistrationError {
    #[error("Invalid Git URL format: {0}")]
    InvalidUrl(String),

    #[error("Repository '{0}' is already registered")]
    AlreadyRegistered(String),

    #[error("Repository '{0}' not found")]
    NotFound(String),

    #[error("Git operation failed: {0}")]
    GitError(#[from] git2::Error),

    #[error("IO operation failed: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON parsing failed: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}
```

## 正確性プロパティ

_プロパティとは、システムのすべての有効な実行において真であるべき特性または動作のことです。本質的には、システムが何をすべきかについての形式的な記述です。プロパティは、人間が読める仕様と機械で検証可能な正確性保証との橋渡しとなります。_

### プロパティ 1: 登録後の bare リポジトリ存在

*任意の*有効な Git URL に対して、register コマンドが成功した場合、~/.wtx/<name>.git に bare リポジトリが存在しなければならない
**検証: 要件 1.1**

### プロパティ 2: 登録後の config.json エントリ追加

*任意の*有効な Git URL に対して、register コマンドが成功した場合、config.json に該当するリポジトリのエントリ（name、remote、local_path）が追加されなければならない
**検証: 要件 1.2**

### プロパティ 3: 重複登録の拒否

*任意の*リポジトリに対して、同じ URL または同じ名前で 2 回目の登録を試みた場合、システムはエラーを返し、config.json は変更されないままでなければならない
**検証: 要件 1.3, 4.5**

### プロパティ 4: list 操作の完全性

*任意の*登録済みリポジトリのセットに対して、list コマンドは登録されているすべてのリポジトリを返さなければならない
**検証: 要件 2.1**

### プロパティ 5: リポジトリ情報の完全性

*任意の*リポジトリに対して、list コマンドの出力には、リポジトリ名、リモート URL、ローカルパスのすべてが含まれていなければならない
**検証: 要件 2.2**

### プロパティ 6: unregister 後のディレクトリ削除

*任意の*登録済みリポジトリに対して、unregister コマンドが成功した場合、ローカルパスの bare リポジトリディレクトリが削除されなければならない
**検証: 要件 3.1**

### プロパティ 7: unregister 後の config.json エントリ削除

*任意の*登録済みリポジトリに対して、ディレクトリ削除が成功した場合のみ、config.json から該当するエントリが削除されなければならない
**検証: 要件 3.2**

### プロパティ 8: 未登録リポジトリの unregister 拒否

*任意の*未登録のリポジトリ名に対して、unregister コマンドはエラーを返し、config.json は変更されないままでなければならない
**検証: 要件 3.3**

### プロパティ 9: URL 検証の正確性

*任意の*URL に対して、validate_url 関数は、SSH または HTTPS 形式の場合のみ成功を返さなければならない
**検証: 要件 4.1**

### プロパティ 10: 無効な URL 拒否

*任意の*無効な URL に対して、register コマンドはエラーを返し、bare クローンは実行されず、config.json は変更されないままでなければならない
**検証: 要件 4.2**

### プロパティ 11: リポジトリ名抽出の正確性

*任意の*Git URL に対して、extract_repo_name 関数は、URL の最後のパスコンポーネントから.git 拡張子（存在する場合）を除いた文字列を返さなければならない
**検証: 要件 4.4**

### プロパティ 12: バックアップ作成

*任意の*リポジトリ登録操作に対して、bare クローンを開始する前に、config.backup.json が作成されなければならない
**検証: 要件 6.1**

### プロパティ 13: 成功時のバックアップ削除

*任意の*リポジトリ登録操作に対して、bare クローンが成功した場合、config.backup.json は削除されなければならない
**検証: 要件 6.2**

### プロパティ 14: JSON 構造検証

*任意の*config.json ファイルに対して、load 関数は、JSON 構造が有効な場合のみ成功を返さなければならない
**検証: 要件 6.4**

### プロパティ 15: 無効な JSON 拒否

*任意の*無効な JSON ファイルに対して、load 関数はエラーを返し、ファイルは変更されないままでなければならない
**検証: 要件 6.5**

## エラーハンドリング

### エラー戦略

1. **早期検証**: 操作を実行する前に入力を検証
2. **詳細なエラーメッセージ**: ユーザーが問題を理解し修正できるように
3. **ロールバック**: 失敗時にはバックアップから復元
4. **エラー伝播**: `Result<T, E>`を使用して明示的にエラーを伝播

### エラーシナリオと対応

| シナリオ               | 検出方法             | 対応                               |
| ---------------------- | -------------------- | ---------------------------------- |
| 無効な URL             | 正規表現検証         | エラーメッセージで正しい形式を説明 |
| 重複登録               | config.json 内検索   | 既存エントリを表示してエラー       |
| Git 操作失敗           | git2 エラーキャッチ  | Git エラー出力を含めて表示         |
| ファイルシステムエラー | IO エラーキャッチ    | 操作とパスを含めて表示             |
| JSON 解析エラー        | serde エラーキャッチ | 行番号と問題箇所を表示             |
| ディスク容量不足       | IO エラー分析        | ディスク容量確認を促す             |
| ネットワークエラー     | git2 エラー分析      | 接続確認を促す                     |

### バックアップと復元フロー

```
register開始
  ↓
config.backup.json作成
  ↓
bareクローン実行
  ↓
成功? ─Yes→ config.json更新 → backup削除 → 完了
  ↓
  No
  ↓
backup復元 → エラー表示 → 終了
```

## テスト戦略

### ユニットテスト

各コンポーネントの個別機能をテストします:

- **ConfigManager**:

  - 空の config ファイル作成
  - 既存 config の読み込み
  - バックアップ作成と復元
  - 不正な JSON 処理

- **GitOperations**:

  - URL 検証（有効/無効なパターン）
  - リポジトリ名抽出（様々な URL 形式）

- **RepositoryService**:
  - リポジトリ追加/削除ロジック
  - 重複チェック
  - エラーハンドリング

### プロパティベーステスト

**使用ライブラリ**: `proptest` (Rust 標準のプロパティベーステストライブラリ)

**設定**: 各プロパティテストは最低 100 回の反復を実行

**タグ付け規則**: 各プロパティベーステストには、設計ドキュメントの正確性プロパティを参照するコメントを付与

- フォーマット: `// Feature: repo-registration, Property {number}: {property_text}`

**テスト対象プロパティ**:

1. **プロパティ 1-3**: 登録操作の正確性

   - ランダムな Git URL を生成
   - 登録後の状態を検証
   - 重複登録の拒否を検証

2. **プロパティ 4-5**: 一覧表示の正確性

   - ランダムな数のリポジトリを登録
   - list 出力の完全性を検証

3. **プロパティ 6-8**: 登録解除の正確性

   - ランダムなリポジトリを登録・解除
   - 状態の変化を検証

4. **プロパティ 9-11**: URL 処理の正確性

   - ランダムな URL（有効/無効）を生成
   - 検証と名前抽出の正確性を検証

5. **プロパティ 12-15**: 設定管理の正確性
   - ランダムな JSON（有効/無効）を生成
   - バックアップと復元の動作を検証

### 統合テスト

エンドツーエンドのシナリオをテストします:

- 完全な登録 → 一覧表示 → 登録解除フロー
- エラーからの復旧シナリオ
- 複数リポジトリの同時管理

### テスト環境

- 各テストは独立した一時ディレクトリで実行
- テスト後のクリーンアップを保証
- モック Git リポジトリを使用（実際のネットワークアクセスなし）

## 実装の詳細

### ディレクトリ構造

```
src/
├── main.rs              # CLIエントリーポイント
├── cli.rs               # CLIパーサー定義
├── commands/
│   ├── mod.rs
│   ├── register.rs      # registerコマンド実装
│   ├── list.rs          # listコマンド実装
│   └── unregister.rs    # unregisterコマンド実装
├── service/
│   ├── mod.rs
│   └── repository.rs    # RepositoryService
├── config/
│   ├── mod.rs
│   └── manager.rs       # ConfigManager
├── git/
│   ├── mod.rs
│   └── operations.rs    # GitOperations
├── models/
│   ├── mod.rs
│   ├── config.rs        # Config, Repository
│   └── error.rs         # エラー型定義
└── utils/
    ├── mod.rs
    └── path.rs          # パス操作ユーティリティ
```

### 設定ファイルパス

- **Config**: `~/.wtx/config.json`
- **Backup**: `~/.wtx/config.backup.json`
- **Bare repositories**: `~/.wtx/<repo-name>.git/`

### URL 検証パターン

```rust
// SSH形式: git@github.com:org/repo.git または git@github.com:org/team/repo.git
// ネストしたパス、アンダースコア、.git拡張子なしにも対応
const SSH_PATTERN: &str = r"^git@[\w\.\-]+:[\w\.\-_/]+?(?:\.git)?$";

// HTTPS形式: https://github.com/org/repo.git または https://github.com/org/repo
// ネストしたパス、アンダースコア、.git拡張子なしにも対応
const HTTPS_PATTERN: &str = r"^https://[\w\.\-]+/[\w\.\-_/]+?(?:\.git)?$";
```

**対応する URL 形式の例:**

- `git@github.com:org/repo.git` (SSH、標準)
- `git@github.com:org/repo` (SSH、.git なし)
- `git@github.com:org/team/repo.git` (SSH、ネストパス)
- `https://github.com/org/repo.git` (HTTPS、標準)
- `https://github.com/org/repo` (HTTPS、.git なし)
- `https://github.com/org/team/repo.git` (HTTPS、ネストパス)
- `git@github.com:my_org/my_repo.git` (アンダースコア含む)

### パフォーマンス考慮事項

- bare クローンは大きなリポジトリで時間がかかる可能性がある
- プログレス表示を実装（git2 のコールバック機能を使用）
- 並列操作は現時点では不要（将来的な拡張として検討）

### セキュリティ考慮事項

- SSH 鍵と HTTPS 認証情報は git2 が自動的に処理
- ファイルパスのサニタイゼーション（パストラバーサル攻撃防止）
- config.json のパーミッション設定（600）
