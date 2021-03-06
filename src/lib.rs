//! Create, parse, edit, and compare semantic version numbers.
//!
//! This library provides you with nearly everything you need to make use of
//! version numbers that are compatible with version 2.0.0 of the Semantic
//! Versioning specification. The specification is required reading if you do
//! not already know the rules and requirements for a semantic version number.
//! The specification is pretty small, so it shouldn't take too long!
//!
//! Specification
//! -------------
//!
//! You can find the specification at [semver.org][].
//!
//! [semver.org]: https://semver.org/
//!
//! Usage
//! -----
//!
//! The library tries to make working with semantic version numbers as simple
//! as possible. As a result of this effort, there is a lot of things you can
//! do with these numbers. This usage guide will cover high level stuff, so
//! you may want to read the documentation for the structures and traits that
//! are included for more advanced information.
//!
//! ```
//! #[macro_use]
//! extern crate recital;
//!
//! use recital::prelude::*;
//! # fn main() {
//! # }
//! ```
//!
//! ### Creating
//!
//! ```
//! # #[macro_use]
//! # extern crate recital;
//! # fn main() {
//! let version = version!(1, 2, 3,
//!                        vec![id!("abc"), id!(456)],
//!                        vec![id!("def"), id!(789)]);
//! # }
//! ```
//!
//! ### Parsing
//!
//! ```
//! # use recital::version::Version;
//! let version: Version = "1.2.3-abc.456+def.789".parse().unwrap();
//! ```
//!
//! ### Modifying
//!
//! ```
//! # #[macro_use]
//! # extern crate recital;
//! # use recital::version::Version;
//! # fn main() {
//! // `0.0.0`
//! let mut version = Version::new();
//!
//! // `1.2.3`
//! version.major = 1;
//! version.minor = 2;
//! version.patch = 3;
//!
//! // `1.2.3-abc.456+def.789`
//! version.pre.push(id!("abc"));
//! version.pre.push(id!(456));
//! version.build.push(id!("def"));
//! version.build.push(id!(789));
//!
//! // `2.1.1`
//! version.increment_major();
//! version.increment_minor();
//! version.increment_patch();
//! # }
//! ```
//!
//! ### Comparing
//!
//! You can compare versions like you would any number.
//!
//! ```
//! # use recital::version::Version;
//! let a: Version = "1.2.3-alpha".parse().unwrap();
//! let b: Version = "1.2.3".parse().unwrap();
//!
//! assert!(a < b);
//! assert!(!(a > b));
//! assert!(a != b);
//! ```

#[macro_use]
extern crate nom;

// Version number management.
#[macro_use]
pub mod version;

// Version string parser.
mod parser;

/// Re-exports submodules for glob imports.
pub mod prelude {
    pub use resolve::{Constraint, Constraints, Operation, resolve};
    pub use version::{Identifier, Version};
}

// Version constraints management.
#[macro_use]
pub mod resolve;
