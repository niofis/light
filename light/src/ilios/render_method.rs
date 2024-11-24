use super::rng::{Rng, XorRng};
use crate::{Color, Renderer, Section};
use rayon::prelude::*;

#[derive(Clone, Debug)]
pub enum RenderMethod {
    Pixels,
    Tiles,
    Scanlines,
}

pub type TraceFn = fn(&Renderer, &mut dyn Rng, (u32, u32)) -> Color;
pub type RenderFn = fn(&mut Renderer, &Section, TraceFn) -> Vec<Color>;

impl RenderMethod {
    pub fn get(&self) -> RenderFn {
        match self {
            RenderMethod::Pixels => render_pixels,
            RenderMethod::Tiles => render_tiles,
            RenderMethod::Scanlines => render_scanlines,
        }
    }
}

fn render_pixels(renderer: &mut Renderer, section: &Section, trace: TraceFn) -> Vec<Color> {
    let Section {
        left,
        top,
        height,
        width,
    } = section;
    let mut rng = XorRng::new();
    (0..width * height)
        .map(|idx| (left + (idx % width), top + (idx / width)))
        .map(|pixel| trace(renderer, &mut rng, pixel))
        .collect()
}

fn render_tiles(renderer: &mut Renderer, section: &Section, trace: TraceFn) -> Vec<Color> {
    let Section {
        left,
        top,
        height,
        width,
    } = section;
    let tile_size = 4;
    let sections_v = height / tile_size;
    let sections_h = width / tile_size;

    let tiles: Vec<(u32, u32)> = (0..sections_v * sections_h)
        .map(|idx| {
            let x = left + (idx % sections_h) * tile_size;
            let y = top + (idx / sections_h) * tile_size;
            (x, y)
        })
        .collect();
    println!("Threads: {}", rayon::current_num_threads());
    // let mut rng = XorRng::new();
    let tiles = tiles
        .into_par_iter()
        .map_init(XorRng::new, |rng, (x, y)| {
            (0..tile_size * tile_size)
                .map(|idx| (x + (idx % tile_size), y + (idx / tile_size)))
                .map(|pixel| trace(renderer, rng, pixel))
                .collect()
        })
        .collect::<Vec<Vec<Color>>>();

    let mut pixels: Vec<Color> = vec![Color::default(); (width * height) as usize];
    for (section, colors) in tiles.into_iter().enumerate() {
        let start_x = (section as u32 % sections_h) * tile_size;
        let start_y = (section as u32 / sections_h) * tile_size;
        for (idx, color) in colors.into_iter().enumerate() {
            let x = idx as u32 % tile_size;
            let y = idx as u32 / tile_size;
            pixels[((start_y + y) * width + start_x + x) as usize] = color;
        }
    }

    pixels
}

fn render_scanlines(renderer: &mut Renderer, section: &Section, trace: TraceFn) -> Vec<Color> {
    let Section {
        height, width, top, ..
    } = section;

    let mut rng = XorRng::new();

    (0..*height)
        .flat_map(|row| {
            let y = top + row;

            (0..*width)
                .map(|idx| (idx, y))
                .map(|pixel| trace(renderer, &mut rng, pixel))
                .collect::<Vec<Color>>()
        })
        .collect::<Vec<Color>>()
}
