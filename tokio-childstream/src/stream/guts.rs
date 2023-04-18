use super::ChildStream;
use crate::{ByteSource, StreamItem};
use futures::channel::mpsc;
use futures::stream::{Chain, Stream};
use std::pin::Pin;
use tokio::io::AsyncRead;
use tokio::process::Child;

pub(super) type InnerStream = Chain<mpsc::UnboundedReceiver<StreamItem>, ExitStream>;
pub(super) type ExitStream = Pin<Box<dyn Stream<Item = StreamItem>>>;

pub(super) fn from_child(mut child: Child) -> ChildStream {
    use crate::ChildEvent;
    use futures::StreamExt;

    let id = child.id().unwrap();
    let (sender, receiver) = mpsc::unbounded();

    create_optional_send_task(sender.clone(), child.stdout.take(), ByteSource::Stdout);
    create_optional_send_task(sender, child.stderr.take(), ByteSource::Stderr);

    let exitstream: ExitStream = Box::pin(futures::stream::once(async move {
        child.wait().await.map(ChildEvent::Exit)
    }));

    let stream = receiver.chain(exitstream);

    ChildStream { id, stream }
}

fn create_optional_send_task<R>(
    sender: mpsc::UnboundedSender<StreamItem>,
    optout: Option<R>,
    source: ByteSource,
) where
    R: AsyncRead + Unpin + Send + 'static,
{
    use crate::ChildEvent::Output;
    use futures::StreamExt;
    use tokio_util::io::ReaderStream;

    if let Some(out) = optout {
        tokio::task::spawn(async move {
            let mut stream = ReaderStream::new(out);
            while let Some(bytesres) = stream.next().await {
                sender
                    .unbounded_send(bytesres.map(|b| Output(source, b)))
                    .unwrap();
            }
        });
    }
}
