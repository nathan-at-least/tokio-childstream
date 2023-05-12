use crate::ChildStream;
use tokio::process::Command;

/// Extend [tokio::process::Command] to enable spawning a child directly into a [ChildStream]
pub trait CommandExt {
    /// Spawn a child process and convert it into a [ChildStream] with `line_buffering` optionally enabled
    ///
    /// See [ChildStream] for a description of line-buffering.
    fn spawn_stream(&mut self, line_buffering: bool) -> std::io::Result<ChildStream>;
}

impl CommandExt for Command {
    fn spawn_stream(&mut self, line_buffering: bool) -> std::io::Result<ChildStream> {
        use std::process::Stdio;

        self.stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map(|c| ChildStream::new(c, line_buffering))
    }
}
