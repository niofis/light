// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

use light::{
    demos, parsers, Accelerator, Algorithm, Camera, Color, Point, RenderMethod, Renderer, Section,
    Transform,
};
use tauri::ipc::Response;
use std::sync::Mutex;

const width: u32 = 640;
const height: u32 = 360;
const bpp: u32 = 4;
static renderer_global: Mutex<Option<Renderer>> = Mutex::new(None);
static frames_count: Mutex<f32> = Mutex::new(0.0);
static frames_acc_global: Mutex<Vec<Color>> = Mutex::new(vec![]);

#[tauri::command()]
fn initialize_renderer(json: String) {
    let mut frames_acc = frames_acc_global.lock().unwrap();
    let mut renderer = renderer_global.lock().unwrap();
    let (camera, world) = parsers::json(&json);
    let section = Section::new(0, 0, width, height);
    let mut renderer_builder = Renderer::builder();
    renderer_builder
        .width(width)
        .height(height)
        .algorithm(Algorithm::PathTracing)
        .render_method(RenderMethod::Tiles)
        .accelerator(Accelerator::BoundingVolumeHierarchy)
        .camera(camera)
        .world(world);

    renderer.replace(renderer_builder.build());
    *frames_acc = vec![Color(0.0, 0.0, 0.0); (width * height) as usize];
    let mut count = frames_count.lock().unwrap();
    *count = 0.0;
}

#[tauri::command(async)]
fn generate_image() -> Response {
    let mut frames_acc = frames_acc_global.lock().unwrap();
    let mut renderer = renderer_global.lock().unwrap();

    let mut img = vec![0; (width * height * bpp) as usize];
    if renderer.is_none() {
        return tauri::ipc::Response::new(img);
    }

    let section = Section::new(0, 0, width, height);
    let pixels = renderer.as_mut().unwrap().render(&section);
    let mut count = frames_count.lock().unwrap();
    *count = *count + 1.0;

    for (idx, pixel) in pixels.into_iter().enumerate() {
        let px = frames_acc[idx] + pixel;
        frames_acc[idx] = px;
        let px = px / *count;
        let x = section.left + (idx as u32 % section.width);
        let y = section.top + (idx as u32 / section.width);
        let offset = (y * (width) + x) * 4;
        let (red, green, blue) = (px).as_gamma_corrected_rgb_u8();
        img[offset as usize] = blue;
        img[(offset + 1) as usize] = green;
        img[(offset + 2) as usize] = red;
        img[(offset + 3) as usize] = 0xff;
    }

    tauri::ipc::Response::new(img)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![initialize_renderer, generate_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
