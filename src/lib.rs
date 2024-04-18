#![crate_name = "fast_list"]

#![doc = include_str!("../README.md")]

mod linked_list;
mod walker;

pub use linked_list::*;

#[cfg(feature = "unstable")]
pub use walker::*;
