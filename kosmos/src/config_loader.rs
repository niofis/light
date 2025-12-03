use anyhow::{Result, anyhow};
use ilios_types::{
    camera::Camera,
    geometry::{Point, Triangle},
    material::Material,
    solids::Solid,
    transform::Transform,
    world::World,
};
use serde::de::DeserializeOwned;
use std::{collections::HashMap, sync::Arc};

use crate::{
    config_types::{self, CameraConfig, SceneConfig, TransformConfig},
    loaders::Loader,
    parsers,
};

pub struct ConfigLoader {
    loader: Box<dyn Loader>,
}
impl ConfigLoader {
    pub fn new<L: Loader + 'static>(loader: L) -> ConfigLoader {
        ConfigLoader {
            loader: Box::new(loader),
        }
    }

    pub fn camera(&self) -> Result<Camera> {
        let config = self
            .loader
            .load("camera.json")
            .map(deserialize::<CameraConfig>)??;
        let CameraConfig {
            eye,
            left_bottom,
            left_top,
            right_top,
        }: CameraConfig = config;

        Ok(Camera::new(
            eye.into(),
            left_top.into(),
            left_bottom.into(),
            right_top.into(),
        ))
    }

    pub fn world(&self) -> Result<World> {
        let into_transform = |cfg: &TransformConfig| -> Transform {
            match cfg {
                TransformConfig::Rotate { values } => {
                    Transform::rotate(values[0], values[1], values[2])
                }
                TransformConfig::Translate { values } => {
                    Transform::translate(values[0], values[1], values[2])
                }
                TransformConfig::Scale { values } => {
                    Transform::scale(values[0], values[1], values[2])
                }
            }
        };
        let from_transforms = |trs: &[TransformConfig]| {
            Transform::combine(&trs.iter().map(into_transform).collect::<Vec<Transform>>())
        };

        let config = self
            .loader
            .load("scene.json")
            .map(deserialize::<SceneConfig>)??;

        let SceneConfig { materials, solids } = config;
        let mut builder = World::builder();
        let mut materials_hash: HashMap<String, Arc<Material>> = HashMap::default();
        for material in materials.iter() {
            match material {
                config_types::MaterialConfig::Diffuse { color, id } => {
                    let mt = Material::Diffuse(color.into());
                    materials_hash.insert(id.to_string(), Arc::new(mt.clone()));
                    builder.add_material(id, mt);
                }
                config_types::MaterialConfig::Emissive { color, id } => {
                    let mt = Material::Emissive(color.into());
                    materials_hash.insert(id.to_string(), Arc::new(mt.clone()));
                    builder.add_material(id, mt);
                }
                config_types::MaterialConfig::Reflective { color, id } => {
                    let mt = Material::Reflective(color.into(), 1.0);
                    materials_hash.insert(id.to_string(), Arc::new(mt.clone()));
                    builder.add_material(id, mt);
                }
                config_types::MaterialConfig::Refractive { id } => {
                    let mt = Material::Refractive;
                    materials_hash.insert(id.to_string(), Arc::new(mt.clone()));
                    builder.add_material(id, mt);
                }
            }
        }
        let get_material = |id: &str| -> Result<&Arc<Material>> {
            Ok(materials_hash
                .get(id)
                .ok_or(anyhow!("material not found: {}", id))?)
        };
        for solid in solids.into_iter() {
            let sld = match solid {
                config_types::SolidConfig::Torus {
                    radius1,
                    radius2,
                    steps1,
                    steps2,
                    transforms,
                    material,
                } => {
                    let mt = get_material(&material)?;
                    Solid::Torus(
                        radius1,
                        radius2,
                        steps1 as usize,
                        steps2 as usize,
                        from_transforms(&transforms),
                        mt.clone(),
                    )
                }
                config_types::SolidConfig::Sphere {
                    sections,
                    material,
                    transforms,
                } => {
                    let mt = get_material(&material)?;
                    Solid::Sphere(sections as usize, from_transforms(&transforms), mt.clone())
                }
                config_types::SolidConfig::ColoredCube { transforms } => {
                    Solid::ColoredCube(from_transforms(&transforms))
                }
                config_types::SolidConfig::Cube {
                    material,
                    transforms,
                } => {
                    let mt = get_material(&material)?;
                    Solid::Cube(from_transforms(&transforms), mt.clone())
                }
                config_types::SolidConfig::CornellBox { transforms } => {
                    Solid::CornellBox(from_transforms(&transforms))
                }
                config_types::SolidConfig::Plane {
                    transforms,
                    material,
                } => {
                    let mt = get_material(&material)?;
                    Solid::Plane(from_transforms(&transforms), mt.clone())
                }
                config_types::SolidConfig::Ply {
                    file,
                    transforms,
                    material,
                    normalize,
                } => {
                    let mt = get_material(&material)?;
                    let trn = from_transforms(&transforms);
                    let ply = parsers::ply::parse(&self.loader.load(&file)?);
                    let mut faces: Vec<Vec<Point>> = ply.faces().collect();
                    if normalize.is_some_and(|x| x) {
                        faces = normalize_points(faces);
                    }
                    let triangles = faces
                        .into_iter()
                        .map(|pts: Vec<Point>| Triangle::new(pts[0], pts[1], pts[2], mt.clone()))
                        .collect();
                    Solid::Mesh(trn, triangles)
                }
            };
            builder.add_solid(sld);
        }
        Ok(builder.build())
    }
}

fn deserialize<T>(buffer: String) -> Result<T>
where
    T: DeserializeOwned,
{
    Ok(serde_json::from_str(buffer.as_ref())?)
}

fn normalize_points(mut faces: Vec<Vec<Point>>) -> Vec<Vec<Point>> {
    let mut min = [f32::MAX; 3];
    let mut max = [f32::MIN; 3];
    for face in faces.iter() {
        for pt in face.iter() {
            min[0] = min[0].min(pt.0);
            min[1] = min[1].min(pt.1);
            min[2] = min[2].min(pt.2);
            max[0] = max[0].max(pt.0);
            max[1] = max[1].max(pt.1);
            max[2] = max[2].max(pt.2);
        }
    }
    let neg_centroid = [
        (min[0] + max[0]) / -2.0,
        (min[1] + max[1]) / -2.0,
        (min[2] + max[2]) / -2.0,
    ];
    let diagonal = [max[0] - min[0], max[1] - min[1], max[2] - min[2]];
    let norm =
        (diagonal[0] * diagonal[0] + diagonal[1] * diagonal[1] + diagonal[2] * diagonal[2]).sqrt();

    for face in faces.iter_mut() {
        for pt in face.iter_mut() {
            pt.0 += neg_centroid[0];
            pt.1 += neg_centroid[1];
            pt.2 += neg_centroid[2];
            pt.0 /= norm;
            pt.1 /= norm;
            pt.2 /= norm;
        }
    }
    faces
}
