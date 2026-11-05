use std::{env, path::PathBuf, process::Command};
fn main() {
    println!("cargo:rerun-if-changed=platform/macos/haptics.m");
    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("macos") {
        return;
    }
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let arch = match env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
        "aarch64" => "arm64",
        "x86_64" => "x86_64",
        _ => panic!("Unsupported macOS architecture"),
    };
    let sdk = Command::new("xcrun")
        .arg("--show-sdk-path")
        .output()
        .expect("Xcode command line tools required");
    assert!(sdk.status.success());
    let sdk = String::from_utf8(sdk.stdout).unwrap();
    assert!(
        Command::new("xcrun")
            .args([
                "clang",
                "-fobjc-arc",
                "-fmodules",
                "-fblocks",
                "-mmacosx-version-min=11.0",
                "-arch",
                arch,
                "-isysroot",
                sdk.trim(),
                "-c",
                "platform/macos/haptics.m",
                "-o"
            ])
            .arg(out.join("haptics.o"))
            .arg(format!(
                "-fmodules-cache-path={}",
                out.join("clang-modules").display()
            ))
            .status()
            .unwrap()
            .success()
    );
    assert!(
        Command::new("ar")
            .arg("crs")
            .arg(out.join("libarcade_haptics.a"))
            .arg(out.join("haptics.o"))
            .status()
            .unwrap()
            .success()
    );
    println!("cargo:rustc-link-search=native={}", out.display());
    println!("cargo:rustc-link-lib=static=arcade_haptics");
    for framework in ["Foundation", "GameController", "CoreHaptics"] {
        println!("cargo:rustc-link-lib=framework={framework}");
    }
}
