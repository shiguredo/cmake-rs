// Cargo.toml の [package.metadata.external-dependencies] から
// cmake.version と sha256 を取得してコンパイル時環境変数に設定する

fn main() {
    println!("cargo:rerun-if-changed=Cargo.toml");

    let cargo_toml = std::fs::read_to_string("Cargo.toml").expect("failed to read Cargo.toml");

    let table = shiguredo_toml::from_str(&cargo_toml).expect("failed to parse Cargo.toml");

    let cmake = table
        .get("package")
        .and_then(|v| v.get("metadata"))
        .and_then(|v| v.get("external-dependencies"))
        .and_then(|v| v.get("cmake"))
        .expect("[package.metadata.external-dependencies.cmake] is missing");

    let version = cmake
        .get("version")
        .and_then(|v| v.as_str())
        .expect("[package.metadata.external-dependencies.cmake] version is missing");

    println!("cargo:rustc-env=SHIGUREDO_CMAKE_VERSION={version}");

    let sha256 = cmake
        .get("sha256")
        .expect("[package.metadata.external-dependencies.cmake.sha256] is missing");

    // 各プラットフォームの SHA256 ハッシュ値を環境変数に設定する
    for target in [
        "macos-universal",
        "linux-x86_64",
        "linux-aarch64",
        "windows-x86_64",
        "windows-arm64",
    ] {
        let hash = sha256
            .get(target)
            .and_then(|v| v.as_str())
            .unwrap_or_else(|| {
                panic!("[package.metadata.external-dependencies.cmake.sha256] {target} is missing")
            });
        let env_key = target.replace('-', "_").to_uppercase();
        println!("cargo:rustc-env=SHIGUREDO_CMAKE_SHA256_{env_key}={hash}");
    }
}
