use anyhow::{Result, anyhow};
use ilios_types::{camera::Camera, world::World};
use std::{fs, path::Path};

use crate::{
    config_loader::ConfigLoader,
    loaders::{FileLoader, Loader, ZipLoader},
};
mod config_loader;
mod config_types;
mod loaders;
pub mod parsers;

#[derive(Debug)]
pub struct SceneDescriptor {
    pub camera: Camera,
    pub world: World,
}

pub fn load_folder(path: &str) -> Result<SceneDescriptor> {
    let file_loader = FileLoader::new(path);
    load_scene(file_loader)
}

pub fn load_zip_file(path: &str) -> Result<SceneDescriptor> {
    let data: Vec<u8> = fs::read(path)?;
    load_zip_data(data)
}

pub fn load_zip_data(data: Vec<u8>) -> Result<SceneDescriptor> {
    let zip_loader = ZipLoader::new(data)?;
    load_scene(zip_loader)
}

fn load_scene<L: Loader + 'static>(loader: L) -> Result<SceneDescriptor> {
    let config_loader = ConfigLoader::new(loader);
    let camera = config_loader.camera()?;
    let world = config_loader.world()?;

    Ok(SceneDescriptor { camera, world })
}

pub fn load(path: &str) -> Result<SceneDescriptor> {
    let p = Path::new(path);
    let metadata = fs::metadata(p).unwrap();
    let file_type = metadata.file_type();
    if file_type.is_dir() {
        return load_folder(path);
    } else if file_type.is_file() {
        if let Some(ext) = p.extension()
            && ext.to_ascii_lowercase() == "zip"
        {
            return load_zip_file(path);
        }
    }
    Err(anyhow!("file type not supported: {}", path))
}
