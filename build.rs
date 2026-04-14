use std::path::{Path, PathBuf};

fn rerun_if_changed(path: &Path) {
    println!("cargo:rerun-if-changed={}", path.display());
}

fn main() {
    let manifest_dir = PathBuf::from(
        std::env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is set by Cargo"),
    );
    let yespower_dir = manifest_dir.join("depends/yespower");

    for file in [
        "insecure_memzero.h",
        "sha256.c",
        "sha256.h",
        "sysendian.h",
        "yespower-opt.c",
        "yespower-platform.c",
        "yespower.h",
    ] {
        rerun_if_changed(&yespower_dir.join(file));
    }

    cc::Build::new()
        .warnings(false)
        .include(&yespower_dir)
        .std("gnu99")
        .flag_if_supported("-fomit-frame-pointer")
        .flag_if_supported("-funroll-loops")
        .file(yespower_dir.join("yespower-opt.c"))
        .file(yespower_dir.join("sha256.c"))
        .compile("yespower_tidecoin");
}
