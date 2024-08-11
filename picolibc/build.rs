use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {
    // Gather system paths
    // WONDERFUL_TOOLCHAIN ?= /opt/wonderful
    // BLOCKSDS ?= $(WONDERFUL_TOOLCHAIN)/thirdparty/blocksds/core
    let wonderful_toolchain =
        env::var("WONDERFUL_TOOLCHAIN").unwrap_or_else(|_| "/opt/wonderful".to_string());
    let wonderful_toolchain = Path::new(&wonderful_toolchain);
    let _ = env::var("BLOCKSDS")
        .map(PathBuf::from)
        .unwrap_or_else(|_| wonderful_toolchain.join("thirdparty/blocksds/core"));
    let gcc_dir = wonderful_toolchain.join("toolchain/gcc-arm-none-eabi");

    // Only set search paths, the actual linking is done by the linker script in the user crate
    println!(
        "cargo:rustc-link-search=native={}",
        gcc_dir.join("arm-none-eabi/lib/").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        gcc_dir.join("arm-none-eabi/lib/arm946e-s").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        gcc_dir.join("lib/gcc/arm-none-eabi/14.2.0").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        gcc_dir
            .join("lib/gcc/arm-none-eabi/14.2.0/arm946e-s")
            .display()
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
    let includes = [format!(
        "-I{}",
        gcc_dir.join("arm-none-eabi/include").display()
    )];

    let bindings = bindgen::Builder::default()
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
        .allowlist_recursively(false)
        .merge_extern_blocks(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("picolibc.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rerun-if-changed=build.rs");
}
