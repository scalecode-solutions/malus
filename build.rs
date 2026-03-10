fn main() {
    // Always needed for the ObjC runtime
    println!("cargo:rustc-link-lib=framework=Foundation");

    #[cfg(feature = "core-foundation")]
    println!("cargo:rustc-link-lib=framework=CoreFoundation");

    #[cfg(feature = "appkit")]
    println!("cargo:rustc-link-lib=framework=AppKit");

    #[cfg(feature = "metal")]
    println!("cargo:rustc-link-lib=framework=Metal");

    #[cfg(feature = "core-graphics")]
    println!("cargo:rustc-link-lib=framework=CoreGraphics");

    #[cfg(feature = "core-text")]
    println!("cargo:rustc-link-lib=framework=CoreText");

    #[cfg(feature = "quartz-core")]
    println!("cargo:rustc-link-lib=framework=QuartzCore");
}
