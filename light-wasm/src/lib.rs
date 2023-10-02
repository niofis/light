use light::{Camera, Point, Renderer};

static mut WIDTH: i32 = 0;
static mut HEIGHT: i32 = 0;
static mut LEN: usize = 0;

#[no_mangle]
pub unsafe fn render(ptr: *mut u8) {
    let mut bytes: Vec<u8> = Vec::from_raw_parts(ptr, LEN * 3, LEN * 3);

    let mut offset = 0;
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            bytes[offset] = i as u8;
            bytes[offset + 1] = j as u8;
            offset += 3;
        }
    }

    std::mem::forget(bytes);
}

#[no_mangle]
pub unsafe fn init(width: i32, height: i32) -> *mut u8 {
    WIDTH = width;
    HEIGHT = height;
    LEN = (width * height) as usize;
    let mut buffer = Vec::with_capacity(3 * LEN);
    let ptr = buffer.as_mut_ptr();
    std::mem::forget(buffer);
    let mut renderer = Renderer::builder();
    let v_offset = 3.0;
    let z_offset = -10.0;
    renderer
        .width(width as u32)
        .height(height as u32)
        .camera(Camera::new(
            Point(0.0, 9.0 / 2.0 + v_offset, -60.0 - z_offset),
            Point(-8.0, 9.0 + v_offset, -50.0 - z_offset),
            Point(-8.0, 0.0 + v_offset, -50.0 - z_offset),
            Point(8.0, 9.0 + v_offset, -50.0 - z_offset),
        ));
    ptr
}

#[no_mangle]
pub unsafe fn deinit(ptr: *mut u8) {
    let _bytes: Vec<u8> = Vec::from_raw_parts(ptr, LEN * 3, LEN * 3);
}
