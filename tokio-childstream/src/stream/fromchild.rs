use super::ChildStream;
use super::StreamItem;
use futures::Stream;
use std::pin::Pin;
use tokio::process::Child;

impl From<Child> for ChildStream {
    fn from(mut child: Child) -> Self {
        use crate::{ByteSource::*, ChildItem::*};
        use futures::channel::mpsc;
        use futures::StreamExt;
        use tokio_util::io::ReaderStream;

        let id = child.id().unwrap();
        let optout = child.stdout.take();
        let opterr = child.stderr.take();

        let (sender, receiver) = mpsc::unbounded();

        if let Some(stdout) = optout {
            let outsender = sender.clone();
            tokio::task::spawn(async move {
                let mut stream = ReaderStream::new(stdout);
                while let Some(bytesres) = stream.next().await {
                    outsender
                        .unbounded_send(bytesres.map(|b| Bytes(Stdout, b)))
                        .unwrap();
                }
            });
        }

        if let Some(stderr) = opterr {
            let errsender = sender;
            tokio::task::spawn(async move {
                let mut stream = ReaderStream::new(stderr);
                while let Some(bytesres) = stream.next().await {
                    errsender
                        .unbounded_send(bytesres.map(|b| Bytes(Stderr, b)))
                        .unwrap();
                }
            });
        }

        let exitstream: Pin<Box<dyn Stream<Item = StreamItem>>> =
            Box::pin(futures::stream::once(async move {
                child.wait().await.map(Exit)
            }));

        let stream = receiver.chain(exitstream);

        ChildStream { id, stream }
    }
}
