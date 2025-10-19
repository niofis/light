use crate::{
    float::{Float, PI},
    geometry::{Point, Triangle},
    material::Material,
    transform::Transform,
};

#[derive(Clone, Debug)]
pub enum Solid {
    Triangle(Point, Point, Point, Material),
    Cube(Transform),
    InvertedCube(Transform),
    CornellBox(Transform),
    Sphere(usize, Transform, Material),
    Torus(Float, Float, usize, usize, Transform, Material),
    Mesh(Transform, Vec<Triangle>),
    Plane(Transform, Material),
}

impl Solid {
    pub fn primitives(&self) -> Vec<Triangle> {
        match self {
            Solid::Triangle(pt1, pt2, pt3, material) => {
                vec![Triangle::new(*pt1, *pt2, *pt3, material.clone())]
            }
            Solid::Cube(transform) => cube(transform, false),
            Solid::CornellBox(transform) => cornell_box(transform),
            Solid::Sphere(sc1, transform, material) => sphere(1.0, *sc1, transform, material),
            Solid::Torus(rd1, rd2, sc1, sc2, transform, material) => {
                torus(*rd1, *rd2, *sc1, *sc2, transform, material)
            }
            Solid::Mesh(transform, triangles) => mesh(transform, triangles),
            Solid::Plane(transform, material) => plane(transform, material),
            Solid::InvertedCube(transform) => cube(transform, true),
        }
    }
}

fn cube(transform: &Transform, invert_normals: bool) -> Vec<Triangle> {
    let pt1 = || transform.apply(&Point(-0.5, 0.5, -0.5));
    let pt2 = || transform.apply(&Point(0.5, 0.5, -0.5));
    let pt3 = || transform.apply(&Point(0.5, -0.5, -0.5));
    let pt4 = || transform.apply(&Point(-0.5, -0.5, -0.5));
    let pt5 = || transform.apply(&Point(-0.5, 0.5, 0.5));
    let pt6 = || transform.apply(&Point(0.5, 0.5, 0.5));
    let pt7 = || transform.apply(&Point(0.5, -0.5, 0.5));
    let pt8 = || transform.apply(&Point(-0.5, -0.5, 0.5));
    if invert_normals {
        vec![
            //frontside
            Triangle::new(pt1(), pt4(), pt2(), Material::blue()),
            Triangle::new(pt2(), pt4(), pt3(), Material::blue()),
            ////right
            Triangle::new(pt2(), pt7(), pt6(), Material::magenta()),
            Triangle::new(pt2(), pt3(), pt7(), Material::magenta()),
            //back
            Triangle::new(pt5(), pt6(), pt8(), Material::green()),
            Triangle::new(pt6(), pt7(), pt8(), Material::green()),
            //left
            Triangle::new(pt5(), pt4(), pt1(), Material::yellow()),
            Triangle::new(pt5(), pt8(), pt4(), Material::yellow()),
            //top
            Triangle::new(pt5(), pt2(), pt6(), Material::red()),
            Triangle::new(pt1(), pt2(), pt5(), Material::red()),
            //bottom
            Triangle::new(pt4(), pt8(), pt3(), Material::cyan()),
            Triangle::new(pt3(), pt8(), pt7(), Material::cyan()),
        ]
    } else {
        vec![
            //frontside
            Triangle::new(pt1(), pt2(), pt4(), Material::blue()),
            Triangle::new(pt2(), pt3(), pt4(), Material::blue()),
            ////right
            Triangle::new(pt2(), pt6(), pt7(), Material::magenta()),
            Triangle::new(pt2(), pt7(), pt3(), Material::magenta()),
            //back
            Triangle::new(pt5(), pt8(), pt6(), Material::green()),
            Triangle::new(pt6(), pt8(), pt7(), Material::green()),
            //left
            Triangle::new(pt5(), pt1(), pt4(), Material::yellow()),
            Triangle::new(pt5(), pt4(), pt8(), Material::yellow()),
            //top
            Triangle::new(pt5(), pt6(), pt2(), Material::red()),
            Triangle::new(pt1(), pt5(), pt2(), Material::red()),
            //bottom
            Triangle::new(pt4(), pt3(), pt8(), Material::cyan()),
            Triangle::new(pt3(), pt7(), pt8(), Material::cyan()),
        ]
    }
}

fn cornell_box(transform: &Transform) -> Vec<Triangle> {
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
        Triangle::new(pt6(), pt2(), pt7(), Material::green()),
        Triangle::new(pt7(), pt2(), pt3(), Material::green()),
        //back
        Triangle::new(pt5(), pt6(), pt7(), Material::white()),
        Triangle::new(pt5(), pt7(), pt8(), Material::white()),
        //left
        Triangle::new(pt1(), pt5(), pt8(), Material::red()),
        Triangle::new(pt1(), pt8(), pt4(), Material::red()),
        //top
        Triangle::new(pt6(), pt5(), pt2(), Material::white()),
        Triangle::new(pt5(), pt1(), pt2(), Material::white()),
        //bottom
        Triangle::new(pt3(), pt4(), pt8(), Material::white()),
        Triangle::new(pt7(), pt3(), pt8(), Material::white()),
    ]
}

fn torus(
    rd1: Float,
    rd2: Float,
    sc1: usize,
    sc2: usize,
    transform: &Transform,
    material: &Material,
) -> Vec<Triangle> {
    let pt = Point(0.0, rd1, 0.0);
    let rt1 = 2.0 * PI / (sc1 as Float);
    let rt2 = 2.0 * PI / (sc2 as Float);
    let mut triangles: Vec<Triangle> = Vec::new();
    let mut cur = (0..=sc1)
        .map(|x| Transform::rotate((x as Float) * rt1, 0.0, 0.0).apply(&pt))
        .map(|p| Transform::translate(0.0, 0.0, -rd2).apply(&p))
        .collect::<Vec<Point>>();

    for _ in 0..sc2 {
        let next = cur
            .iter()
            .map(|p| Transform::rotate(0.0, rt2, 0.0).apply(p))
            .collect::<Vec<Point>>();

        for n in 0..sc1 {
            triangles.push(Triangle::new(
                transform.apply(&cur[n]),
                transform.apply(&next[n]),
                transform.apply(&next[n + 1]),
                material.clone(),
            ));
            triangles.push(Triangle::new(
                transform.apply(&next[n + 1]),
                transform.apply(&cur[n + 1]),
                transform.apply(&cur[n]),
                material.clone(),
            ));
        }
        cur = next;
    }

    triangles
}

fn sphere(radius: Float, sc1: usize, transform: &Transform, material: &Material) -> Vec<Triangle> {
    let pt = Point(0.0, radius, 0.0);
    let sc2 = sc1 * 2;
    let rt1 = PI / (sc1 as Float);
    let rt2 = 2.0 * PI / (sc2 as Float);
    let mut triangles: Vec<Triangle> = Vec::new();
    let mut cur = (0..=sc1)
        .map(|x| Transform::rotate((x as Float) * rt1, 0.0, 0.0).apply(&pt))
        .collect::<Vec<Point>>();

    for _ in 0..sc2 {
        let next = cur
            .iter()
            .map(|p| Transform::rotate(0.0, rt2, 0.0).apply(p))
            .collect::<Vec<Point>>();
        for n in 0..sc1 {
            triangles.push(Triangle::new(
                transform.apply(&cur[n]),
                transform.apply(&next[n + 1]),
                transform.apply(&next[n]),
                material.clone(),
            ));
            triangles.push(Triangle::new(
                transform.apply(&next[n + 1]),
                transform.apply(&cur[n]),
                transform.apply(&cur[n + 1]),
                material.clone(),
            ));
        }
        cur = next;
    }

    triangles
}

fn mesh(transform: &Transform, triangles: &[Triangle]) -> Vec<Triangle> {
    triangles
        .iter()
        .map(|t: &Triangle| {
            let Triangle {
                origin,
                pt2,
                pt3,
                material,
                ..
            } = t;
            let a = transform.apply(origin);
            let b = transform.apply(pt2);
            let c = transform.apply(pt3);
            Triangle::new(a, b, c, material.clone())
        })
        .collect()
}

fn plane(transform: &Transform, material: &Material) -> Vec<Triangle> {
    let pt1 = || transform.apply(&Point(-0.5, 0.0, -0.5));
    let pt2 = || transform.apply(&Point(0.5, 0.0, -0.5));
    let pt3 = || transform.apply(&Point(-0.5, 0.0, 0.5));
    let pt4 = || transform.apply(&Point(0.5, 0.0, 0.5));
    vec![
        Triangle::new(pt4(), pt3(), pt2(), material.clone()),
        Triangle::new(pt3(), pt1(), pt2(), material.clone()),
    ]
}
