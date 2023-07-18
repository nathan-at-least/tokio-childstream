//! `Iterator` and `Stream` types for splitting bytes on `'\n'`
mod buf;

pub use self::buf::{ByteLineBuf, IntoIter};

#[cfg(any(test, feature = "stream"))]
mod stream;

#[cfg(any(test, feature = "stream"))]
pub use self::stream::ByteLineStream;

#[cfg(any(test, feature = "testutil"))]
pub mod testutil;
