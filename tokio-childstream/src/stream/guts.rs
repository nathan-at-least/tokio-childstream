use super::ChildStream;
use crate::{OutputSource, StreamItem};
use bytes::Bytes;
use futures::channel::mpsc;
use futures::stream::{Chain, Stream};
use std::pin::Pin;
use tokio::io::AsyncRead;
use tokio::process::Child;

pub(super) type InnerStream = Chain<mpsc::UnboundedReceiver<StreamItem>, ExitStream>;
pub(super) type ExitStream = Pin<Box<dyn Stream<Item = StreamItem> + Send>>;

pub(super) fn from_child(mut child: Child, buffer_lines: bool) -> ChildStream {
    use crate::ChildEvent;
    use futures::StreamExt;

    let id = child.id().unwrap();
    let (sender, receiver) = mpsc::unbounded();

    create_optional_send_task(
        sender.clone(),
        child.stdout.take(),
        OutputSource::Stdout,
        buffer_lines,
    );
    create_optional_send_task(
        sender,
        child.stderr.take(),
        OutputSource::Stderr,
        buffer_lines,
    );

    let exitstream: ExitStream = Box::pin(futures::stream::once(async move {
        child.wait().await.map(ChildEvent::Exit)
    }));

    let stream = receiver.chain(exitstream);

    ChildStream { id, stream }
}

fn create_optional_send_task<R>(
    sender: mpsc::UnboundedSender<StreamItem>,
    optout: Option<R>,
    source: OutputSource,
    buffer_lines: bool,
) where
    R: AsyncRead + Unpin + Send + 'static,
{
    if let Some(out) = optout {
        tokio::task::spawn(async move {
            use bytelinebuf::ByteLineStream;
            use tokio_util::io::ReaderStream;

            let stream = ReaderStream::new(out);

            if buffer_lines {
                stream_sender_loop(sender, source, ByteLineStream::from(stream)).await;
            } else {
                stream_sender_loop(sender, source, stream).await;
            }
        });
    }
}

async fn stream_sender_loop<S>(
    sender: mpsc::UnboundedSender<StreamItem>,
    source: OutputSource,
    mut stream: S,
) where
    S: Stream<Item = std::io::Result<Bytes>> + Unpin,
{
    use crate::ChildEvent::Output;
    use futures::StreamExt;

    while let Some(bytesres) = stream.next().await {
        sender
            .unbounded_send(bytesres.map(|b| Output(source, b)))
            .unwrap();
    }
}
