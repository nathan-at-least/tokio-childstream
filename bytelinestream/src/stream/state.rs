mod active;

pub(super) use self::active::Active;
use futures::stream::Stream;
use futures::task::{Context, Poll};
use pin_project::pin_project;
use std::pin::Pin;

#[pin_project(
    project = StateProj,
    project_replace = StateReplace,
)]
pub(super) enum State<S> {
    Active(#[pin] Active<S>),
    WindDown(bytelinebuf::IntoIter),
}

impl<S> From<S> for State<S> {
    fn from(inner: S) -> Self {
        State::Active(Active::from(inner))
    }
}

enum PollNextStep<S, T, E> {
    Cont,
    Ret(Poll<Option<Result<T, E>>>),
    NewState(State<S>),
}

impl<S> State<S> {
    fn poll_next_step<T, E>(
        self: &mut Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> PollNextStep<S, T, E>
    where
        S: Stream<Item = Result<T, E>>,
        T: IntoIterator<Item = u8> + From<Vec<u8>>,
    {
        match self.as_mut().project() {
            StateProj::Active(x) => x.poll_next_state(cx),
            StateProj::WindDown(x) => PollNextStep::Ret(Poll::Ready(x.next().map(T::from).map(Ok))),
        }
    }
}

impl<S, T, E> Stream for State<S>
where
    S: Stream<Item = Result<T, E>>,
    T: IntoIterator<Item = u8> + From<Vec<u8>>,
{
    type Item = Result<T, E>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        use PollNextStep::*;

        loop {
            match self.poll_next_step(cx) {
                Cont => {
                    continue;
                }
                Ret(v) => {
                    return v;
                }
                NewState(s) => {
                    self.set(s);
                }
            }
        }
    }
}
