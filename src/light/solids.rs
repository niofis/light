use crate::light::material::*;
use crate::light::primitive::*;
use crate::light::transform::*;
use crate::light::vector::*;

pub fn cube(transform: &Transform) -> Vec<Primitive> {
    let pt1 = || transform.apply(&Vector(-0.5, 0.5, -0.5));
    let pt2 = || transform.apply(&Vector(0.5, 0.5, -0.5));
    let pt3 = || transform.apply(&Vector(0.5, -0.5, -0.5));
    let pt4 = || transform.apply(&Vector(-0.5, -0.5, -0.5));
    let pt5 = || transform.apply(&Vector(-0.5, 0.5, 0.5));
    let pt6 = || transform.apply(&Vector(0.5, 0.5, 0.5));
    let pt7 = || transform.apply(&Vector(0.5, -0.5, 0.5));
    let pt8 = || transform.apply(&Vector(-0.5, -0.5, 0.5));
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

pub fn cornell_box(transform: &Transform) -> Vec<Primitive> {
    let pt1 = || transform.apply(&Vector(-0.5, 0.5, -0.5));
    let pt2 = || transform.apply(&Vector(0.5, 0.5, -0.5));
    let pt3 = || transform.apply(&Vector(0.5, -0.5, -0.5));
    let pt4 = || transform.apply(&Vector(-0.5, -0.5, -0.5));
    let pt5 = || transform.apply(&Vector(-0.5, 0.5, 0.5));
    let pt6 = || transform.apply(&Vector(0.5, 0.5, 0.5));
    let pt7 = || transform.apply(&Vector(0.5, -0.5, 0.5));
    let pt8 = || transform.apply(&Vector(-0.5, -0.5, 0.5));
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
        Primitive::new_triangle(pt6(), pt5(), pt2(), Material::white()),
        Primitive::new_triangle(pt5(), pt1(), pt2(), Material::white()),
        //bottom
        Primitive::new_triangle(pt3(), pt4(), pt8(), Material::white()),
        Primitive::new_triangle(pt7(), pt3(), pt8(), Material::white()),
    ]
}
