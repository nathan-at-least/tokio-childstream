use crate::ByteLineBuf;
use futures::stream::Stream;
use futures::task::{Context, Poll};
use pin_project::pin_project;
use std::pin::Pin;

/// Map a stream of byte container results into a stream of byte lines results
///
/// The final item will not have a `\n` terminator. If the final byte
/// of a stream is `\n` then the final item will be `vec![]`.
#[pin_project]
pub struct ByteLineStream<S, T, E>(#[pin] State<S, T, E>)
where
    S: Stream<Item = Result<T, E>>,
    T: IntoIterator<Item = u8>,
    T: From<Vec<u8>>;

impl<S, T, E> From<S> for ByteLineStream<S, T, E>
where
    S: Stream<Item = Result<T, E>>,
    T: IntoIterator<Item = u8>,
    T: From<Vec<u8>>,
{
    fn from(upstream: S) -> Self {
        ByteLineStream(State::Active {
            buf: ByteLineBuf::default(),
            upstream,
        })
    }
}
impl<S, T, E> Stream for ByteLineStream<S, T, E>
where
    S: Stream<Item = Result<T, E>>,
    T: IntoIterator<Item = u8>,
    T: From<Vec<u8>>,
{
    type Item = Result<T, E>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.project().0.poll_next(cx)
    }
}

#[pin_project(
    project = StateProj,
    project_replace = StateReplace,
)]
pub enum State<S, T, E>
where
    S: Stream<Item = Result<T, E>>,
    T: IntoIterator<Item = u8>,
    T: From<Vec<u8>>,
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

impl<S, T, E> Stream for State<S, T, E>
where
    S: Stream<Item = Result<T, E>>,
    T: IntoIterator<Item = u8>,
    T: From<Vec<u8>>,
{
    type Item = Result<T, E>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        use StateProj::*;

        let optnewstate = {
            if let Active { buf, upstream } = self.as_mut().project() {
                match upstream.poll_next(cx) {
                    Poll::Ready(optitem) => {
                        if let Some(res) = optitem {
                            match res {
                                Ok(bytes) => {
                                    buf.extend(bytes);
                                    None
                                }
                                error => {
                                    return Poll::Ready(Some(error));
                                }
                            }
                        } else {
                            let buf = std::mem::take(buf);
                            Some(State::WindDown { buf })
                        }
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
                    (Poll::Ready(Some(Ok(T::from(line)))), None)
                } else {
                    (Poll::Pending, None)
                }
            }
            WindDown { buf } => {
                let optline = buf.drain_lines().next();
                if let Some(line) = optline {
                    (Poll::Ready(Some(Ok(T::from(line)))), None)
                } else {
                    (
                        Poll::Ready(
                            Ok(std::mem::take(buf).drain_remainder().map(T::from)).transpose(),
                        ),
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
