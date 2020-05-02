fn main() {
    println!("cargo:rustc-link-search=/home/bashi/work/rustracy/lib");

    // TODO: Figure out how to build this library on unix platforms.
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=user32");
        cc::Build::new()
            .use_plt(false)
            .flag_if_supported("-flto")
            .flag_if_supported("/LTCG")
            .cpp(true)
            .include("./tracy")
            .define("TRACY_ENABLE", None)
            .file("./tracy/TracyClient.cpp")
            .compile("libtracy.a");
    }
}
