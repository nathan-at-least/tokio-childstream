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

    pub fn convert_into<M>(&self) -> Rect<M>
    where
        M: Copy + From<N>,
    {
        let width = M::from(self.width);
        let height = M::from(self.height);
        Rect { width, height }
    }
}
