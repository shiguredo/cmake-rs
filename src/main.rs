//! ダウンロードした CMake バイナリを `cmake` コマンドとして実行する。
//!
//! コマンドライン引数はすべて実体の cmake にそのまま渡し、cmake の終了コードを
//! そのままプロセスの終了コードとして返す。

fn main() {
    let cmake = match shiguredo_cmake::cmake_path() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    };

    let status = match std::process::Command::new(&cmake)
        .args(std::env::args_os().skip(1))
        .status()
    {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: failed to execute {}: {e}", cmake.display());
            std::process::exit(1);
        }
    };

    std::process::exit(status.code().unwrap_or(1));
}
