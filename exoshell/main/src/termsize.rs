use exoshell_aui::Rect;

pub fn term_size() -> anyhow::Result<Rect<u16>> {
    let dims = crossterm::terminal::size()?;
    Ok(Rect::from(dims))
}
