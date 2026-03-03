use crate::error::Error;

/// プラットフォーム固有の情報
pub struct PlatformInfo {
    /// アーカイブファイル名
    pub archive_name: String,
    /// ダウンロード URL
    pub url: String,
    /// SHA256 チェックサム (hex)
    pub sha256: &'static str,
    /// 展開後の cmake バイナリの相対パス
    pub cmake_relative_path: &'static str,
    /// ターゲット識別子 (キャッシュディレクトリ名に使用)
    pub target: &'static str,
}

/// 現在のプラットフォームに対応する情報を返す
pub fn detect() -> Result<PlatformInfo, Error> {
    let version = crate::cmake_version();
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    let (archive_suffix, sha256, cmake_relative_path, target) = match (os, arch) {
        ("macos", "x86_64" | "aarch64") => (
            format!("cmake-{version}-macos10.10-universal.tar.gz"),
            "910b965a6fc72928412dd369c957643ff17a0990cc2435a2573b04c1352d9ff3",
            "CMake.app/Contents/bin/cmake",
            "macos-universal",
        ),
        ("linux", "x86_64") => (
            format!("cmake-{version}-linux-x86_64.tar.gz"),
            "5bb505d5e0cca0480a330f7f27ccf52c2b8b5214c5bba97df08899f5ef650c23",
            "bin/cmake",
            "linux-x86_64",
        ),
        ("linux", "aarch64") => (
            format!("cmake-{version}-linux-aarch64.tar.gz"),
            "e529c75f18f27ba27c52b329efe7b1f98dc32ccc0c6d193c7ab343f888962672",
            "bin/cmake",
            "linux-aarch64",
        ),
        ("windows", "x86_64") => (
            format!("cmake-{version}-windows-x86_64.zip"),
            "eb4ebf5155dbb05436d675706b2a08189430df58904257ae5e91bcba4c86933c",
            "bin/cmake.exe",
            "windows-x86_64",
        ),
        ("windows", "aarch64") => (
            format!("cmake-{version}-windows-arm64.zip"),
            "751b206b1cf65151b72c525d26267c1d9beebf8fafc365ae00286571d9fd3ed9",
            "bin/cmake.exe",
            "windows-arm64",
        ),
        _ => {
            return Err(Error::UnsupportedPlatform {
                os: os.to_owned(),
                arch: arch.to_owned(),
            });
        }
    };

    let url =
        format!("https://github.com/Kitware/CMake/releases/download/v{version}/{archive_suffix}");

    Ok(PlatformInfo {
        archive_name: archive_suffix,
        url,
        sha256,
        cmake_relative_path,
        target,
    })
}
