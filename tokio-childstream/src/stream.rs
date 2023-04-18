mod guts;

use self::guts::InnerStream;
use crate::ChildEvent;
use futures::task::{Context, Poll};
use futures::Stream;
use std::pin::Pin;
use tokio::process::Child;

/// Provide a [Stream](futures::Stream) over [std::io::Result]s of [ChildEvent]s
///
/// Convert a [tokio::process::Child] with [ChildStream::from].
///
/// To spawn a [ChildStream] directly from [tokio::process::Command] see
/// [CommandExt::spawn_stream](crate::CommandExt::spawn_stream).
pub struct ChildStream {
    id: u32,
    stream: InnerStream,
}

pub type StreamItem = std::io::Result<ChildEvent>;

impl ChildStream {
    pub fn id(&self) -> u32 {
        self.id
    }
}

impl From<Child> for ChildStream {
    fn from(child: Child) -> Self {
        self::guts::from_child(child)
    }
}

impl Stream for ChildStream {
    type Item = StreamItem;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mutself = Pin::into_inner(self);
        Stream::poll_next(Pin::new(&mut mutself.stream), cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.stream.size_hint()
    }
}
