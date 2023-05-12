mod guts;

use self::guts::InnerStream;
use crate::StreamItem;
use futures::task::{Context, Poll};
use futures::Stream;
use pin_project::pin_project;
use std::pin::Pin;
use tokio::process::Child;

/// Provide a [Stream](futures::Stream) over [StreamItem]s from a [tokio::process::Child]
///
/// Convert a [tokio::process::Child] with [ChildStream::from].
///
/// To spawn a [ChildStream] directly from [tokio::process::Command] see
/// [CommandExt::spawn_stream](crate::CommandExt::spawn_stream).
#[pin_project]
pub struct ChildStream {
    id: u32,
    #[pin]
    stream: InnerStream,
}

impl ChildStream {
    /// Return the OS id of the child
    ///
    /// ⚠ Warning: This is invalid after the child exits  and may refer to a different arbitrary
    /// process on some OSes. This may occur prior to [ChildEvent::Exit] is yielded.
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
        self.project().stream.poll_next(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.stream.size_hint()
    }
}
