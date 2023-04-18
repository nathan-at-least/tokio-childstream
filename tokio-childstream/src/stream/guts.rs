use crate::StreamItem;
use futures::channel::mpsc::UnboundedReceiver;
use futures::stream::{Chain, Stream};
use std::pin::Pin;

pub(super) type InnerStream = Chain<UnboundedReceiver<StreamItem>, ExitStream>;
pub(super) type ExitStream = Pin<Box<dyn Stream<Item = StreamItem>>>;
