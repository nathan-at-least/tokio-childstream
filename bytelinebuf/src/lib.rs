//! `Iterator` and `Stream` types for splitting bytes on `'\n'`
mod buf;

pub use self::buf::{ByteLineBuf, IntoIter};

#[cfg(any(test, feature = "testutil"))]
pub mod testutil;
