use super::ChildStream;
use crate::ChildItem;
use futures::task::{Context, Poll};
use futures::Stream;
use std::pin::Pin;

impl Stream for ChildStream {
    type Item = std::io::Result<ChildItem>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mutself = Pin::into_inner(self);
        Stream::poll_next(Pin::new(&mut mutself.stream), cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.stream.size_hint()
    }
}
