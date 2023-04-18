use super::ChildStream;
use crate::{ByteSource, StreamItem};
use futures::channel::mpsc;
use futures::{Stream, StreamExt};
use std::pin::Pin;
use tokio::io::AsyncRead;
use tokio::process::Child;

impl From<Child> for ChildStream {
    fn from(mut child: Child) -> Self {
        use crate::{ByteSource::*, ChildItem::*};

        let id = child.id().unwrap();
        let (sender, receiver) = mpsc::unbounded();

        create_optional_send_task(sender.clone(), child.stdout.take(), Stdout);
        create_optional_send_task(sender, child.stderr.take(), Stderr);

        let exitstream: Pin<Box<dyn Stream<Item = StreamItem>>> =
            Box::pin(futures::stream::once(async move {
                child.wait().await.map(Exit)
            }));

        let stream = receiver.chain(exitstream);

        ChildStream { id, stream }
    }
}

fn create_optional_send_task<R>(
    sender: mpsc::UnboundedSender<StreamItem>,
    optout: Option<R>,
    source: ByteSource,
) where
    R: AsyncRead + Unpin + Send + 'static,
{
    use crate::ChildItem::Bytes;
    use tokio_util::io::ReaderStream;

    if let Some(out) = optout {
        tokio::task::spawn(async move {
            let mut stream = ReaderStream::new(out);
            while let Some(bytesres) = stream.next().await {
                sender
                    .unbounded_send(bytesres.map(|b| Bytes(source, b)))
                    .unwrap();
            }
        });
    }
}
