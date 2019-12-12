use core::pin::Pin;
use core::task::{Context, Poll};
use tokio::io::{AsyncRead, Result};

#[derive(Debug)]
pub struct OptAsyncRead<R>(Option<R>);

impl<R> From<Option<R>> for OptAsyncRead<R> {
    fn from(optr: Option<R>) -> OptAsyncRead<R> {
        OptAsyncRead(optr)
    }
}

impl<R: AsyncRead + Unpin> AsyncRead for OptAsyncRead<R> {
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context, buf: &mut [u8]) -> Poll<Result<usize>> {
        match self.get_mut().0 {
            None => Poll::Ready(Ok(0)),
            Some(ref mut r) => AsyncRead::poll_read(Pin::new(r), cx, buf),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::OptAsyncRead;
    use tokio::io::Result;

    #[tokio::test]
    async fn read_none() -> Result<()> {
        use tokio::io::AsyncReadExt;

        let mut oar: OptAsyncRead<&[u8]> = OptAsyncRead::from(None);
        let mut buf: Vec<u8> = vec![0, 0, 0];
        let readcount = oar.read(&mut buf).await?;
        assert_eq!(0, readcount);

        Ok(())
    }
}
