use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum TransformConfig {
    #[serde(rename = "rotate")]
    Rotate { values: [f32; 3] },
    #[serde(rename = "translate")]
    Translate { values: [f32; 3] },
    #[serde(rename = "scale")]
    Scale { values: [f32; 3] },
}

#[derive(Deserialize, Debug)]
pub struct CameraConfig {
    pub eye: [f32; 3],
    #[serde(rename = "leftBottom")]
    pub left_bottom: [f32; 3],
    #[serde(rename = "leftTop")]
    pub left_top: [f32; 3],
    #[serde(rename = "rightTop")]
    pub right_top: [f32; 3],
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum SolidConfig {
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
        sections: u32,
        material: String,
        transforms: Vec<TransformConfig>,
    },
    #[serde(rename = "cube")]
    Cube {
        material: String,
        transforms: Vec<TransformConfig>,
    },
    #[serde(rename = "coloredCube")]
    ColoredCube { transforms: Vec<TransformConfig> },
    #[serde(rename = "cornellBox")]
    CornellBox { transforms: Vec<TransformConfig> },
    #[serde(rename = "plane")]
    Plane {
        material: String,
        transforms: Vec<TransformConfig>,
    },
    #[serde(rename = "ply")]
    Ply {
        file: String,
        normalize: Option<bool>,
        material: String,
        transforms: Vec<TransformConfig>,
    },
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum MaterialConfig {
    #[serde(rename = "diffuse")]
    Diffuse { color: [f32; 3], id: String },
    #[serde(rename = "emissive")]
    Emissive { color: [f32; 3], id: String },
    #[serde(rename = "reflective")]
    Reflective { color: [f32; 3], id: String },
    #[serde(rename = "refractive")]
    Refractive { id: String },
}

#[derive(Deserialize, Debug)]
pub struct SceneConfig {
    pub materials: Vec<MaterialConfig>,
    pub solids: Vec<SolidConfig>,
}
