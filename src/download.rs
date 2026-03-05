use std::path::Path;
use std::process::Command;

use crate::error::Error;

/// curl コマンドでファイルをダウンロードする
pub fn download(url: &str, output: &Path) -> Result<(), Error> {
    let status = Command::new("curl")
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

/// システムコマンドでファイルの SHA256 チェックサムを検証する
///
/// Linux: sha256sum, macOS: shasum -a 256, Windows: certutil -hashfile
pub fn verify_sha256(path: &Path, expected: &str) -> Result<(), Error> {
    let output = sha256_command(path).map_err(|e| Error::Sha256 {
        path: path.to_owned(),
        source: e,
    })?;

    if !output.status.success() {
        return Err(Error::Sha256Failed {
            path: path.to_owned(),
            status: output.status,
        });
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let actual = parse_sha256_output(&stdout);

    if actual != expected {
        return Err(Error::ChecksumMismatch {
            expected: expected.to_owned(),
            actual,
        });
    }

    Ok(())
}

/// プラットフォームに応じた SHA256 コマンドを実行する
fn sha256_command(path: &Path) -> Result<std::process::Output, std::io::Error> {
    if cfg!(target_os = "macos") {
        Command::new("shasum")
            .args(["-a", "256"])
            .arg(path)
            .output()
    } else if cfg!(target_os = "windows") {
        Command::new("certutil")
            .args(["-hashfile"])
            .arg(path)
            .arg("SHA256")
            .output()
    } else {
        Command::new("sha256sum").arg(path).output()
    }
}

/// SHA256 コマンドの出力からハッシュ値を取り出す
///
/// shasum / sha256sum: "<hash>  <filename>"
/// certutil: "SHA256 hash of <path>:\n<hash>\nCertUtil: ..."
fn parse_sha256_output(stdout: &str) -> String {
    if cfg!(target_os = "windows") {
        // certutil の出力は 2 行目がハッシュ値（スペース区切りの場合があるので除去する）
        stdout
            .lines()
            .nth(1)
            .unwrap_or("")
            .replace(' ', "")
            .to_ascii_lowercase()
    } else {
        // shasum / sha256sum はハッシュの後にスペースとファイル名が続く
        stdout.split_whitespace().next().unwrap_or("").to_owned()
    }
}
