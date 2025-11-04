use ilios_types::world::World;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use std::fs;
use std::path::Path;
pub mod parsers;

#[derive(Deserialize, Debug)]
enum TransformType {
    #[serde(rename = "rotate")]
    Rotate,
    #[serde(rename = "translate")]
    Translate,
}

#[derive(Deserialize, Debug)]
struct TransformConfig {
    #[serde(rename = "type")]
    r#type: TransformType,
    values: [f32; 3],
}

#[derive(Deserialize, Debug)]
struct CameraConfig {
    eye: [f32; 3],
    #[serde(rename = "leftBottom")]
    left_bottom: [f32; 3],
    #[serde(rename = "leftTop")]
    left_top: [f32; 3],
    #[serde(rename = "rightTop")]
    right_top: [f32; 3],
    transforms: Vec<TransformConfig>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
enum SolidConfig {
    #[serde(rename = "torus")]
    Torus {
        radius1: f32,
        radius2: f32,
        steps1: u32,
        steps2: u32,
        transforms: Vec<TransformConfig>,
        material: String,
    },
    #[serde(rename = "sphere")]
    Sphere {
        radius: f32,
        sections: f32,
        material: String,
        transforms: Vec<TransformConfig>,
    },
    #[serde(rename = "cube")]
    Cube { transforms: Vec<TransformConfig> },
    #[serde(rename = "cornellBox")]
    CornellBox { transforms: Vec<TransformConfig> },
    #[serde(rename = "plane")]
    Plane {
        transforms: Vec<TransformConfig>,
        material: String,
    },
    #[serde(rename = "ply")]
    Ply {
        file: String,
        transforms: Vec<TransformConfig>,
        material: String,
    },
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
enum MaterialConfig {
    #[serde(rename = "diffuse")]
    Diffuse { color: [f32; 3], id: String },
    #[serde(rename = "emissive")]
    Emissive { color: [f32; 3], id: String },
    #[serde(rename = "reflective")]
    Reflective { color: [f32; 3], id: String },
    #[serde(rename = "refractive")]
    Refractive { color: [f32; 3], id: String },
}

#[derive(Deserialize, Debug)]
struct SceneConfig {
    materials: Vec<MaterialConfig>,
    solids: Vec<SolidConfig>,
}

#[derive(Deserialize, Debug)]
struct GeometryConfig {
    width: u32,
    height: u32,
}

#[derive(Deserialize, Debug)]
struct RendererConfig {
    threads: Option<u32>,
    samples: u32,
    geometry: GeometryConfig,
}

struct FileLoader {
    base: String,
}

impl FileLoader {
    fn new(base: &str) -> FileLoader {
        FileLoader {
            base: base.to_string(),
        }
    }

    fn load_file(&self, file_name: &str) -> Option<String> {
        let base = Path::new(&self.base);
        fs::read_to_string(base.join(file_name)).ok()
    }
}

struct ConfigLoader {
    file_loader: FileLoader,
}

impl ConfigLoader {
    fn new(file_loader: FileLoader) -> ConfigLoader {
        ConfigLoader { file_loader }
    }

    fn camera(&self) -> Option<CameraConfig> {
        self.file_loader
            .load_file("camera.json")
            .map(deserialize)
            .unwrap()
        //.map(|s| serde_json::from_str(&s).unwrap())
    }

    fn renderer(&self) -> Option<RendererConfig> {
        self.file_loader
            .load_file("renderer.json")
            .map(deserialize)
            .unwrap()
    }

    fn scene(&self) -> Option<SceneConfig> {
        self.file_loader
            .load_file("scene.json")
            .map(deserialize)
            .unwrap()
    }
}

fn read_file(base: &Path, file_name: &str) -> Option<String> {
    fs::read_to_string(base.join(file_name)).ok()
}

fn deserialize<T>(buffer: String) -> Option<T>
where
    T: DeserializeOwned,
{
    serde_json::from_str(buffer.as_ref()).ok()
}

pub fn load_folder(path: &str) -> Option<World> {
    let base = Path::new(path);
    let camera_json = read_file(base, "camera.json").unwrap();
    println!("camera = {:?}", camera_json);
    //let deserialized: CameraConfig = deserialize(&camera_json).unwrap();
    //println!("camera deserialized = {:?}", deserialized);

    //let scene: SceneConfig = read_file(base, "scene.json")
    //    .map(|s| deserialize(&s))
    //    .unwrap();
    //println!("scene = {:?}", scene);

    let file_loader = FileLoader::new(path);
    let config_loader = ConfigLoader::new(file_loader);
    let camera = config_loader.camera().unwrap();
    println!("camera = {:?}", camera);

    let renderer = config_loader.renderer().unwrap();
    println!("renderer = {:?}", renderer);

    let scene = config_loader.scene().unwrap();
    println!("scene = {:?}", scene);
    None
}

pub fn load_zip_file(_path: &str) -> Option<World> {
    None
}

pub fn load_zip_blob(_blob: &[u8]) -> Option<World> {
    None
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
