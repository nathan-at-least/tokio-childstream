#![doc = include_str!("../README.md")]

mod stream;

pub use self::stream::{child_stream, spawn_stream, ChildItem};

#[cfg(test)]
mod tests;
