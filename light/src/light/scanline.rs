use rayon::iter::{plumbing::UnindexedConsumer, IntoParallelIterator, ParallelIterator};

pub struct Scanline {
    pub x: usize,
    pub y: usize,
    pub width: usize,
}

pub struct ScanlineIterator {
    pub left: usize,
    pub top: usize,
    pub width: usize,
    pub height: usize,
}

impl ScanlineIterator {
    pub fn new(left: usize, top: usize, width: usize, height: usize) -> Self {
        ScanlineIterator {
            left,
            top,
            width,
            height,
        }
    }
}

impl ParallelIterator for ScanlineIterator {
    type Item = Scanline;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        let ScanlineIterator {
            left,
            top,
            width,
            height,
        } = self;

        (0..height)
            .into_par_iter()
            .map(|row| {
                let x = left;
                let y = top + row;
                Scanline { x, y, width }
            })
            .drive_unindexed(consumer)
    }
}
