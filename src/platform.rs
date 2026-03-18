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
            env!("SHIGUREDO_CMAKE_SHA256_MACOS_UNIVERSAL"),
            "CMake.app/Contents/bin/cmake",
            "macos-universal",
        ),
        ("linux", "x86_64") => (
            format!("cmake-{version}-linux-x86_64.tar.gz"),
            env!("SHIGUREDO_CMAKE_SHA256_LINUX_X86_64"),
            "bin/cmake",
            "linux-x86_64",
        ),
        ("linux", "aarch64") => (
            format!("cmake-{version}-linux-aarch64.tar.gz"),
            env!("SHIGUREDO_CMAKE_SHA256_LINUX_AARCH64"),
            "bin/cmake",
            "linux-aarch64",
        ),
        ("windows", "x86_64") => (
            format!("cmake-{version}-windows-x86_64.zip"),
            env!("SHIGUREDO_CMAKE_SHA256_WINDOWS_X86_64"),
            "bin/cmake.exe",
            "windows-x86_64",
        ),
        ("windows", "aarch64") => (
            format!("cmake-{version}-windows-arm64.zip"),
            env!("SHIGUREDO_CMAKE_SHA256_WINDOWS_ARM64"),
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
