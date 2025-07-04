//! A crate to implement an algorithm to annotate lines in tracked files with the commits that changed them.
//!
//! ### Terminology
//!
//! * **Blamed File**
//!    - The file as it exists in `HEAD`.
//!    - the initial state with all lines that we need to associate with a *Source File*.
//! * **Source File**
//!    - A file at a version (i.e., commit) that introduces hunks into the final 'image' of the *Blamed File*.
//! * **Suspects**
//!    - The versions of the files that can contain hunks that we could use in the final 'image'
//!    - multiple at the same time as the commit-graph may split up.
//!    - They turn into a *Source File* once we have found an association into the *Blamed File*.
#![deny(rust_2018_idioms, missing_docs)]
#![forbid(unsafe_code)]

mod error;
pub use error::Error;
mod types;
pub use types::{BlameEntry, BlamePathEntry, BlameRanges, Options, Outcome, Statistics};

mod file;
pub use file::function::file;
