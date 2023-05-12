#![doc = include_str!("../README.md")]

mod commandext;
mod event;
mod stream;

pub use self::commandext::CommandExt;
pub use self::event::{ChildEvent, OutputSource};
pub use self::stream::ChildStream;

/// The [ChildStream] items yield a [Result] of either a [ChildEvent] or [std::io::Error]
pub type StreamItem = std::io::Result<ChildEvent>;

#[cfg(test)]
mod tests;
