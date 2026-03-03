use std::path::Path;

use crate::error::Error;

/// curl コマンドでファイルをダウンロードする
pub fn download(url: &str, output: &Path) -> Result<(), Error> {
    let status = std::process::Command::new("curl")
        .args(["-fSL", "--retry", "3", "-o"])
        .arg(output)
        .arg(url)
        .status()
        .map_err(|e| Error::Download {
            url: url.to_owned(),
            source: e,
        })?;

    if !status.success() {
        return Err(Error::DownloadFailed {
            url: url.to_owned(),
            status,
        });
    }

    Ok(())
}

/// ファイルの SHA256 チェックサムを検証する
pub fn verify_sha256(path: &Path, expected: &str) -> Result<(), Error> {
    use sha2::Digest;

    let data = std::fs::read(path)?;
    let hash = sha2::Sha256::digest(&data);
    let actual = format!("{hash:x}");

    if actual != expected {
        return Err(Error::ChecksumMismatch {
            expected: expected.to_owned(),
            actual,
        });
    }

    Ok(())
}
