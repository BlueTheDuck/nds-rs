use std::env;

fn main() {
    let dkp = env::var("DEVKITPRO").expect("Please set $DEVKITPRO");

    println!("cargo:rustc-link-search=native={dkp}/libnds/lib");
    println!(
        "cargo:rustc-link-lib=static={}",
        match env::var("PROFILE").unwrap().as_str() {
            "debug" => "nds9d",
            _ => "nds9",
        }
    );
}
