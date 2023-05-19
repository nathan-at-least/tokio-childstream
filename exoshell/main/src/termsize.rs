#[derive(Copy, Clone, Debug)]
pub struct TermSize {
    cols: u16,
    rows: u16,
}

impl TermSize {
    pub fn new() -> anyhow::Result<Self> {
        let (cols, rows) = crossterm::terminal::size()?;
        let me = TermSize { cols, rows };
        if cols > 0 && rows > 0 {
            Ok(me)
        } else {
            Err(anyhow::anyhow!("invalid terminal size {me:?}"))
        }
    }

    pub fn cols_log(&self) -> u16 {
        self.cols - 1
    }

    pub fn rows(&self) -> u16 {
        self.rows
    }

    pub fn last_row(&self) -> u16 {
        self.rows - 1
    }
}
