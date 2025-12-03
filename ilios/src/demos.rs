use std::sync::Arc;

use ilios_types::{
    color::Color, float::PI, geometry::Point, material::Material, solids::Solid,
    transform::Transform, world::World,
};

pub fn cornell() -> World {
    let mut world_builder = World::builder();

    let simple_sphere = Solid::Sphere(
        10,
        Transform::combine(&[
            Transform::scale(5.0, 5.0, 5.0),
            Transform::translate(16.0, -2.0, 10.0),
        ]),
        Arc::new(Material::Reflective(Color(0.0, 0.0, 1.0), 1.0)),
    );

    world_builder.add_solid(simple_sphere);

    // cube thingy
    let cube_trs = vec![
        Transform::rotate(0.0, PI / 4.0, PI / 4.0),
        Transform::scale(3.0, 3.0, 3.0),
        Transform::translate(10.0, -5.0, -20.0),
    ];
    let cube = Solid::ColoredCube(Transform::combine(&cube_trs));
    world_builder.add_solid(cube);

    //cornell box
    let cornell_trs = vec![
        Transform::scale(42.0, 30.0, 50.0),
        Transform::translate(0.0, 7.5, 0.0),
    ];
    let cornell = Solid::CornellBox(Transform::combine(&cornell_trs));
    world_builder.add_solid(cornell);

    //this is a donut
    let donut_trs = vec![
        Transform::rotate(PI / -4.0, PI / -4.0, 0.0),
        Transform::translate(-17.0, -3.5, 2.),
    ];
    let donut = Solid::Torus(
        1.25,
        3.5,
        60,
        100,
        Transform::combine(&donut_trs),
        Material::green(),
    );
    world_builder.add_solid(donut);

    let geo_trs = Transform::combine(&[
        Transform::scale(3.0, 3.0, 3.0),
        Transform::translate(10.0, 10.0, -10.0),
    ]);
    let geo = Solid::Sphere(20, geo_trs, Material::blue());
    world_builder.add_solid(geo);

    let top_light_trs = Transform::combine(&[
        Transform::scale(30.0, 10.0, 10.0),
        Transform::rotate(0.0, 0.0, 0.0),
        Transform::translate(0.0, 22.4, -15.0),
    ]);
    let top_light = Solid::Plane(
        top_light_trs,
        Arc::new(Material::Emissive(ilios_types::color::WHITE * 1.)),
    );
    world_builder.add_solid(top_light);

    let light_sphere_2 = Solid::Sphere(
        10,
        Transform::combine(&[
            Transform::scale(2.0, 2.0, 2.0),
            Transform::translate(0., -4., -1.0),
        ]),
        Arc::new(Material::Emissive(ilios_types::color::WHITE * 1.)),
    );
    world_builder.add_solid(light_sphere_2);

    let tmp = 5.0;
    let cube_trs = vec![
        Transform::scale(tmp, tmp, tmp),
        Transform::translate(-21. + tmp / 2., 22.5 - tmp / 2., 25. - tmp / 2.),
    ];
    let corner_cube = Solid::ColoredCube(Transform::combine(&cube_trs));
    world_builder.add_solid(corner_cube);

    let corner_cube_light = Solid::Sphere(
        10,
        Transform::combine(&[
            Transform::scale(tmp / 2.1, tmp / 2.1, tmp / 2.1),
            Transform::translate(-21. + tmp / 2., 22.5 - tmp / 2., 25. - tmp / 2.),
        ]),
        Arc::new(Material::Emissive(ilios_types::color::WHITE * 5.)),
    );
    world_builder.add_solid(corner_cube_light);

    let glass_sphere = Solid::Sphere(
        10,
        Transform::combine(&[
            Transform::scale(2.0, 2.0, 2.0),
            Transform::translate(-16.0, -5.0, -10.0),
        ]),
        Arc::new(Material::Refractive),
    );
    world_builder.add_solid(glass_sphere);

    world_builder.build()
}

pub fn simple() -> World {
    let mut world_builder = World::builder();

    let simple_sphere = Solid::Sphere(
        10,
        Transform::combine(&[
            Transform::scale(5.0, 5.0, 5.0),
            Transform::translate(16.0, -2.0, 10.0),
        ]),
        Arc::new(Material::Reflective(Color(0.0, 0.0, 1.0), 1.0)),
    );
    world_builder.add_solid(simple_sphere);

    let simple_triangle = Solid::Triangle(
        Point(-800.0, -7.0, -800.0),
        Point(0.0, -7.0, 800.0),
        Point(800.0, -7.0, -800.0),
        Arc::new(Material::Diffuse(Color(1.0, 1.0, 1.0))),
    );
    world_builder.add_solid(simple_triangle);

    // cube thingy
    let cube_trs = vec![
        Transform::rotate(0.0, PI / 4.0, PI / 4.0),
        Transform::scale(3.0, 3.0, 3.0),
        Transform::translate(-10.0, -2.0, 0.0),
    ];
    let cube = Solid::ColoredCube(Transform::combine(&cube_trs));
    world_builder.add_solid(cube);

    //this is a donut
    let donut_trs = vec![Transform::rotate(PI / -4.0, 0.0, 0.0)];
    let donut = Solid::Torus(
        1.5,
        4.0,
        30,
        50,
        Transform::combine(&donut_trs),
        Material::green(),
    );
    world_builder.add_solid(donut);

    let sphere_trs = vec![
        Transform::scale(2.0, 2.0, 2.0),
        Transform::translate(-16.0, -2.0, 10.0),
    ];
    let geo_sphere = Solid::Sphere(20, Transform::combine(&sphere_trs), Material::blue());
    world_builder.add_solid(geo_sphere);

    let top_light_trs = Transform::combine(&[
        Transform::scale(30.0, 10.0, 10.0),
        Transform::rotate(0.0, 0.0, 0.0),
        Transform::translate(0.0, 22.4, -15.0),
    ]);
    let top_light = Solid::Plane(
        top_light_trs,
        Arc::new(Material::Emissive(ilios_types::color::WHITE * 1.)),
    );
    world_builder.add_solid(top_light);

    let glass_sphere = Solid::Sphere(
        10,
        Transform::combine(&[
            Transform::scale(2.0, 2.0, 2.0),
            Transform::translate(0.0, -5.0, -7.0),
        ]),
        Arc::new(Material::Refractive),
    );
    world_builder.add_solid(glass_sphere);

    world_builder.build()
}
