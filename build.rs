fn main() {
    println!("cargo:rerun-if-changed=./src");
    println!("cargo:rerun-if-changed=./build.rs");

    let output_dir = std::env::var("OUT_DIR").unwrap();

    println!("cargo:rustc-link-search=all=C:/Users/mycrl/Desktop/tests/temp");
    println!("cargo:rustc-link-search=all={}", &output_dir);
    println!("cargo:rustc-link-lib=static=rtc");

    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=winmm");
        println!("cargo:rustc-link-lib=secur32");
        println!("cargo:rustc-link-lib=msdmo");
        println!("cargo:rustc-link-lib=dmoguids");
        println!("cargo:rustc-link-lib=wmcodecdspuuid");
        println!("cargo:rustc-link-lib=iphlpapi");
    }

    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=c++");
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=AudioToolbox");
        println!("cargo:rustc-link-lib=framework=AudioUnit");
        println!("cargo:rustc-link-lib=framework=CoreServices");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=CoreAudio");
        println!("cargo:rustc-link-lib=framework=CoreGraphics");
    }

    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=stdc++");
    }
}
