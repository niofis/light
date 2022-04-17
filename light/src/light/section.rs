use super::pixel::Pixel;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

#[derive(Copy, Clone)]
pub struct Section {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Section {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Section {
        Section {
            x,
            y,
            width,
            height,
        }
    }
}

pub struct SectionIterator {
    left: usize,
    top: usize,
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    right: usize,
    bottom: usize,
}

impl SectionIterator {
    pub fn new(left: usize, top: usize, width: usize, height: usize) -> Self {
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
        self.width * self.height
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
