use crate::ByteLineBuf;
use futures::stream::Stream;
use futures::task::{Context, Poll};
use pin_project::pin_project;
use std::pin::Pin;

/// Map a stream of byte containers into a stream of byte lines
///
/// The final item will not have a `\n` terminator. If the final byte
/// of a stream is `\n` then the final item will be `vec![]`.
#[pin_project]
pub struct ByteLineStream<S, I>(#[pin] State<S, I>)
where
    S: Stream<Item = I>,
    I: IntoIterator<Item = u8>;

impl<S, I> From<S> for ByteLineStream<S, I>
where
    S: Stream<Item = I>,
    I: IntoIterator<Item = u8>,
{
    fn from(upstream: S) -> Self {
        ByteLineStream(State::Active {
            buf: ByteLineBuf::default(),
            upstream,
        })
    }
}
impl<S, I> Stream for ByteLineStream<S, I>
where
    S: Stream<Item = I>,
    I: IntoIterator<Item = u8>,
{
    type Item = Vec<u8>;

    // Required method
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.project().0.poll_next(cx)
    }
}

#[pin_project(
    project = StateProj,
    project_replace = StateReplace,
)]
pub enum State<S, I>
where
    S: Stream<Item = I>,
    I: IntoIterator<Item = u8>,
{
    Active {
        buf: ByteLineBuf,
        #[pin]
        upstream: S,
    },
    WindDown {
        buf: ByteLineBuf,
    },
    Complete,
}

impl<S, I> Stream for State<S, I>
where
    S: Stream<Item = I>,
    I: IntoIterator<Item = u8>,
{
    type Item = Vec<u8>;

    // Required method
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        use StateProj::*;

        let optnewstate = {
            if let Active { buf, upstream } = self.as_mut().project() {
                match upstream.poll_next(cx) {
                    Poll::Ready(Some(bytes)) => {
                        buf.extend(bytes);
                        None
                    }
                    Poll::Ready(None) => {
                        let buf = std::mem::take(buf);
                        Some(State::WindDown { buf })
                    }
                    Poll::Pending => None,
                }
            } else {
                None
            }
        };

        if let Some(newstate) = optnewstate {
            self.set(newstate);
        }

        let (ret, optnewstate) = match self.as_mut().project() {
            Active { buf, .. } => {
                if let Some(line) = buf.drain_lines().next() {
                    (Poll::Ready(Some(line)), None)
                } else {
                    (Poll::Pending, None)
                }
            }
            WindDown { buf } => {
                let optline = buf.drain_lines().next();
                if let Some(line) = optline {
                    (Poll::Ready(Some(line)), None)
                } else {
                    (
                        Poll::Ready(Some(std::mem::take(buf).drain_remainder())),
                        Some(State::Complete),
                    )
                }
            }
            Complete => (Poll::Ready(None), None),
        };

        if let Some(newstate) = optnewstate {
            self.set(newstate);
        }

        ret
    }
}
