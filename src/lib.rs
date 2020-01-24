//! # `-lstdc++` or `-lc++`
//!
//! This crate exists for the purpose of passing `-lstdc++` or `-lc++` to the
//! linker, while making it possible for an application to make that choice on
//! behalf of its library dependencies.
//!
//! Without this crate, a library would need to:
//!
//! - pick one or the other to link, with no way for downstream applications to
//!   override the choice;
//! - or link neither and require an explicit link flag provided by downstream
//!   applications even if they would be fine with a default choice;
//!
//! neither of which are good experiences.
//!
//! <br>
//!
//! # Options
//!
//! An application or library that is fine with either of libstdc++ or libc++
//! being linked, whichever is the platform's default, should use:
//!
//! ```toml
//! [dependencies]
//! link-cplusplus = "1.0"
//! ```
//!
//! An application that wants a particular one or the other linked should use:
//!
//! ```toml
//! [dependencies]
//! link-cplusplus = { version = "1.0", features = ["libstdcxx"] }
//!
//! # or
//!
//! link-cplusplus = { version = "1.0", features = ["libcxx"] }
//! ```
//!
//! An application that wants to handle its own more complicated logic for link
//! flags from its build script can make this crate do nothing by using:
//!
//! ```toml
//! [dependencies]
//! link-cplusplus = { version = "1.0", features = ["nothing"] }
//! ```
