fn main() {
    println!(
        "cargo:rustc-env=PKG_BUILD_DATE={}",
        chrono::Utc::now().format("%Y-%m-%d")
    );
}
