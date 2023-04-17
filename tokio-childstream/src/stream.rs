mod fromchild;
mod streamimpl;

use crate::ChildItem;
use futures::channel::mpsc;

/// Provide a [Stream](futures::Stream) over [std::io::Result]s of [ChildItem]s
///
/// Convert a [tokio::process::Child] with [ChildStream::from].
///
/// To spawn a [ChildStream] directly from [tokio::process::Command] see
/// [CommandExt::spawn_stream](crate::CommandExt::spawn_stream).
#[derive(Debug)]
pub struct ChildStream(mpsc::UnboundedReceiver<std::io::Result<ChildItem>>);
