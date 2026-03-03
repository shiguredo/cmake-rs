use std::path::{Path, PathBuf};

use crate::error::Error;

/// キャッシュディレクトリのパスを返す
/// $HOME/.cache/shiguredo_cmake/v{version}/{target}/
pub fn cache_dir(version: &str, target: &str) -> Result<PathBuf, Error> {
    let home = home_dir()?;
    Ok(home
        .join(".cache")
        .join("shiguredo_cmake")
        .join(format!("v{version}"))
        .join(target))
}

/// キャッシュディレクトリを作成する
pub fn ensure_cache_dir(dir: &Path) -> Result<(), Error> {
    std::fs::create_dir_all(dir).map_err(|e| Error::CacheDir {
        path: dir.to_owned(),
        source: e,
    })
}

/// ホームディレクトリを取得する
fn home_dir() -> Result<PathBuf, Error> {
    std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
        .ok_or(Error::HomeNotFound)
}
