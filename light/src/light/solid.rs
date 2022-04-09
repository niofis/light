use crate::{Material, Point, Transform};
use std::f32::consts::PI;

use super::primitive::Primitive;

pub enum Solid {
    Triangle(Point, Point, Point, Material),
    Sphere(Point, f32, Material),
    Cube(Transform),
    CornellBox(Transform),
    GeodesicSphere(f32, usize, Transform),
    Torus(f32, f32, usize, usize, Transform),
    Mesh(Vec<Primitive>),
    File(String, Transform),
}

impl Solid {
    pub fn primitives(&self) -> Vec<Primitive> {
        match self {
            Solid::Triangle(pt1, pt2, pt3, material) => {
                vec![Primitive::new_triangle(*pt1, *pt2, *pt3, material.clone())]
            }
            Solid::Sphere(center, radius, material) => vec![Primitive::Sphere {
                center: *center,
                radius: *radius,
                material: material.clone(),
            }],
            Solid::Cube(transform) => cube(transform),
            Solid::CornellBox(transform) => cornell_box(transform),
            Solid::GeodesicSphere(radius, sc1, transform) => sphere(*radius, *sc1, transform),
            Solid::Torus(rd1, rd2, sc1, sc2, transform) => torus(*rd1, *rd2, *sc1, *sc2, transform),
            Solid::Mesh(primitives) => primitives.clone(),
            Solid::File(filename, transform) => load_file(filename, transform),
        }
    }
}

fn cube(transform: &Transform) -> Vec<Primitive> {
    let pt1 = || transform.apply(&Point(-0.5, 0.5, -0.5));
    let pt2 = || transform.apply(&Point(0.5, 0.5, -0.5));
    let pt3 = || transform.apply(&Point(0.5, -0.5, -0.5));
    let pt4 = || transform.apply(&Point(-0.5, -0.5, -0.5));
    let pt5 = || transform.apply(&Point(-0.5, 0.5, 0.5));
    let pt6 = || transform.apply(&Point(0.5, 0.5, 0.5));
    let pt7 = || transform.apply(&Point(0.5, -0.5, 0.5));
    let pt8 = || transform.apply(&Point(-0.5, -0.5, 0.5));
    vec![
        //frontside
        Primitive::new_triangle(pt1(), pt2(), pt4(), Material::red()),
        Primitive::new_triangle(pt2(), pt3(), pt4(), Material::red()),
        ////right
        Primitive::new_triangle(pt2(), pt6(), pt7(), Material::blue()),
        Primitive::new_triangle(pt2(), pt7(), pt3(), Material::blue()),
        //back
        Primitive::new_triangle(pt5(), pt8(), pt6(), Material::green()),
        Primitive::new_triangle(pt6(), pt8(), pt7(), Material::green()),
        //left
        Primitive::new_triangle(pt5(), pt1(), pt4(), Material::yellow()),
        Primitive::new_triangle(pt5(), pt4(), pt8(), Material::yellow()),
        //top
        Primitive::new_triangle(pt5(), pt6(), pt2(), Material::magenta()),
        Primitive::new_triangle(pt1(), pt5(), pt2(), Material::magenta()),
        //bottom
        Primitive::new_triangle(pt4(), pt3(), pt8(), Material::cyan()),
        Primitive::new_triangle(pt3(), pt7(), pt8(), Material::cyan()),
    ]
}

fn cornell_box(transform: &Transform) -> Vec<Primitive> {
    let pt1 = || transform.apply(&Point(-0.5, 0.5, -0.5));
    let pt2 = || transform.apply(&Point(0.5, 0.5, -0.5));
    let pt3 = || transform.apply(&Point(0.5, -0.5, -0.5));
    let pt4 = || transform.apply(&Point(-0.5, -0.5, -0.5));
    let pt5 = || transform.apply(&Point(-0.5, 0.5, 0.5));
    let pt6 = || transform.apply(&Point(0.5, 0.5, 0.5));
    let pt7 = || transform.apply(&Point(0.5, -0.5, 0.5));
    let pt8 = || transform.apply(&Point(-0.5, -0.5, 0.5));
    vec![
        ////right
        Primitive::new_triangle(pt6(), pt2(), pt7(), Material::green()),
        Primitive::new_triangle(pt7(), pt2(), pt3(), Material::green()),
        //back
        Primitive::new_triangle(pt5(), pt6(), pt7(), Material::white()),
        Primitive::new_triangle(pt5(), pt7(), pt8(), Material::white()),
        //left
        Primitive::new_triangle(pt1(), pt5(), pt8(), Material::red()),
        Primitive::new_triangle(pt1(), pt8(), pt4(), Material::red()),
        //top
        Primitive::new_triangle(
            pt6(),
            pt5(),
            pt2(),
            Material::Emissive(crate::light::color::WHITE),
        ),
        Primitive::new_triangle(
            pt5(),
            pt1(),
            pt2(),
            Material::Emissive(crate::light::color::WHITE),
        ),
        //bottom
        Primitive::new_triangle(pt3(), pt4(), pt8(), Material::white()),
        Primitive::new_triangle(pt7(), pt3(), pt8(), Material::white()),
    ]
}

fn torus(rd1: f32, rd2: f32, sc1: usize, sc2: usize, transform: &Transform) -> Vec<Primitive> {
    let pt = Point(0.0, rd1, 0.0);
    let rt1 = 2.0 * PI / (sc1 as f32);
    let rt2 = 2.0 * PI / (sc2 as f32);
    let mut triangles: Vec<Primitive> = Vec::new();
    let mut cur = (0..=sc1)
        .map(|x| Transform::rotate((x as f32) * rt1, 0.0, 0.0).apply(&pt))
        .map(|p| Transform::translate(0.0, 0.0, -rd2).apply(&p))
        .collect::<Vec<Point>>();

    for _ in 0..sc2 {
        let next = cur
            .iter()
            .map(|p| Transform::rotate(0.0, rt2, 0.0).apply(p))
            .collect::<Vec<Point>>();

        for n in 0..sc1 {
            triangles.push(Primitive::new_triangle(
                transform.apply(&cur[n]),
                transform.apply(&next[n]),
                transform.apply(&next[n + 1]),
                Material::green(),
            ));
            triangles.push(Primitive::new_triangle(
                transform.apply(&next[n + 1]),
                transform.apply(&cur[n + 1]),
                transform.apply(&cur[n]),
                Material::green(),
            ));
        }
        cur = next;
    }

    triangles
}

fn sphere(radius: f32, sc1: usize, transform: &Transform) -> Vec<Primitive> {
    let pt = Point(0.0, radius, 0.0);
    let sc2 = sc1 * 2;
    let rt1 = PI / (sc1 as f32);
    let rt2 = 2.0 * PI / (sc2 as f32);
    let mut triangles: Vec<Primitive> = Vec::new();
    let mut cur = (0..=sc1)
        .map(|x| Transform::rotate((x as f32) * rt1, 0.0, 0.0).apply(&pt))
        .collect::<Vec<Point>>();

    for _ in 0..sc2 {
        let next = cur
            .iter()
            .map(|p| Transform::rotate(0.0, rt2, 0.0).apply(p))
            .collect::<Vec<Point>>();
        for n in 0..sc1 {
            triangles.push(Primitive::new_triangle(
                transform.apply(&cur[n]),
                transform.apply(&next[n + 1]),
                transform.apply(&next[n]),
                Material::yellow(),
            ));
            triangles.push(Primitive::new_triangle(
                transform.apply(&next[n + 1]),
                transform.apply(&cur[n]),
                transform.apply(&cur[n + 1]),
                Material::yellow(),
            ));
        }
        cur = next;
    }

    triangles
}

fn load_file(filename: &str, transform: &Transform) -> Vec<Primitive> {
    let mut triangles: Vec<Primitive> = vec![];
    let bunny_obj = tobj::load_obj(&std::path::Path::new(filename));
    if bunny_obj.is_err() {
        panic!("obj model is not valid!");
    }
    let (models, _) = bunny_obj.unwrap();

    for (_, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        for f in 0..mesh.indices.len() / 3 {
            let i = 3 * f;
            let x = 3 * mesh.indices[i] as usize;
            let pt1 = Point(
                -mesh.positions[x],
                mesh.positions[x + 1],
                mesh.positions[x + 2],
            );
            let x = 3 * mesh.indices[i + 1] as usize;
            let pt2 = Point(
                -mesh.positions[x],
                mesh.positions[x + 1],
                mesh.positions[x + 2],
            );
            let x = 3 * mesh.indices[i + 2] as usize;
            let pt3 = Point(
                -mesh.positions[x],
                mesh.positions[x + 1],
                mesh.positions[x + 2],
            );
            triangles.push(Primitive::new_triangle(
                transform.apply(&pt1),
                transform.apply(&pt3),
                transform.apply(&pt2),
                Material::white(),
            ));
        }
    }
    triangles
}
