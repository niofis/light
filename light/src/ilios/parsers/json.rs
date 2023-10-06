use std::collections::HashMap;

use serde_json::Value;

use crate::{float::Float, Camera, Color, Material, Point, Solid, Transform, Vector, World};

pub fn json(json: &str) -> (Camera, World) {
    let json: Value = serde_json::from_str(json).unwrap();
    let world = parse_world(&json);
    let camera = parse_camera(&json);
    (camera, world)
}

fn parse_camera(json: &Value) -> Camera {
    let camera = &json["camera"];
    let transforms = parse_transforms(camera);
    let mut camera = Camera::new(
        parse_point(&camera["eye"]),
        parse_point(&camera["leftTop"]),
        parse_point(&camera["leftBottom"]),
        parse_point(&camera["rightTop"]),
    );
    camera.apply_transform(&transforms);
    camera
}

fn parse_world(json: &Value) -> World {
    let world = &json["world"];
    let materials = parse_materials(&world["materials"]);
    let objects = parse_objects(&world["objects"], &materials);

    World {
        lights: vec![],
        objects,
        materials,
    }
}

fn parse_materials(json: &Value) -> HashMap<String, Material> {
    let materials = json.as_array().unwrap();
    materials
        .iter()
        .map(|mat| {
            let id = mat["id"].as_str().unwrap();
            let mat_type = mat["type"].as_str().unwrap();
            let material = match mat_type {
                "diffuse" => parse_diffuse(mat),
                "emissive" => parse_emissive(mat),
                "reflective" => parse_reflective(mat),
                "refractive" => parse_refractive(mat),
                _ => panic!("unknown material type: {}", mat_type),
            };
            (id.to_owned(), material)
        })
        .collect()
}

fn parse_diffuse(mat: &Value) -> Material {
    let color = parse_color(&mat["color"]);
    Material::Diffuse(color)
}

fn parse_emissive(mat: &Value) -> Material {
    let color = parse_color(&mat["color"]);
    Material::Emissive(color)
}

fn parse_reflective(mat: &Value) -> Material {
    let color = parse_color(&mat["color"]);
    Material::Reflective(color, 1.0)
}

fn parse_refractive(_mat: &Value) -> Material {
    Material::Refractive
}

fn parse_objects(json: &Value, materials: &HashMap<String, Material>) -> Vec<Solid> {
    let objects = json.as_array().unwrap();
    objects
        .iter()
        .map(|obj| {
            let obj_type = obj["type"].as_str().unwrap();
            match obj_type {
                "sphere" => parse_sphere(obj, materials),
                "cube" => parse_cube(obj),
                "cornellBox" => parse_cornell_box(obj),
                "torus" => parse_torus(obj, materials),
                "plane" => parse_plane(obj, materials),
                _ => panic!("unknown object type: {}", obj_type),
            }
        })
        .collect()
}

fn parse_sphere(sphere: &Value, materials: &HashMap<String, Material>) -> Solid {
    let material_key = sphere["material"].as_str().unwrap();

    Solid::Sphere(
        parse_point(&sphere["center"]),
        parse_float(&sphere["radius"]),
        materials.get(material_key).unwrap().clone(),
    )
}

fn parse_cube(cube: &Value) -> Solid {
    let transforms = parse_transforms(cube);
    Solid::Cube(transforms)
}

fn parse_cornell_box(cornell_box: &Value) -> Solid {
    let transforms = parse_transforms(cornell_box);
    Solid::CornellBox(transforms)
}

fn parse_torus(torus: &Value, materials: &HashMap<String, Material>) -> Solid {
    let transforms = parse_transforms(torus);
    let material_key = torus["material"].as_str().unwrap();
    let radius1 = parse_float(&torus["radius1"]);
    let radius2 = parse_float(&torus["radius2"]);
    let steps1 = parse_usize(&torus["steps1"]);
    let steps2 = parse_usize(&torus["steps2"]);
    Solid::Torus(
        radius1,
        radius2,
        steps1,
        steps2,
        transforms,
        materials.get(material_key).unwrap().clone(),
    )
}

fn parse_plane(plane: &Value, materials: &HashMap<String, Material>) -> Solid {
    let transforms = parse_transforms(plane);
    let material_key = plane["material"].as_str().unwrap();
    Solid::Plane(transforms, materials.get(material_key).unwrap().clone())
}

fn parse_transforms(obj: &Value) -> Transform {
    let transforms = obj["transforms"].as_array().unwrap();
    let transforms: Vec<Transform> = transforms.iter().map(parse_transform).collect();
    Transform::combine(&transforms)
}

fn parse_transform(transform: &Value) -> Transform {
    let type_str = transform["type"].as_str().unwrap();
    let values = parse_vector(&transform["values"]);
    match type_str {
        "rotate" => Transform::rotate(values[0], values[1], values[2]),
        "scale" => Transform::scale(values[0], values[1], values[2]),
        "translate" => Transform::translate(values[0], values[1], values[2]),
        _ => Transform::translate(0.0, 0.0, 0.0),
    }
}

fn parse_point(point: &Value) -> Point {
    let values = point.as_array().unwrap();
    Point(
        parse_float(&values[0]),
        parse_float(&values[1]),
        parse_float(&values[2]),
    )
}

fn parse_vector(vector: &Value) -> Vector {
    let values = vector.as_array().unwrap();
    Vector(
        parse_float(&values[0]),
        parse_float(&values[1]),
        parse_float(&values[2]),
    )
}

fn parse_color(color: &Value) -> Color {
    let values = color.as_array().unwrap();
    Color(
        parse_float(&values[0]),
        parse_float(&values[1]),
        parse_float(&values[2]),
    )
}

fn parse_float(float: &Value) -> Float {
    float.as_f64().unwrap() as Float
}

fn parse_usize(usize: &Value) -> usize {
    usize.as_u64().unwrap() as usize
}
