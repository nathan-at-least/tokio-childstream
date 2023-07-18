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
/// # Line Buffering
///
/// If line-buffering is enabled via [ChildStream::new] or
/// [CommandExt::spawn_stream](crate::CommandExt::spawn_stream), each
/// returned [ChildEvent::Output](crate::ChildEvent::Output) bytes is guaranteed to terminate with
/// `'\n'` except the last.
///
/// If the child output terminates with `'\n'` then the last
/// [ChildEvent::Output](crate::ChildEvent::Output) will contain an empty [Bytes](bytes::Bytes).
///
/// Without line-buffering, the [Bytes](bytes::Bytes) items contain an unspecified
/// segmentation of child output.
///
/// The `From<tokio::process::Child>` impl does not use line-buffering.
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
    /// Convert a [Child] to a [ChildStream] with `line_buffering` optionally enabled
    ///
    /// See [ChildStream] for a description of line-buffering.
    pub fn new(child: Child, line_buffering: bool) -> Self {
        self::guts::from_child(child, line_buffering)
    }

    /// Return the OS id of the child
    ///
    /// ⚠ Warning: This is invalid after the child exits  and may refer to a different arbitrary
    /// process on some OSes. This may occur prior to [ChildEvent::Exit](crate::ChildEvent::Exit) is yielded.
    pub fn id(&self) -> u32 {
        self.id
    }
}

impl From<Child> for ChildStream {
    fn from(child: Child) -> Self {
        Self::new(child, false)
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
