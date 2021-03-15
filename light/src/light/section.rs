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
