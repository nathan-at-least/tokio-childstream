mod optasyncread;

use crate::optasyncread::OptAsyncRead;
use core::pin::Pin;
use futures_core::task::{Context, Poll};
use futures_core::Stream;
use tokio::io::{AsyncBufReadExt, BufReader, Lines, Result};
use tokio::process::{Child, ChildStdout};

pub struct ChildStream<'a> {
    substream: Lines<BufReader<OptAsyncRead<&'a mut ChildStdout>>>,
    // err: OptAsyncRead<&'a mut ChildStderr>,
}

pub enum ChildStreamItem {
    OutLine(String),
    ErrLine(String),
}

impl<'a> ChildStream<'a> {
    pub fn new(child: &'a mut Child) -> ChildStream {
        let out = child.stdout();
        let oar = OptAsyncRead::from(out.as_mut());
        let bufread = BufReader::new(oar);
        let lines = bufread.lines();

        ChildStream { substream: lines }
    }
}

impl<'a> Stream for ChildStream<'a> {
    type Item = Result<String>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        Stream::poll_next(Pin::new(&mut self.get_mut().substream), cx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn iter_echo() -> Result<()> {
        use futures_util::stream::StreamExt;
        use std::process::Stdio;

        let mut child = tokio::process::Command::new("echo")
            .arg("foo")
            .arg("bar")
            .stdout(Stdio::piped())
            .spawn()?;
        let mut stream = ChildStream::new(&mut child);
        assert_eq!(stream.next().await.map(|r| r.unwrap()), Some(String::from("foo bar")));
        assert_eq!(stream.next().await.map(|r| r.unwrap()), None);
        Ok(())
    }
}
