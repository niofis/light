use crate::{
    ilios::{float::PI, solids::Solid},
    Color, LightSource, Material, Point, Transform, World,
};

pub fn cornell() -> World {
    let mut world_builder = World::builder();

    let simple_sphere = Solid::Sphere(
        10,
        Transform::combine(&[
            Transform::scale(5.0, 5.0, 5.0),
            Transform::translate(16.0, -2.0, 10.0),
        ]),
        Material::Reflective(Color(0.0, 0.0, 1.0), 1.0),
    );

    world_builder.add_object(simple_sphere);

    // cube thingy
    let cube_trs = vec![
        Transform::rotate(0.0, PI / 4.0, PI / 4.0),
        Transform::scale(3.0, 3.0, 3.0),
        Transform::translate(10.0, -5.0, -20.0),
    ];
    let cube = Solid::Cube(Transform::combine(&cube_trs));
    world_builder.add_object(cube);

    //cornell box
    let cornell_trs = vec![
        Transform::scale(42.0, 30.0, 50.0),
        Transform::translate(0.0, 7.5, 0.0),
    ];
    let cornell = Solid::CornellBox(Transform::combine(&cornell_trs));
    world_builder.add_object(cornell);

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
    world_builder.add_object(donut);

    let geo_trs = Transform::combine(&[
        Transform::scale(3.0, 3.0, 3.0),
        Transform::translate(10.0, 10.0, -10.0),
    ]);
    let geo = Solid::Sphere(20, geo_trs, Material::blue());
    world_builder.add_object(geo);

    world_builder.add_light(LightSource::Point(Point(0.0, 15.0, 0.0), 100.0));
    world_builder.add_light(LightSource::Point(Point(0.0, 10.0, -50.0), 100.0));

    let top_light_trs = Transform::combine(&[
        Transform::scale(30.0, 10.0, 10.0),
        Transform::rotate(0.0, 0.0, 0.0),
        Transform::translate(0.0, 22.4, -15.0),
    ]);
    let top_light = Solid::Plane(
        top_light_trs,
        Material::Emissive(crate::ilios::color::WHITE * 1.),
    );
    world_builder.add_object(top_light);

    let light_sphere_2 = Solid::Sphere(
        10,
        Transform::combine(&[
            Transform::scale(2.0, 2.0, 2.0),
            Transform::translate(0., -4., -1.0),
        ]),
        Material::Emissive(crate::ilios::color::WHITE * 1.),
    );
    world_builder.add_object(light_sphere_2);

    let tmp = 5.0;
    let cube_trs = vec![
        Transform::scale(tmp, tmp, tmp),
        Transform::translate(-21. + tmp / 2., 22.5 - tmp / 2., 25. - tmp / 2.),
    ];
    let corner_cube = Solid::InvertedCube(Transform::combine(&cube_trs));
    world_builder.add_object(corner_cube);

    let corner_cube_light = Solid::Sphere(
        10,
        Transform::combine(&[
            Transform::scale(tmp / 2.1, tmp / 2.1, tmp / 2.1),
            Transform::translate(-21. + tmp / 2., 22.5 - tmp / 2., 25. - tmp / 2.),
        ]),
        Material::Emissive(crate::ilios::color::WHITE * 5.),
    );
    world_builder.add_object(corner_cube_light);

    let glass_sphere = Solid::Sphere(
        10,
        Transform::combine(&[
            Transform::scale(2.0, 2.0, 2.0),
            Transform::translate(-16.0, -5.0, -10.0),
        ]),
        Material::Refractive,
    );
    world_builder.add_object(glass_sphere);

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
        Material::Reflective(Color(0.0, 0.0, 1.0), 1.0),
    );
    world_builder.add_object(simple_sphere);

    let simple_triangle = Solid::Triangle(
        Point(-800.0, -7.0, -800.0),
        Point(0.0, -7.0, 800.0),
        Point(800.0, -7.0, -800.0),
        Material::Diffuse(Color(1.0, 1.0, 1.0)),
    );
    world_builder.add_object(simple_triangle);

    // cube thingy
    let cube_trs = vec![
        Transform::rotate(0.0, PI / 4.0, PI / 4.0),
        Transform::scale(3.0, 3.0, 3.0),
        Transform::translate(-10.0, -2.0, 0.0),
    ];
    let cube = Solid::Cube(Transform::combine(&cube_trs));
    world_builder.add_object(cube);

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
    world_builder.add_object(donut);

    let sphere_trs = vec![
        Transform::scale(2.0, 2.0, 2.0),
        Transform::translate(-16.0, -2.0, 10.0),
    ];
    let geo_sphere = Solid::Sphere(20, Transform::combine(&sphere_trs), Material::blue());
    world_builder.add_object(geo_sphere);

    world_builder.add_light(LightSource::Point(Point(-10.0, 10.0, -10.0), 100.0));

    let top_light_trs = Transform::combine(&[
        Transform::scale(30.0, 10.0, 10.0),
        Transform::rotate(0.0, 0.0, 0.0),
        Transform::translate(0.0, 22.4, -15.0),
    ]);
    let top_light = Solid::Plane(
        top_light_trs,
        Material::Emissive(crate::ilios::color::WHITE * 1.),
    );
    world_builder.add_object(top_light);

    let glass_sphere = Solid::Sphere(
        10,
        Transform::combine(&[
            Transform::scale(2.0, 2.0, 2.0),
            Transform::translate(0.0, -5.0, -7.0),
        ]),
        Material::Refractive,
    );
    world_builder.add_object(glass_sphere);

    world_builder.build()
}

pub fn shader_bench() -> World {
    World::builder()
        .add_light(LightSource::Point(Point(0.0, 0.0, -10.0), 100.0))
        .add_object(Solid::Triangle(
            Point(-100.0, -100.0, 0.0),
            Point(0.0, 100.0, 0.0),
            Point(100.0, -100.0, 0.0),
            Material::Diffuse(Color(1.0, 1.0, 1.0)),
        ))
        .build()
}
