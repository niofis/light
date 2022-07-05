#[derive(Copy, Clone)]
pub struct Section {
    pub left: u32,
    pub top: u32,
    pub width: u32,
    pub height: u32,
}

impl Section {
    pub fn new(left: u32, top: u32, width: u32, height: u32) -> Section {
        Section {
            left,
            top,
            width,
            height,
        }
    }
}
