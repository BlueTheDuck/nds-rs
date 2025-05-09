use std::env;
use std::path::{Path, PathBuf};

// HACK: Allows us to have a folder called "build" with submodules
// so we don't have to have a bunch of files in the root directory
// or everything in "build.rs".
mod build {
    mod cb;

    pub use cb::*;
}
pub use build::CbEnumRenamer;

fn configure_console_h(builder: bindgen::Builder) -> bindgen::Builder {
    const REPLACEMENTS: &[[&str; 2]] = &[
        ["DebugDevice_NULL", "Null"],
        ["DebugDevice_NOCASH", "NoCash"],
        ["DebugDevice_CONSOLE", "Console"],
    ];
    builder
        .parse_callbacks(CbEnumRenamer::new_boxed(REPLACEMENTS))
        .allowlist_file(".*/console\\.h")
        .rustified_enum("DebugDevice")
}
fn configure_background_h(builder: bindgen::Builder) -> bindgen::Builder {
    builder
        .constified_enum_module("BackgroundControl")
        .rustified_enum("BgSize")
        .rustified_enum("BgType")
        .allowlist_file(".*/background\\.h")
}

fn main() {
    // Time at the start of the build
    let now = chrono::Local::now();

    // Gather system paths
    // WONDERFUL_TOOLCHAIN ?= /opt/wonderful
    // BLOCKSDS ?= $(WONDERFUL_TOOLCHAIN)/thirdparty/blocksds/core
    let wonderful_toolchain =
        env::var("WONDERFUL_TOOLCHAIN").unwrap_or_else(|_| "/opt/wonderful".to_string());
    let wonderful_toolchain = Path::new(&wonderful_toolchain);
    let blocksds = env::var("BLOCKSDS")
        .map(PathBuf::from)
        .unwrap_or_else(|_| wonderful_toolchain.join("thirdparty/blocksds/core"));
    let gcc_dir = wonderful_toolchain.join("toolchain/gcc-arm-none-eabi");

    let libnds_path = blocksds.join("libs/libnds/lib/libnds9.a");
    if !libnds_path.exists() {
        panic!("libnds9.a not found at {libnds_path:?}");
    }

    println!(
        "cargo:rustc-link-search=native={}",
        blocksds.join("libs/libnds/lib").display()
    );
    println!(
        "cargo:rustc-link-lib=static={}",
        match std::env::var("PROFILE").unwrap().as_str() {
            "debug" => "nds9d",
            _ => "nds9",
        }
    );

    let system_flags = [
        "-mthumb",
        "-mcpu=arm946e-s+nofp",
        "-std=gnu17",
        "--target=arm-none-eabi",
    ];
    let defines = [
        "-D__NDS__",
        "-D__BLOCKSDS__",
        "-DARM9",
        "-D__USES_INITFINI__",
    ];
    let includes = [
        format!("-I{}", blocksds.join("libs/libnds/include").display()),
        format!("-I{}", gcc_dir.join("arm-none-eabi/include").display()),
    ];

    let base_builder = bindgen::Builder::default()
        .clang_args(system_flags)
        .clang_arg(format!(
            "--sysroot={}",
            gcc_dir.join("arm-none-eabi").display()
        ))
        .clang_arg("-isystem \\$SYSROOT/include")
        .clang_args(defines)
        .clang_args(includes)
        .header("./wrapper.h")
        .use_core()
        .sort_semantically(true)
        .layout_tests(false)
        .raw_line(format!("// T = {now}"))
        .prepend_enum_name(false)
        .allowlist_recursively(false)
        .merge_extern_blocks(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    let bindings_folder = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings");
    std::fs::DirBuilder::new()
        .recursive(true)
        .create(&bindings_folder)
        .unwrap();

    configure_console_h(base_builder.clone())
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(bindings_folder.join("console.rs"))
        .expect("Couldn't write bindings!");

    configure_background_h(base_builder.clone())
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(bindings_folder.join("backgrounds.rs"))
        .expect("Couldn't write bindings!");
    println!("cargo:rerun-if-changed=build.rs");
}
