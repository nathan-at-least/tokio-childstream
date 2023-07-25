use crate::Rect;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Pane<LineMeta> {
    rect: Rect<u16>,
    content: Content<LineMeta>,
}

#[derive(Debug)]
pub enum Content<M> {
    Full(Vec<(M, String)>),
    Truncated {
        head: Vec<(M, String)>,
        tail: Vec<(M, String)>,
    },
}
use Content::*;

impl<M, T> From<T> for Pane<M>
where
    Rect<u16>: From<T>,
{
    fn from(rect: T) -> Self {
        let rect = Rect::from(rect);
        let content = Full(vec![]);
        Pane { rect, content }
    }
}

impl<LineMeta> Pane<LineMeta> {
    pub fn width(&self) -> usize {
        usize::from(self.rect.width())
    }

    pub fn height(&self) -> usize {
        usize::from(self.rect.height())
    }

    pub fn content_len(&self) -> usize {
        self.content.len()
    }

    pub fn append_line<M, S>(&mut self, meta: M, line: S) -> anyhow::Result<()>
    where
        S: Into<String>,
        LineMeta: From<M> + std::fmt::Debug,
    {
        use anyhow::anyhow;

        let meta = meta.into();
        let line = line.into();
        if line.chars().count() > self.width() {
            Err(anyhow!(
                "cannot append {meta:#?} {line:#?} which is longer than the width {}",
                self.width()
            ))
        } else {
            self.content.append(self.height(), meta, line);
            Ok(())
        }
    }

    /// Return an iterator over contents
    ///
    /// A `None` entry indicates a truncation row.
    pub fn iter(&self) -> impl Iterator<Item = Option<(LineMeta, &str)>> + '_
    where
        LineMeta: Copy,
    {
        ContentIter::from(&self.content)
    }
}

impl<M> Content<M> {
    fn len(&self) -> usize {
        match self {
            Full(b) => b.len(),
            Truncated { head, tail } => head.len() + tail.len() + 1,
        }
    }

    fn append(&mut self, height: usize, meta: M, line: String) {
        self.append_raw(meta, line);
        self.start_truncation_if_necessary(height);
        self.truncate_tail_if_necessary(height);
    }

    fn append_raw(&mut self, meta: M, line: String) {
        let b = match self {
            Full(b) => b,
            Truncated { tail, .. } => tail,
        };
        b.push((meta, line));
    }

    fn start_truncation_if_necessary(&mut self, height: usize) {
        // Now handle truncation if necessary:
        let (height_head, _) = truncation_heights(height);
        let optnewself = match self {
            Full(b) if b.len() == height + 1 => {
                let tail = b.split_off(height_head);
                let head = std::mem::take(b);
                Some(Truncated { head, tail })
            }
            _ => None,
        };
        if let Some(newself) = optnewself {
            *self = newself;
        }
    }

    fn truncate_tail_if_necessary(&mut self, height: usize) {
        if let Truncated { tail, .. } = self {
            let (_, height_tail) = truncation_heights(height);
            tail.drain(tail.len() - height_tail..);
        }
    }
}

/// Return the (head, tail) truncation heights; these exclude a truncation indicator row
fn truncation_heights(height: usize) -> (usize, usize) {
    assert!(height > 0);
    if height == 1 {
        (1, 0)
    } else {
        let head = std::cmp::max(1, (height - 1) / 2);
        let tail = height - head - 1;
        (head, tail)
    }
}

#[doc(hidden)]
pub enum ContentIter<'a, M> {
    Full(VecDeque<&'a (M, String)>),
    Truncated {
        head: VecDeque<&'a (M, String)>,
        tail: VecDeque<&'a (M, String)>,
    },
}

impl<'a, M> From<&'a Content<M>> for ContentIter<'a, M> {
    fn from(c: &'a Content<M>) -> Self {
        match c {
            Full(v) => ContentIter::Full(v.iter().collect()),
            Truncated { head, tail } => ContentIter::Truncated {
                head: head.iter().collect(),
                tail: tail.iter().collect(),
            },
        }
    }
}

impl<'a, M> Iterator for ContentIter<'a, M>
where
    M: Copy,
{
    type Item = Option<(M, &'a str)>;

    fn next(&mut self) -> Option<Self::Item> {
        use ContentIter::*;

        let (optnewself, item) = match self {
            Full(v) => (None, v.pop_front().map(|(m, s)| Some((*m, s.as_str())))),
            Truncated { head, tail } if head.is_empty() => {
                (Some(Full(std::mem::take(tail))), Some(None))
            }
            Truncated { head, .. } => (None, head.pop_front().map(|(m, s)| Some((*m, s.as_str())))),
        };
        if let Some(newself) = optnewself {
            *self = newself;
        }
        item
    }
}

#[cfg(test)]
mod tests;
