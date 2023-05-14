use futures::task::{Context, Poll};
use futures::Stream;
use std::pin::Pin;
use tokio::time::{Instant, Interval};

#[derive(Debug)]
pub struct IntervalStream(pub(crate) Interval);

impl Stream for IntervalStream {
    type Item = Instant;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.get_mut().0.poll_tick(cx).map(Some)
    }
}
