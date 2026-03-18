# 変更履歴

- UPDATE
  - 後方互換がある変更
- ADD
  - 後方互換がある追加
- CHANGE
  - 後方互換のない変更
- FIX
  - バグ修正

## develop

## 4.3.0

**リリース日**: 2026-03-18

- [UPDATE] cmake 4.3.0 に上げる
  - @voluntas
- [CHANGE] SHA256 チェックサム検証を `sha2` クレートからシステムコマンドに置き換える
  - Linux: `sha256sum`, macOS: `shasum -a 256`, Windows: PowerShell `Get-FileHash`
  - `sha2` クレートとその依存 8 クレートを削除する
  - @voluntas

## 4.2.3

**リリース日**: 2026-03-03

- [ADD] CMake プリビルトバイナリのダウンロードとキャッシュ機能を追加する
  - Kitware 公式 GitHub Releases からプリビルトバイナリを自動取得する
  - macOS (x86_64, aarch64), Linux (x86_64, aarch64), Windows (x86_64, aarch64) に対応する
  - SHA256 チェックサムによるダウンロード検証を行う
  - `$HOME/.cache/shiguredo_cmake/` にキャッシュする
  - @voluntas
