extern crate cmake;
extern crate pkg_config;

use std::env;

fn main() {
    if env::var_os("HARFBUZZ_SYS_NO_PKG_CONFIG").is_none() {
        if pkg_config::find_library("harfbuzz").is_ok() {
            return
        }
    }

    let mut cmd = cmake::Config::new("harfbuzz");
    cmd.define("HB_HAVE_GRAPHITE2", "OFF");
    cmd.define("HB_BUILTIN_UCDN", "ON");
    cmd.define("HB_HAVE_GLIB", "OFF");
    cmd.define("HB_HAVE_ICU", "OFF");

    cmd.define("HB_HAVE_FREETYPE", "ON");
    println!("cargo:rustc-link-search=native={}/lib", env::var("DEP_FREETYPE_LIBS").unwrap());
    println!("cargo:rustc-link-lib=static=freetyped");
    cmd.define("FREETYPE_LIBRARY", "freetyped");
    cmd.define("FREETYPE_INCLUDE_DIRS", env::var("DEP_FREETYPE_INCLUDE").unwrap());

    cmd.define("HB_BUILD_UTILS", "OFF");
    cmd.define("BUILD_SHARED_LIBS", "OFF");

    #[cfg(any(target_os="macos", target_os="ios"))]
    {
        cmd.define("HB_HAVE_CORETEXT", "ON");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
    }

    let dest = cmd.build();
    println!("cargo:rustc-link-search=native={}/lib", dest.display());
    println!("cargo:rustc-link-lib=static=harfbuzz");

    // Dependent crates that need to find hb.h can use DEP_HARFBUZZ_INCLUDE from their build.rs.
    println!("cargo:include={}", env::current_dir().unwrap().join("harfbuzz/src").display());
}
