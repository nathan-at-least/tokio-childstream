/// Split a string on either newlines or max_width
#[derive(derive_new::new)]
pub(crate) struct FormatRows<'a> {
    max_width: usize,
    buf: &'a str,
}

impl<'a> Iterator for FormatRows<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.is_empty() {
            None
        } else {
            let mut byteix = 0;
            for (i, c) in self.buf.chars().enumerate() {
                if i == self.max_width || c == '\n' {
                    break;
                }
                byteix += c.len_utf8();
            }

            let (item, nextbuf) = self.buf.split_at(byteix);
            self.buf = nextbuf;
            Some(item)
        }
    }
}
