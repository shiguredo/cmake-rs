// Cargo.toml の [package.metadata.external-dependencies] から
// cmake.version を取得してコンパイル時環境変数に設定する

fn main() {
    println!("cargo:rerun-if-changed=Cargo.toml");

    let cargo_toml = std::fs::read_to_string("Cargo.toml").expect("Cargo.toml の読み込みに失敗");

    let table = shiguredo_toml::from_str(&cargo_toml).expect("Cargo.toml のパースに失敗");

    let version = table
        .get("package")
        .and_then(|v| v.get("metadata"))
        .and_then(|v| v.get("external-dependencies"))
        .and_then(|v| v.get("cmake"))
        .and_then(|v| v.get("version"))
        .and_then(|v| v.as_str())
        .expect("[package.metadata.external-dependencies] cmake.version が見つからない");

    println!("cargo:rustc-env=SHIGUREDO_CMAKE_VERSION={version}");
}
