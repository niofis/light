use rayon::iter::{plumbing::UnindexedConsumer, IntoParallelIterator, ParallelIterator};

pub struct Tile {
    pub x: usize,
    pub y: usize,
    pub size: usize,
}

pub struct TileIterator {
    pub left: usize,
    pub top: usize,
    pub width: usize,
    pub height: usize,
}

impl TileIterator {
    pub fn new(left: usize, top: usize, width: usize, height: usize) -> Self {
        TileIterator {
            left,
            top,
            width,
            height,
        }
    }
}

impl ParallelIterator for TileIterator {
    type Item = Tile;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        let TileIterator {
            left,
            top,
            width,
            height,
        } = self;
        let tile_size = 16;
        let sections_v = height / tile_size;
        let sections_h = width / tile_size;

        (0..sections_v * sections_h)
            .into_par_iter()
            .map(|idx| {
                let x = left + (idx % sections_h) * tile_size;
                let y = top + (idx / sections_h) * tile_size;
                Tile {
                    x,
                    y,
                    size: tile_size,
                }
            })
            .drive_unindexed(consumer)
    }
}
