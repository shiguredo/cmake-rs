---
name: shiguredo-cmake
description: shiguredo_cmake クレート (CMake 公式プリビルトバイナリを GitHub Releases からダウンロードしてキャッシュする Rust ライブラリ + CLI) を利用する側のリファレンス。build.rs から CMake をビルドする、CLI として cmake コマンドを使う、Config API でビルド設定をカスタマイズする、必要な実行環境を確認するときに使う。
---

# shiguredo_cmake クレート

CMake の公式プリビルトバイナリを Kitware 公式 GitHub Releases からダウンロードしてキャッシュする Rust クレート。`cmake` クレートのドロップイン代替として `build.rs` から使うか、`cmake` コマンドとして使う。

## 概要

- 初回呼び出し時にプリビルトバイナリを自動ダウンロード
- `$HOME/.cache/shiguredo_cmake/` 配下にキャッシュ (2 回目以降はダウンロード不要)
- SHA256 チェックサム検証付き
- [`cmake`](https://crates.io/crates/cmake) クレートの `Config` API をそのまま利用可能
- 依存最小化: `curl`、`tar`、SHA256 検証コマンドなどシステムコマンドのみで動作
- macOS / Linux / Windows (x86_64, aarch64) 対応

## 必要な環境

- Rust 1.88 以上
- `curl` (ダウンロード)
- `tar` (展開、`.tar.gz` と `.zip` 両対応、Windows 10+ は内蔵 tar で OK)
- SHA256 検証コマンド
  - Linux: `sha256sum`
  - macOS: `shasum`
  - Windows: `certutil`

## API リファレンス

### 関数

| シグネチャ | 説明 |
| :-- | :-- |
| `cmake_version() -> &'static str` | クレートが取得する CMake のバージョン文字列 |
| `cmake_dir() -> Result<PathBuf, Error>` | CMake インストールディレクトリ (無ければダウンロード) |
| `cmake_path() -> Result<PathBuf, Error>` | `cmake` バイナリの絶対パス (無ければダウンロード) |
| `build<P: AsRef<Path>>(path: P) -> PathBuf` | `CMAKE` 環境変数を設定して `cmake::build(path)` を実行 |
| `set_cmake_env()` | `CMAKE` 環境変数を設定する (既に設定済みなら何もしない) |

### 再エクスポート

- `shiguredo_cmake::Config` = `cmake::Config` (詳細は [`cmake` クレートのドキュメント](https://docs.rs/cmake) を参照)
- `shiguredo_cmake::Error` (固有エラー型、`std::error::Error` 実装あり)

### 環境変数

- `CMAKE`: 利用者が事前にこの環境変数を設定しておくと `set_cmake_env` / `build` はダウンロードせず既存の値を尊重する。システムにインストールされた CMake を使いたい場合に使う

## CLI として使う

```bash
cargo install shiguredo_cmake
```

インストール後、`cmake` コマンドとして使える。引数はそのまま実 cmake バイナリにフォワードされる。

```bash
cmake --version
cmake -S . -B build
cmake --build build
```

## build.rs で使う

### デフォルト

```rust
fn main() {
    let dst = shiguredo_cmake::build("libfoo");
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=foo");
}
```

### Config API でカスタマイズ

```rust
fn main() {
    // CMAKE 環境変数を設定 (キャッシュが無ければこの呼び出し中にダウンロード)
    shiguredo_cmake::set_cmake_env();

    let dst = shiguredo_cmake::Config::new("libfoo")
        .define("FOO", "BAR")
        .cflag("-foo")
        .build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=foo");
}
```

`Config` の API は `cmake` クレートと完全互換なので、`define` / `cflag` / `cxxflag` / `profile` / `generator` などはそのまま使える。

## キャッシュ

- 場所: `$HOME/.cache/shiguredo_cmake/v{version}/{target}/`
  - Windows では `%USERPROFILE%\.cache\shiguredo_cmake\v{version}\{target}\`
  - `{target}` は `macos-universal` / `linux-x86_64` / `linux-aarch64` / `windows-x86_64` / `windows-arm64`
- バージョンごとに別ディレクトリなので、過去バージョンを使い回さない
- 不要になったら手動で削除して問題ない

## 注意点

- `set_cmake_env` / `build` の呼び出し時にキャッシュが無ければダウンロードが走るため、`build.rs` の初回ビルドは遅くなる
- `Config` は `cmake` クレートをそのまま再エクスポートしているので、API ドキュメントは `cmake` クレートを参照する
- システム CMake を使いたい場合は `CMAKE` 環境変数を事前に設定する
