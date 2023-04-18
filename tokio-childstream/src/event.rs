use bytes::Bytes;
use std::process::ExitStatus;

/// Represents events from a [ChildStream](crate::ChildStream)
#[derive(Clone, Debug)]
pub enum ChildEvent {
    /// Bytes read from the child
    Output(ByteSource, Bytes),

    /// The [ExitStatus] of the child
    Exit(ExitStatus),
}

/// Indicate the source of child output
#[derive(Copy, Clone, Debug)]
pub enum ByteSource {
    Stdout,
    Stderr,
}
