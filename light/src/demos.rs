use std::{f32::consts::PI, path::Path};

use crate::{Color, Light, Material, Solid, Transform, Vector, World};

pub fn cornell() -> World {
    let simple_sphere = Solid::Sphere(
        Vector(16.0, -2.0, 10.0),
        5.0,
        Material::Reflective(Color(0.0, 0.0, 1.0), 1.0),
    );
    let simple_triangle = Solid::Triangle(
        Vector(-8.0, 0.0, 0.0),
        Vector(-7.0, 2.0, 0.0),
        Vector(-6.0, 0.0, 0.0),
        Material::Simple(Color(0.0, 1.0, 0.0)),
    );

    // cube thingy
    let cube_trs = vec![
        Transform::rotate(0.0, PI / 4.0, PI / 4.0),
        Transform::scale(3.0, 3.0, 3.0),
        Transform::translate(-10.0, -2.0, 0.0),
    ];
    let cube = Solid::Cube(Transform::combine(&cube_trs)).into();

    //cornell box
    let cornell_trs = vec![
        Transform::scale(42.0, 30.0, 50.0),
        Transform::translate(0.0, 7.5, 0.0),
    ];
    let cornell = Solid::CornellBox(Transform::combine(&cornell_trs)).into();

    //this is a donut
    let donut_trs = vec![Transform::rotate(PI / -4.0, 0.0, 0.0)];
    let donut = Solid::Torus(1.5, 4.0, 30, 50, Transform::combine(&donut_trs)).into();

    let lights = vec![Light::Point(Vector(-10.0, 10.0, -10.0))];

    let world = World::build()
        .lights(lights)
        .objects(vec![simple_sphere, simple_triangle, cube, cornell, donut])
        .finish();

    world
}

pub fn simple() -> World {
    let simple_sphere = Solid::Sphere(
        Vector(16.0, -2.0, 10.0),
        5.0,
        Material::Reflective(Color(0.0, 0.0, 1.0), 1.0),
    );
    let simple_triangle = Solid::Triangle(
        Vector(-800.0, -7.0, -800.0),
        Vector(0.0, -7.0, 800.0),
        Vector(800.0, -7.0, -800.0),
        Material::Simple(Color(1.0, 1.0, 1.0)),
    );

    // cube thingy
    let cube_trs = vec![
        Transform::rotate(0.0, PI / 4.0, PI / 4.0),
        Transform::scale(3.0, 3.0, 3.0),
        Transform::translate(-10.0, -2.0, 0.0),
    ];
    let cube = Solid::Cube(Transform::combine(&cube_trs)).into();

    //this is a donut
    let donut_trs = vec![Transform::rotate(PI / -4.0, 0.0, 0.0)];
    let donut = Solid::Torus(1.5, 4.0, 30, 50, Transform::combine(&donut_trs)).into();
    let sphere_trs = vec![Transform::translate(-16.0, -2.0, 10.0)];
    let geo_sphere = Solid::GeodesicSphere(2.0, 20, Transform::combine(&sphere_trs));
    let lights = vec![Light::Point(Vector(-10.0, 10.0, -10.0))];

    World::build()
        .lights(lights)
        .objects(vec![
            simple_sphere,
            simple_triangle,
            cube,
            donut,
            geo_sphere,
        ])
        .finish()
}

pub fn shader_bench() -> World {
    let lights = vec![Light::Point(Vector(0.0, 0.0, -10.0))];

    World::build()
        .lights(lights)
        .objects(vec![Solid::Triangle(
            Vector(-100.0, -100.0, 0.0),
            Vector(0.0, 100.0, 0.0),
            Vector(100.0, -100.0, 0.0),
            Material::Simple(Color(1.0, 1.0, 1.0)),
        )])
        .finish()
}

pub fn bunny() -> World {
    let mut solids: Vec<Solid> = Vec::new();

    solids.push(Solid::Triangle(
        Vector(-800.0, -7.0, -800.0),
        Vector(0.0, -7.0, 800.0),
        Vector(800.0, -7.0, -800.0),
        Material::Simple(Color(1.0, 1.0, 1.0)),
    ));

    let bunny_obj = tobj::load_obj(&Path::new("models/bunny_res2.obj"));
    if bunny_obj.is_ok() == false {
        panic!("obj model is not valid!");
    }
    let (models, _) = bunny_obj.unwrap();
    let mesh_trs = Transform::combine(&vec![
        Transform::scale(120.0, 120.0, 120.0),
        Transform::translate(0.0, -11.0, 0.0),
    ]);

    for (_, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        for f in 0..mesh.indices.len() / 3 {
            let i = 3 * f;
            let x = 3 * mesh.indices[i] as usize;
            let pt1 = Vector(
                -mesh.positions[x],
                mesh.positions[x + 1],
                mesh.positions[x + 2],
            );
            let x = 3 * mesh.indices[i + 1] as usize;
            let pt2 = Vector(
                -mesh.positions[x],
                mesh.positions[x + 1],
                mesh.positions[x + 2],
            );
            let x = 3 * mesh.indices[i + 2] as usize;
            let pt3 = Vector(
                -mesh.positions[x],
                mesh.positions[x + 1],
                mesh.positions[x + 2],
            );
            solids.push(Solid::Triangle(
                mesh_trs.apply(&pt1),
                mesh_trs.apply(&pt3),
                mesh_trs.apply(&pt2),
                Material::white(),
            ));
        }
    }
    let lights = vec![Light::Point(Vector(0.0, 20.0, -50.0))];

    World::build().lights(lights).objects(solids).finish()
}
