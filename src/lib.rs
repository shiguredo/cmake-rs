mod archive;
mod cache;
mod download;
mod error;
mod platform;

pub use cmake::Config;
pub use error::Error;

use std::path::PathBuf;

/// build.rs で設定された CMake バージョン
const CMAKE_VERSION: &str = env!("SHIGUREDO_CMAKE_VERSION");

/// CMake バージョンを返す
pub fn cmake_version() -> &'static str {
    CMAKE_VERSION
}

/// CMake インストールディレクトリを返す
/// キャッシュになければダウンロードして展開する
pub fn cmake_dir() -> Result<PathBuf, Error> {
    let info = platform::detect()?;
    let dir = cache::cache_dir(CMAKE_VERSION, info.target)?;
    let cmake_bin = dir.join(info.cmake_relative_path);

    if cmake_bin.exists() {
        return Ok(dir);
    }

    // 一時ディレクトリにダウンロード・展開し、完了後にリネームすることで
    // 複数プロセスが同時に実行された場合の競合を防ぐ
    let tmp_dir = dir.with_file_name(format!("{}.tmp.{}", info.target, std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp_dir);
    cache::ensure_cache_dir(&tmp_dir)?;

    let archive_path = tmp_dir.join(&info.archive_name);
    let result = download::download(&info.url, &archive_path)
        .and_then(|()| download::verify_sha256(&archive_path, info.sha256))
        .and_then(|()| archive::extract(&archive_path, &tmp_dir));

    if let Err(e) = result {
        let _ = std::fs::remove_dir_all(&tmp_dir);
        return Err(e);
    }

    let _ = std::fs::remove_file(&archive_path);

    // 最終ディレクトリにリネーム
    // 別プロセスが先に完了していた場合はリネームに失敗するが、既存のものを使用する
    match std::fs::rename(&tmp_dir, &dir) {
        Ok(()) => {}
        Err(_) if cmake_bin.exists() => {
            let _ = std::fs::remove_dir_all(&tmp_dir);
        }
        Err(e) => {
            let _ = std::fs::remove_dir_all(&tmp_dir);
            return Err(Error::Io(e));
        }
    }

    if !cmake_bin.exists() {
        return Err(Error::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!(
                "cmake binary not found after extraction: {}",
                cmake_bin.display()
            ),
        )));
    }

    Ok(dir)
}

/// CMake バイナリのパスを返す
/// キャッシュになければダウンロードして展開する
pub fn cmake_path() -> Result<PathBuf, Error> {
    let dir = cmake_dir()?;
    let info = platform::detect()?;
    Ok(dir.join(info.cmake_relative_path))
}

/// デフォルト設定で CMake プロジェクトをビルドする
/// CMAKE 環境変数を自動設定してから cmake::build を呼ぶ
pub fn build<P: AsRef<std::path::Path>>(path: P) -> PathBuf {
    set_cmake_env();
    cmake::build(path)
}

/// CMAKE 環境変数にダウンロード済みバイナリのパスを設定する
/// build.rs で Config::new() の前に呼ぶことで cmake クレートが自動的にこのバイナリを使用する
pub fn set_cmake_env() {
    if std::env::var_os("CMAKE").is_some() {
        return;
    }
    match cmake_path() {
        Ok(path) => {
            // build.rs はシングルスレッドで実行されるため安全
            unsafe {
                std::env::set_var("CMAKE", path);
            }
        }
        Err(e) => {
            println!("cargo:warning=shiguredo_cmake: failed to set CMAKE: {e}");
        }
    }
}
