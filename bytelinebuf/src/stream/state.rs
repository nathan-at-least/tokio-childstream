use crate::ByteLineBuf;
use futures::stream::Stream;
use futures::task::{Context, Poll};
use pin_project::pin_project;
use std::pin::Pin;

#[pin_project(
    project = StateProj,
    project_replace = StateReplace,
)]
pub(super) enum State<S> {
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

impl<'pin, S> StateProj<'pin, S> {
    fn mut_buf(&mut self) -> Option<&mut ByteLineBuf> {
        use StateProj::*;

        match self {
            Active { buf, .. } => Some(buf),
            WindDown { buf } => Some(buf),
            Complete => None,
        }
    }
}

impl<S, T, E> Stream for State<S>
where
    S: Stream<Item = Result<T, E>>,
    T: IntoIterator<Item = u8>,
    T: From<Vec<u8>>,
{
    type Item = Result<T, E>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        use StateProj::*;

        loop {
            let optnewstate = {
                let mut projself = self.as_mut().project();

                // First always ensure buf is drained of any ready lines:
                if let Some(buf) = projself.mut_buf() {
                    if let Some(line) = buf.pop_line() {
                        return Poll::Ready(Some(Ok(T::from(line))));
                    }
                }

                match projself {
                    Active { buf, upstream } => match upstream.poll_next(cx) {
                        // Precondition, buf is line-drained above.
                        Poll::Ready(Some(res)) => match res {
                            Ok(bytes) => {
                                buf.extend(bytes);
                                continue;
                            }
                            error => {
                                return Poll::Ready(Some(error));
                            }
                        },
                        Poll::Ready(None) => {
                            let buf = std::mem::take(buf);
                            Some(State::WindDown { buf }) // optnewstate
                        }
                        Poll::Pending => {
                            return Poll::Pending;
                        }
                    },
                    WindDown { buf } => {
                        dbg!(&buf);
                        if let Some(r) = std::mem::take(buf).drain_remainder() {
                            dbg!(&r);
                            return Poll::Ready(Some(Ok(T::from(r))));
                        } else {
                            Some(State::Complete) // optnewstate
                        }
                    }
                    Complete => {
                        return Poll::Ready(None);
                    }
                }
            };

            if let Some(newstate) = optnewstate {
                self.set(newstate);
            }
        }
    }
}
