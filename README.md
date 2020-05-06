`-lstdc++` or `-lc++`
=====================

[![Build Status](https://img.shields.io/github/workflow/status/dtolnay/link-cplusplus/CI/master)](https://github.com/dtolnay/link-cplusplus/actions?query=branch%3Amaster)
[![Latest Version](https://img.shields.io/crates/v/link-cplusplus.svg)](https://crates.io/crates/link-cplusplus)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/link-cplusplus)

This crate exists for the purpose of passing `-lstdc++` or `-lc++` to the
linker, while making it possible for an application to make that choice on
behalf of its library dependencies.

Without this crate, a library would need to:

- pick one or the other to link, with no way for downstream applications to
  override the choice;
- or link neither and require an explicit link flag provided by downstream
  applications even if they would be fine with a default choice;

neither of which are good experiences.

<br>

## Options

An application or library that is fine with either of libstdc++ or libc++ being
linked, whichever is the platform's default, should use:

```toml
[dependencies]
link-cplusplus = "1.0"
```

An application that wants a particular one or the other linked should use:

```toml
[dependencies]
link-cplusplus = { version = "1.0", features = ["libstdcxx"] }

# or

link-cplusplus = { version = "1.0", features = ["libcxx"] }
```

An application that wants to handle its own more complicated logic for link
flags from its build script can make this crate do nothing by using:

```toml
[dependencies]
link-cplusplus = { version = "1.0", features = ["nothing"] }
```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
</sub>
