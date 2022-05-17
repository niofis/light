use rayon::iter::{plumbing::UnindexedConsumer, IntoParallelIterator, ParallelIterator};

pub struct Tile {
    pub x: u32,
    pub y: u32,
    pub size: u32,
}

pub struct TileIterator {
    pub left: u32,
    pub top: u32,
    pub width: u32,
    pub height: u32,
}

impl TileIterator {
    pub fn new(left: u32, top: u32, width: u32, height: u32) -> Self {
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
