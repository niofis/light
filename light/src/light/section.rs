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
