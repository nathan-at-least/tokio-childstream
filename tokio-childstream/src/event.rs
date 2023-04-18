use bytes::Bytes;
use std::process::ExitStatus;

/// Represents events from a [ChildStream](crate::ChildStream)
#[derive(Clone, Debug)]
pub enum ChildEvent {
    /// Output read from the child's stdout/stderr
    Output(OutputSource, Bytes),

    /// The [ExitStatus] of the child
    ///
    /// Note: [ChildStream](crate::ChildStream) ensures this is the last event emitted so long as
    /// there are no [std::io::Error] items yielded.
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
