use std::sync::Arc;

use ilios::{Accelerator, Algorithm, RenderMethod, Renderer};
use ilios_types::{
    camera::Camera,
    color::Color,
    geometry::{Point, Vector},
    material::Material,
    section::Section,
    solids::Solid,
    transform::Transform,
    world::World,
};
use kosmos::SceneDescriptor;

static mut WIDTH: i32 = 0;
static mut HEIGHT: i32 = 0;
static mut LEN: usize = 0;
static mut RENDERER: Option<Renderer> = None;
static mut FRAMES_ACC: Option<Vec<Color>> = None;
static mut TOTAL_FRAMES: f32 = 0.0;

#[no_mangle]
pub unsafe fn alloc(size_in_bytes: usize) -> *mut u8 {
    let mut memory = Vec::with_capacity(size_in_bytes);
    let ptr = memory.as_mut_ptr();
    std::mem::forget(memory);
    ptr
}

#[no_mangle]
pub unsafe fn free(n: usize, ptr: *mut f32) {
    let _bytes: Vec<f32> = Vec::from_raw_parts(ptr, n, n);
}

#[cfg(target_arch = "wasm32")]
#[target_feature(enable = "simd128")]
#[no_mangle]
/// # Safety
///
/// This function is unsafe because it dereferences a raw pointer.
pub unsafe fn render(ptr: *mut u8) {
    render_frame(ptr);
}

#[allow(dead_code)]
#[allow(static_mut_refs)]
unsafe fn render_frame(ptr: *mut u8) {
    let mut bytes: Vec<u8> = Vec::from_raw_parts(ptr, LEN * 3, LEN * 3);

    let section = Section::new(0, 0, WIDTH as u32, HEIGHT as u32);

    if let Some(renderer) = &mut RENDERER {
        let pixels = renderer.render(&section);
        TOTAL_FRAMES = TOTAL_FRAMES + 1.0;
        if let Some(frames_acc) = &mut FRAMES_ACC {
            let mut offset = 0;
            for (idx, color) in pixels.iter().enumerate() {
                frames_acc[idx] = frames_acc[idx] + *color;
                let output_color = frames_acc[idx] / TOTAL_FRAMES;
                let (r, g, b) = output_color.as_gamma_corrected_rgb_u8();
                bytes[offset] = r;
                bytes[offset + 1] = g;
                bytes[offset + 2] = b;
                offset += 3;
            }
        }
    }

    std::mem::forget(bytes);
}

#[no_mangle]
/// # Safety
///
/// This function is unsafe because it returns a raw pointer and dereferences it.
pub unsafe fn init(width: i32, height: i32) -> *mut u8 {
    WIDTH = width;
    HEIGHT = height;
    LEN = (width * height) as usize;
    let mut buffer = Vec::with_capacity(3 * LEN);
    let ptr = buffer.as_mut_ptr();
    std::mem::forget(buffer);
    let w = 4.0;
    let h = 3.0;
    FRAMES_ACC = Some(vec![Color::default(); LEN]);
    TOTAL_FRAMES = 0.0;
    RENDERER = Some(
        Renderer::builder()
            .width(width as u32)
            .height(height as u32)
            .camera(Camera::new(
                Point(0.0, 0.0, -65.0),       //eye
                Point(-w, h + 1.0, -50.0),    //left_top
                Point(-4.0, -h + 1.0, -50.0), //left_bottom
                Point(w, h + 1.0, -50.0),     //right_top
            ))
            .algorithm(Algorithm::PathTracing)
            .render_method(RenderMethod::Tiles)
            .accelerator(Accelerator::BoundingVolumeHierarchy)
            .world(
                World::builder()
                    .add_solid(Solid::Torus(
                        2.0,
                        4.0,
                        12,
                        24,
                        Transform::combine(&[Transform::rotate(3.1415926 / 2.0, 0.0, 0.0)]),
                        Material::green(),
                    ))
                    .add_solid(Solid::Plane(
                        Transform::combine(&[
                            Transform::scale(20.0, 0.0, 20.0),
                            Transform::translate(0.0, 10.0, 0.0),
                        ]),
                        Arc::new(Material::Emissive(Color(1.0, 1.0, 1.0))),
                    ))
                    .add_solid(Solid::Plane(
                        Transform::combine(&[
                            Transform::scale(1000.0, 0.0, 1000.0),
                            Transform::rotate(3.1415926, 0.0, 0.0),
                            Transform::translate(0.0, -5.0, 0.0),
                        ]),
                        Arc::new(Material::Diffuse(Color(3.0, 3.0, 3.0))),
                    ))
                    .build(),
            )
            .build(),
    );
    ptr
}

// #[no_mangle]
// pub unsafe fn init_from_json(width: i32, height: i32, str_len: i32, str_ptr: *mut u8) -> *mut u8 {
//     WIDTH = width;
//     HEIGHT = height;
//     LEN = (width * height) as usize;
//     let mut buffer = Vec::with_capacity(3 * LEN);
//     let ptr = buffer.as_mut_ptr();
//     std::mem::forget(buffer);
//     let w = 4.0;
//     let h = 3.0;
//     let str_buffer: Vec<u8> = Vec::from_raw_parts(str_ptr, str_len as usize, str_len as usize);
//     let json = String::from_utf8_lossy(&str_buffer);
//     let (camera, world) = parsers::json(&json);
//     FRAMES_ACC = Some(vec![Color::default(); LEN]);
//     TOTAL_FRAMES = 0.0;
//     RENDERER = Some(
//         Renderer::builder()
//             .width(width as u32)
//             .height(height as u32)
//             .camera(camera)
//             .algorithm(Algorithm::PathTracing)
//             .render_method(RenderMethod::Tiles)
//             .accelerator(Accelerator::BoundingVolumeHierarchy)
//             .world(world)
//             .build(),
//     );
//     ptr
// }

#[allow(static_mut_refs)]
#[no_mangle]
pub unsafe fn camera_rotate(x: f32, y: f32, z: f32) {
    if let Some(renderer) = &mut RENDERER {
        let transform = Transform::rotate(x, y, z);
        renderer.camera.apply_transform(&transform);
        TOTAL_FRAMES = 0.0;
        FRAMES_ACC = Some(vec![Color::default(); LEN]);
    }
}

#[allow(static_mut_refs)]
#[no_mangle]
pub unsafe fn camera_rotate_orbital(x: f32, y: f32, z: f32) {
    if let Some(renderer) = &mut RENDERER {
        let transform = Transform::combine(&[
            Transform::rotate_around_vector(x, renderer.camera.coordinate_system.u),
            Transform::rotate_around_vector(y, renderer.camera.coordinate_system.v),
            Transform::rotate_around_vector(z, renderer.camera.coordinate_system.w),
        ]);
        renderer.camera.apply_transform(&transform);
        TOTAL_FRAMES = 0.0;
        FRAMES_ACC = Some(vec![Color::default(); LEN]);
    }
}

#[allow(static_mut_refs)]
#[no_mangle]
pub unsafe fn camera_zoom(delta: f32) {
    if let Some(renderer) = &mut RENDERER {
        let algo: Vector = renderer.camera.eye.into();
        let algo = algo.unit() * delta;
        let transform = Transform::translate(algo.0, algo.1, algo.2);
        renderer.camera.apply_transform(&transform);
        TOTAL_FRAMES = 0.0;
        FRAMES_ACC = Some(vec![Color::default(); LEN]);
    }
}

#[no_mangle]
/// # Safety
///
/// This function is unsafe because it dereferences a raw pointer.
pub unsafe fn deinit(ptr: *mut u8) {
    let _bytes: Vec<u8> = Vec::from_raw_parts(ptr, LEN * 3, LEN * 3);
}

#[allow(static_mut_refs)]
#[no_mangle]
pub unsafe fn load_scene_zip(zip_lenght: i32, zip_ptr: *mut u8) {
    let bytes: Vec<u8> = Vec::from_raw_parts(zip_ptr, zip_lenght as usize, zip_lenght as usize);
    let SceneDescriptor { camera, world } = kosmos::load_zip_data(bytes).unwrap();

    let renderer = RENDERER.take();
    let mut builder = renderer.unwrap().into_builder();

    builder.camera(camera);
    builder.world(world);
    RENDERER = Some(builder.build());
}
