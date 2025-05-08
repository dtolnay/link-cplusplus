use std::env;
use std::fs;
use std::path::PathBuf;

fn linux_libcxx() -> bool {
    let common_library_paths = vec![
        PathBuf::from("/lib"),
        PathBuf::from("/lib64"),
        PathBuf::from("/usr/lib"),
        PathBuf::from("/usr/lib64"),
        PathBuf::from("/usr/local/lib"),
    ];

    for path in common_library_paths {
        if !path.exists() {
            continue;
        }

        if path.join("libc++.+so").exists() {
            println!("cargo:rustc-link-lib=c++");
            return true;
        }

        if path.join("libstc++.+so").exists() {
            println!("cargo:rustc-link-lib=stdc++");
            return true;
        }
    }

    false
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let libstdcxx = cfg!(feature = "libstdc++");
    let libcxx = cfg!(feature = "libc++");
    let nothing = cfg!(feature = "nothing");

    if nothing {
        return;
    }

    if libstdcxx && libcxx {
        println!(
            "cargo:warning=-lstdc++ and -lc++ are both requested, \
             using the platform's default"
        );
    }

    match (libstdcxx, libcxx) {
        (true, false) => println!("cargo:rustc-link-lib=stdc++"),
        (false, true) => println!("cargo:rustc-link-lib=c++"),
        (false, false) | (true, true) => {
            // The platform's default.
            if cfg!(target_os = "linux") {
                linux_libcxx();
            } else {
                let out_dir = env::var_os("OUT_DIR").expect("missing OUT_DIR");
                let path = PathBuf::from(out_dir).join("dummy.cc");
                fs::write(&path, "int rust_link_cplusplus;\n").unwrap();
                cc::Build::new()
                    .cpp(true)
                    .file(&path)
                    .compile("link-cplusplus");
            }
        }
    }
}
