fn main() {
    let target = std::env::var("CARGO_CFG_TARGET_OS");

    if target.as_deref() == Ok("macos") || target.as_deref() == Ok("ios") {
        println!("cargo:rustc-link-lib=framework=CoreAudio");
        println!("cargo:rustc-link-lib=framework=AudioToolBox");
    } else if target.as_deref() == Ok("haiku") {
        cc::Build::new()
            .cpp(true)
            .file("src/haiku/QuadSnd.cpp")
            .compile("shims_lib");
        println!("cargo:rustc-link-lib=game");
        println!("cargo:rustc-link-lib=be");

        println!("cargo:rerun-if-changed=src/haiku/QuadSnd.cpp");
    }
}
