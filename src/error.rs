use std::fmt;
use std::io;
use std::path::PathBuf;

/// shiguredo_cmake のエラー型
#[derive(Debug)]
pub enum Error {
    /// サポートされていないプラットフォーム
    UnsupportedPlatform { os: String, arch: String },
    /// ホームディレクトリの取得に失敗
    HomeNotFound,
    /// キャッシュディレクトリの作成に失敗
    CacheDir { path: PathBuf, source: io::Error },
    /// ダウンロードに失敗
    Download { url: String, source: io::Error },
    /// ダウンロードコマンドが非ゼロで終了
    DownloadFailed {
        url: String,
        status: std::process::ExitStatus,
    },
    /// SHA256 コマンドの実行に失敗
    Sha256 { path: PathBuf, source: io::Error },
    /// SHA256 コマンドが非ゼロで終了
    Sha256Failed {
        path: PathBuf,
        status: std::process::ExitStatus,
    },
    /// SHA256 チェックサムの不一致
    ChecksumMismatch { expected: String, actual: String },
    /// アーカイブ展開に失敗
    Extract { path: PathBuf, source: io::Error },
    /// アーカイブ展開コマンドが非ゼロで終了
    ExtractFailed {
        path: PathBuf,
        status: std::process::ExitStatus,
    },
    /// IO エラー
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedPlatform { os, arch } => {
                write!(f, "unsupported platform: os={os}, arch={arch}")
            }
            Self::HomeNotFound => write!(f, "home directory not found"),
            Self::CacheDir { path, source } => {
                write!(
                    f,
                    "failed to create cache directory {}: {source}",
                    path.display()
                )
            }
            Self::Download { url, source } => {
                write!(f, "failed to download {url}: {source}")
            }
            Self::DownloadFailed { url, status } => {
                write!(f, "download failed for {url}: {status}")
            }
            Self::Sha256 { path, source } => {
                write!(
                    f,
                    "failed to compute SHA256 for {}: {source}",
                    path.display()
                )
            }
            Self::Sha256Failed { path, status } => {
                write!(f, "SHA256 command failed for {}: {status}", path.display())
            }
            Self::ChecksumMismatch { expected, actual } => {
                write!(
                    f,
                    "SHA256 checksum mismatch: expected={expected}, actual={actual}"
                )
            }
            Self::Extract { path, source } => {
                write!(f, "failed to extract {}: {source}", path.display())
            }
            Self::ExtractFailed { path, status } => {
                write!(f, "extraction failed for {}: {status}", path.display())
            }
            Self::Io(source) => write!(f, "IO error: {source}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::CacheDir { source, .. }
            | Self::Download { source, .. }
            | Self::Sha256 { source, .. }
            | Self::Extract { source, .. } => Some(source),
            Self::Io(source) => Some(source),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}
