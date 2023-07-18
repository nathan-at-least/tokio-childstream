use super::{PollNextStep, State};
use bytelinebuf::ByteLineBuf;
use futures::stream::Stream;
use futures::task::{Context, Poll};
use pin_project::pin_project;
use std::pin::Pin;

#[pin_project]
pub(crate) struct Active<S> {
    buf: ByteLineBuf,
    #[pin]
    upstream: S,
}

impl<S> From<S> for Active<S> {
    fn from(upstream: S) -> Self {
        Active {
            buf: ByteLineBuf::default(),
            upstream,
        }
    }
}

impl<S, T, E> Active<S>
where
    S: Stream<Item = Result<T, E>>,
    T: IntoIterator<Item = u8> + From<Vec<u8>>,
{
    pub(super) fn poll_next_state(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> PollNextStep<S, T, E> {
        use Poll::*;
        use PollNextStep::*;

        let projself = self.project();

        if let Some(line) = projself.buf.pop_line() {
            Ret(Poll::Ready(Some(Ok(T::from(line)))))
        } else {
            match projself.upstream.poll_next(cx) {
                Ready(Some(res)) => match res {
                    Ok(bytes) => {
                        projself.buf.extend(bytes);
                        Cont
                    }
                    error => Ret(Ready(Some(error))),
                },
                Ready(None) => {
                    let buf = std::mem::take(projself.buf);
                    NewState(State::WindDown(buf.into_iter()))
                }
                Pending => Ret(Pending),
            }
        }
    }
}
