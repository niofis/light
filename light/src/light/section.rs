use super::pixel::Pixel;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

#[derive(Copy, Clone)]
pub struct Section {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Section {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Section {
        Section {
            x,
            y,
            width,
            height,
        }
    }
}

pub struct SectionIterator {
    left: u32,
    top: u32,
    width: u32,
    height: u32,
    x: u32,
    y: u32,
    right: u32,
    bottom: u32,
}

impl SectionIterator {
    pub fn new(left: u32, top: u32, width: u32, height: u32) -> Self {
        Self {
            left,
            top,
            width,
            height,
            x: left,
            y: top,
            right: left + width,
            bottom: top + height,
        }
    }
}

impl Iterator for SectionIterator {
    type Item = Pixel;
    fn next(&mut self) -> Option<Self::Item> {
        let SectionIterator {
            left,
            x,
            y,
            right,
            bottom,
            ..
        } = self;
        let new_next = Pixel {
            x: *x as f32,
            y: *y as f32,
        };

        if y == bottom {
            return None;
        }
        self.x += 1;
        if self.x == *right {
            self.x = *left;
            self.y += 1;
        }
        Some(new_next)
    }
}

impl ParallelIterator for SectionIterator {
    type Item = Pixel;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::UnindexedConsumer<Self::Item>,
    {
        let SectionIterator {
            left,
            top,
            width,
            height,
            ..
        } = self;
        (0..width * height)
            .into_par_iter()
            .map(|pixel| {
                let x = (left + pixel % width) as f32;
                let y = (top + pixel / width) as f32;
                Pixel { x, y }
            })
            .drive_unindexed(consumer)
    }
}

impl IndexedParallelIterator for SectionIterator {
    fn len(&self) -> usize {
        (self.width * self.height) as usize
    }

    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::Consumer<Self::Item>,
    {
        let SectionIterator {
            left,
            top,
            width,
            height,
            ..
        } = self;
        (0..width * height)
            .into_par_iter()
            .map(|pixel| {
                let x = (left + pixel % width) as f32;
                let y = (top + pixel / width) as f32;
                Pixel { x, y }
            })
            .drive(consumer)
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: rayon::iter::plumbing::ProducerCallback<Self::Item>,
    {
        let SectionIterator {
            left,
            top,
            width,
            height,
            ..
        } = self;
        (0..width * height)
            .into_par_iter()
            .map(|pixel| {
                let x = (left + pixel % width) as f32;
                let y = (top + pixel / width) as f32;
                Pixel { x, y }
            })
            .with_producer(callback)
    }
}
