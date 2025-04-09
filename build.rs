fn main() {
    println!("cargo:rerun-if-changed=assets");

    let version_info_str = env!("CARGO_PKG_VERSION");
    let version_info_ver = format!(
        "{},{},{},0",
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
        env!("CARGO_PKG_VERSION_PATCH")
    );
    embed_resource::compile(
        "assets/resources.rc",
        &[
            format!("VERSION_INFO_STR=\"{}\"", version_info_str),
            format!("VERSION_INFO_VER={}", version_info_ver)
        ]
    );
}