fn main() {
    let libstdcxx = cfg!(feature = "libstdcxx");
    let libcxx = cfg!(feature = "libcxx");
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
            cc::Build::new().cpp(true).compile("link-cplusplus");
        }
    }
}
