fn main() {
    let wonderful = std::env::var("WONDERFUL_TOOLCHAIN").unwrap_or_else(|_| "/opt/wonderful".to_string());
    let wonderful = std::path::Path::new(&wonderful);
    let blocksds = wonderful.join("thirdparty/blocksds/core");

    let libnds_path = blocksds.join("libs/libnds/lib/libnds9.a");
    if !libnds_path.exists() {
        panic!("libnds9.a not found at {}", libnds_path.display());
    }

    println!(
        "cargo:rustc-link-search=native={blocksds}/libs/libnds/lib",
        blocksds = blocksds.display()
    );
    println!(
        "cargo:rustc-link-lib=static={}",
        match std::env::var("PROFILE").unwrap().as_str() {
            "debug" => "nds9d",
            _ => "nds9",
        }
    );
}
