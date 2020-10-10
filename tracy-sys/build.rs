use std::env;

fn configure_build(mut b: cc::Build) -> cc::Build {
    if env::var("CARGO_FEATURE_ALLOW_VM").is_ok() {
        b.define("TRACY_TIMER_QPC", None);
    }
    b
}

fn main() {
    let target_os = match env::var("CARGO_CFG_TARGET_OS") {
        Ok(target_os) => target_os,
        Err(err) => panic!("Can't detect target os: {:?}", err),
    };

    if target_os == "windows" {
        println!("cargo:rustc-link-lib=user32");
    }

    let b = cc::Build::new();
    configure_build(b)
        .file("./tracy/TracyClient.cpp")
        .opt_level(3)
        .flag_if_supported("-Wno-class-memaccess")
        .flag_if_supported("-Wno-enum-compare")
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-function")
        .cpp(true)
        .define("TRACY_ENABLE", None)
        .define("NDEBUG", None)
        .compile("libtracy.a");
}
