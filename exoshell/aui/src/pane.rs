use crate::Rect;
use std::ops::Deref;

#[derive(Debug)]
pub struct Pane<LineMeta> {
    rect: Rect<u16>,
    buf: Vec<(LineMeta, String)>,
}

impl<M, T> From<T> for Pane<M>
where
    Rect<u16>: From<T>,
{
    fn from(rect: T) -> Self {
        let rect = Rect::from(rect);
        Pane { rect, buf: vec![] }
    }
}

impl<T> Deref for Pane<T> {
    type Target = Rect<u16>;

    fn deref(&self) -> &Self::Target {
        &self.rect
    }
}

impl<LineMeta> Pane<LineMeta> {
    pub fn append_line<S>(&mut self, meta: LineMeta, line: S) -> anyhow::Result<()>
    where
        S: Into<String>,
        LineMeta: std::fmt::Debug,
    {
        use anyhow::anyhow;

        let (width, height) = self.rect.convert_into().into();
        let line = line.into();
        assert!(self.buf.len() <= height);
        if self.buf.len() == height {
            Err(anyhow!(
                "cannot append {meta:#?} {line:#?} to full pane {self:#?}"
            ))
        } else if line.chars().count() > width {
            Err(anyhow!(
                "cannot append {meta:#?} {line:#?} which is longer than the width {}",
                self.width()
            ))
        } else {
            self.buf.push((meta, line));
            Ok(())
        }
    }
}
