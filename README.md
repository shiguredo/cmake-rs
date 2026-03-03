# cmake-rs

[![shiguredo_cmake](https://img.shields.io/crates/v/shiguredo_cmake.svg)](https://crates.io/crates/shiguredo_cmake)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## About Shiguredo's open source software

We will not respond to PRs or issues that have not been discussed on Discord. Also, Discord is only available in Japanese.

Please read <https://github.com/shiguredo/oss> before use.

## 時雨堂のオープンソースソフトウェアについて

利用前に <https://github.com/shiguredo/oss> をお読みください。

## 概要

[CMake](https://cmake.org/) の公式プリビルトバイナリをダウンロードして提供する Rust クレートです。

Rust の [cmake](https://crates.io/crates/cmake) クレートは CMake バイナリを含まないため、ユーザーが別途 CMake をインストールする必要があります。
このクレートは [Kitware 公式の GitHub Releases](https://github.com/Kitware/CMake/releases) からプリビルトバイナリをダウンロードし、ローカルにキャッシュして提供します。

PyPI の [cmake](https://pypi.org/project/cmake/) パッケージを参考にしています。

## 特徴

- CMake バージョンは `Cargo.toml` の `[package.metadata.external-dependencies]` で管理
- 初回呼び出し時にプリビルトバイナリを自動ダウンロード
- SHA256 チェックサムによる検証
- `$HOME/.cache/shiguredo_cmake/` 以下にキャッシュ (2 回目以降はダウンロード不要)
- ダウンロードは `curl`、展開は `tar` コマンドを使用し依存を最小化
- [cmake](https://crates.io/crates/cmake) クレートの `Config` API をそのまま利用可能
- macOS / Linux / Windows 対応

## 対応プラットフォーム

| OS | アーキテクチャ |
| :-- | :-- |
| macOS | x86_64, aarch64 |
| Linux | x86_64, aarch64 |
| Windows | x86_64, aarch64 |

## 必要な環境

- Rust 1.88 以上
- `curl` コマンド
- `tar` コマンド

## コマンドとしての使用

```bash
cargo install shiguredo_cmake
```

インストール後、`cmake` コマンドとして使用できます。

```bash
cmake --version
```

## build.rs での使用

`cmake` クレートの API をそのまま使用できます。
`CMAKE` 環境変数を自動設定し、ダウンロード済みバイナリを使用します。

```rust
// build.rs

fn main() {
    // デフォルト設定でビルド
    let dst = shiguredo_cmake::build("libfoo");

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=foo");
}
```

`Config` を使ったカスタム設定も可能です。

```rust
// build.rs

fn main() {
    // CMAKE 環境変数を設定
    shiguredo_cmake::set_cmake_env();

    // cmake クレートの Config API をそのまま使用
    let dst = shiguredo_cmake::Config::new("libfoo")
        .define("FOO", "BAR")
        .cflag("-foo")
        .build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=foo");
}
```

## CMake ライセンス

<https://github.com/Kitware/CMake/blob/master/LICENSE.rst>

```text
Copyright 2000-2026 Kitware, Inc. and Contributors

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions
are met:

* Redistributions of source code must retain the above copyright
  notice, this list of conditions and the following disclaimer.

* Redistributions in binary form must reproduce the above copyright
  notice, this list of conditions and the following disclaimer in the
  documentation and/or other materials provided with the distribution.

* Neither the name of Kitware, Inc. nor the names of Contributors
  may be used to endorse or promote products derived from this
  software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
"AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
```

## ライセンス

Apache License 2.0

```text
Copyright 2026-2026, Shiguredo Inc.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```
