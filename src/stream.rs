use bytes::Bytes;
use futures::{Stream, StreamExt};
use std::process::ExitStatus;
use tokio::process::Child;
use tokio::process::Command;
use tokio_util::io::ReaderStream;

/// Spawn `command` and conver the child into a stream of [ChildItem]
pub fn spawn_stream(
    command: &mut Command,
) -> std::io::Result<impl Stream<Item = std::io::Result<ChildItem>>> {
    use std::process::Stdio;

    command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map(child_stream)
}

/// Convert a [Child] into a stream over [ChildItem] events
pub fn child_stream(mut child: Child) -> impl Stream<Item = std::io::Result<ChildItem>> {
    use futures::channel::mpsc;
    use ChildItem::*;

    let optout = child.stdout.take();
    let opterr = child.stderr.take();

    let (sender, receiver) = mpsc::unbounded();

    if let Some(stdout) = optout {
        let outsender = sender.clone();
        tokio::task::spawn(async move {
            let mut stream = ReaderStream::new(stdout);
            while let Some(bytesres) = stream.next().await {
                outsender.unbounded_send(bytesres.map(Stdout)).unwrap();
            }
        });
    }

    if let Some(stderr) = opterr {
        let errsender = sender.clone();
        tokio::task::spawn(async move {
            let mut stream = ReaderStream::new(stderr);
            while let Some(bytesres) = stream.next().await {
                errsender.unbounded_send(bytesres.map(Stderr)).unwrap();
            }
        });
    }

    tokio::task::spawn(async move {
        sender.unbounded_send(child.wait().await.map(Exit)).unwrap();
    });

    receiver
}

/// Represents events from a [Child]
#[derive(Debug)]
pub enum ChildItem {
    /// Bytes read from the child's stdout
    Stdout(Bytes),

    /// Bytes read from the child's stderr
    Stderr(Bytes),

    /// The [ExitStatus] of the child
    Exit(ExitStatus),
}
