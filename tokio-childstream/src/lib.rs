#![doc = include_str!("../README.md")]

mod commandext;
mod event;
mod stream;

pub use self::commandext::CommandExt;
pub use self::event::{ChildEvent, OutputSource};
pub use self::stream::{ChildStream, StreamItem};

#[cfg(test)]
mod tests;
