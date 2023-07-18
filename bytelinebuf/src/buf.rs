use bytes::Bytes;
use std::collections::VecDeque;

/// Convert arbitrary arbitrary byte sequences into `b'\n'`-terminated `Vec<u8>`
///
/// Insert bytes via the [Extend] impl.
#[derive(Debug, Default)]
pub struct ByteLineBuf {
    lines: VecDeque<Vec<u8>>,
    fragment: Vec<u8>,
}

impl ByteLineBuf {
    /// Remove and return the first complete pending line, if any
    pub fn pop_line(&mut self) -> Option<Vec<u8>> {
        self.lines.pop_front()
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

#[derive(Debug)]
pub struct IntoIter(std::collections::vec_deque::IntoIter<Vec<u8>>);

impl IntoIterator for ByteLineBuf {
    type Item = Vec<u8>;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        let ByteLineBuf {
            mut lines,
            fragment,
        } = self;

        if !fragment.is_empty() {
            lines.push_back(fragment);
        }

        IntoIter(lines.into_iter())
    }
}

impl Iterator for IntoIter {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[cfg(test)]
mod tests;
