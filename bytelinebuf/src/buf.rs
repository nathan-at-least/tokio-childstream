use bytes::Bytes;
use std::collections::VecDeque;

/// Convert arbitrary arbitrary byte sequences into `\n`-terminated `Vec<u8>`
///
/// Insert bytes via the [Extend] impl.
#[derive(Debug, Default)]
pub struct ByteLineBuf {
    lines: VecDeque<Vec<u8>>,
    fragment: Vec<u8>,
}

impl ByteLineBuf {
    /// Return an iterator that drains all complete lines, each represented as `Vec<u8>`
    pub fn drain_lines(&mut self) -> DrainLines<'_> {
        DrainLines(self.lines.drain(..))
    }

    /// Convert any bytes remaining in `self` into `Vec<u8>`
    pub fn drain_remainder(self) -> Option<Vec<u8>> {
        if !self.fragment.is_empty() {
            Some(self.fragment)
        } else {
            None
        }
    }

    /// Extend via a single byte
    pub fn extend_byte(&mut self, b: u8) {
        self.fragment.extend(Some(b));
        if b == b'\n' {
            self.lines.push_back(std::mem::take(&mut self.fragment));
        }
    }
}

impl Extend<u8> for ByteLineBuf {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = u8>,
    {
        for b in iter {
            self.extend_byte(b);
        }
    }
}

impl<'a> Extend<&'a u8> for ByteLineBuf {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = &'a u8>,
    {
        for &b in iter {
            self.extend_byte(b);
        }
    }
}

impl Extend<Bytes> for ByteLineBuf {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = Bytes>,
    {
        for bytes in iter {
            self.extend(bytes);
        }
    }
}

/// Drain complete `\n`-terminated lines from a [ByteLineBuf]
pub struct DrainLines<'a>(std::collections::vec_deque::Drain<'a, Vec<u8>>);

impl<'a> Iterator for DrainLines<'a> {
    /// A bytes terminated by `\n`
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
