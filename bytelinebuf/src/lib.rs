use bytes::BytesMut;
use std::collections::VecDeque;

#[derive(Debug, Default)]
pub struct ByteLineBuf(VecDeque<u8>);

impl ByteLineBuf {
    pub fn drain_lines(&mut self) -> DrainLines<'_> {
        DrainLines(&mut self.0)
    }

    pub fn drain_remainder(mut self) -> Option<BytesMut> {
        if self.0.is_empty() {
            None
        } else {
            let mut bytes = BytesMut::with_capacity(self.0.len());
            bytes.extend(self.0.drain(..));
            Some(bytes)
        }
    }
}

impl Extend<u8> for ByteLineBuf {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = u8>,
    {
        self.0.extend(iter)
    }
}

/// Drain complete `\n`-terminated lines from a [ByteLineBuf]
pub struct DrainLines<'a>(&'a mut VecDeque<u8>);

impl<'a> Iterator for DrainLines<'a> {
    /// A bytes terminated by `\n`
    type Item = BytesMut;

    fn next(&mut self) -> Option<Self::Item> {
        let itemlen = {
            let (prefix, suffix) = self.0.as_slices();
            if let Some(i) = prefix.iter().position(|&b| b == b'\n') {
                i + 1
            } else if let Some(j) = suffix.iter().position(|&b| b == b'\n') {
                prefix.len() + j + 1
            } else {
                return None;
            }
        };

        let mut bytes = BytesMut::with_capacity(itemlen);
        bytes.extend(self.0.drain(..itemlen));
        Some(bytes)
    }
}
