mod buf;

pub use self::buf::{ByteLineBuf, DrainLines};

#[cfg(feature = "stream")]
mod stream;

#[cfg(feature = "stream")]
pub use self::stream::ByteLineStream;
