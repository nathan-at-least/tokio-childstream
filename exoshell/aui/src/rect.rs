use derive_more::{Constructor, From, Into};

#[derive(Clone, Copy, Debug, From, Into, Constructor)]
pub struct Rect<N>
where
    N: Copy,
{
    width: N,
    height: N,
}

impl<N> Rect<N>
where
    N: Copy,
{
    pub fn width(&self) -> N {
        self.width
    }

    pub fn height(&self) -> N {
        self.height
    }

    pub fn convert_into<M>(&self) -> Result<Rect<M>, <M as TryFrom<N>>::Error>
    where
        M: Copy + TryFrom<N>,
    {
        let width = M::try_from(self.width)?;
        let height = M::try_from(self.height)?;
        Ok(Rect { width, height })
    }
}
