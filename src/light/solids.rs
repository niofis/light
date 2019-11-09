use crate::light::material::*;
use crate::light::primitive::*;
use crate::light::transform::*;
use crate::light::vector::*;
use std::f32;

pub fn cube() -> Vec<Primitive> {
    let trs = vec![
        Transform::rotate(0.0, f32::consts::PI / 4.0, f32::consts::PI / 4.0),
        Transform::scale(3.0, 3.0, 3.0),
        Transform::translate(0.0, 0.0, 0.0),
    ];
    let tr = Transform::combine(&trs);
    let pt1 = || tr.apply(&Vector(-0.5, 0.5, -0.5));
    let pt2 = || tr.apply(&Vector(0.5, 0.5, -0.5));
    let pt3 = || tr.apply(&Vector(0.5, -0.5, -0.5));
    let pt4 = || tr.apply(&Vector(-0.5, -0.5, -0.5));
    let pt5 = || tr.apply(&Vector(-0.5, 0.5, 0.5));
    let pt6 = || tr.apply(&Vector(0.5, 0.5, 0.5));
    let pt7 = || tr.apply(&Vector(0.5, -0.5, 0.5));
    let pt8 = || tr.apply(&Vector(-0.5, -0.5, 0.5));
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
