use std::path::Path;
use std::process::Command;

use crate::error::Error;

/// アーカイブを展開する
/// tar.gz と zip の両方に対応 (Windows 10+ 内蔵 tar を使用)
pub fn extract(archive: &Path, dest: &Path) -> Result<(), Error> {
    let archive_name = archive.file_name().and_then(|n| n.to_str()).unwrap_or("");

    let tar_flag = if archive_name.ends_with(".tar.gz") {
        "xzf"
    } else if archive_name.ends_with(".zip") {
        "xf"
    } else {
        return Err(Error::Extract {
            path: archive.to_owned(),
            source: std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("unsupported archive format: {archive_name}"),
            ),
        });
    };

    let status = Command::new("tar")
        .arg(tar_flag)
        .arg(archive)
        .arg("-C")
        .arg(dest)
        .arg("--strip-components=1")
        .status()
        .map_err(|e| Error::Extract {
            path: archive.to_owned(),
            source: e,
        })?;

    if !status.success() {
        return Err(Error::ExtractFailed {
            path: archive.to_owned(),
            status,
        });
    }

    Ok(())
}
