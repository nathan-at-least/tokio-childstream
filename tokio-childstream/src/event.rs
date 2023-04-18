use bytes::Bytes;
use std::process::ExitStatus;

/// Represents events from a [ChildStream](crate::ChildStream)
#[derive(Clone, Debug)]
pub enum ChildEvent {
    /// Output read from the child's stdout/stderr
    Output(OutputSource, Bytes),

    /// The [ExitStatus] of the child
    Exit(ExitStatus),
}

/// Indicate the source of [ChildEvent::Output]
#[derive(Copy, Clone, Debug)]
pub enum OutputSource {
    /// Output read from a child's stdout
    Stdout,
    /// Output read from a child's stderr
    Stderr,
}
