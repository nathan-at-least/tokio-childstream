mod state;

use self::state::State;
use futures::stream::Stream;
use futures::task::{Context, Poll};
use pin_project::pin_project;
use std::pin::Pin;

/// Map a stream of byte container results into a stream of byte lines results
///
/// The final item will not have a `b'\n'` terminator. If the final byte
/// of a stream is `b'\n'` then the final item will be `vec![]`.
#[pin_project]
pub struct ByteLineStream<S>(#[pin] State<S>);

impl<S> From<S> for ByteLineStream<S> {
    fn from(upstream: S) -> Self {
        ByteLineStream(State::from(upstream))
    }
}

impl<S, T, E> Stream for ByteLineStream<S>
where
    S: Stream<Item = Result<T, E>>,
    T: IntoIterator<Item = u8> + From<Vec<u8>>,
{
    type Item = Result<T, E>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.project().0.poll_next(cx)
    }
}

#[cfg(test)]
mod tests;
