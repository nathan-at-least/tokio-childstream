use bytes::Bytes;
use std::process::ExitStatus;

/// Represents events from a [ChildStream](crate::ChildStream)
#[derive(Debug)]
pub enum ChildItem {
    /// Bytes read from the child's stdout
    Stdout(Bytes),

    /// Bytes read from the child's stderr
    Stderr(Bytes),

    /// The [ExitStatus] of the child
    Exit(ExitStatus),
}
