use crate::light::vector::*;

type Matrix = [f32; 16];
pub struct Transform(pub Matrix);

fn combine(mts: &Vec<Matrix>) -> Matrix {
    let roxcol = |a: &Matrix, b: &Matrix, r: usize, c: usize| {
        a[0 + r * 4] * b[0 + c]
            + a[1 + r * 4] * b[4 + c]
            + a[2 + r * 4] * b[8 + c]
            + a[3 + r * 4] * b[12 + c]
    };
    mts.iter().rev().fold(
        [
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ],
        |a, b| {
            let rc = |r, c| roxcol(&a, b, r, c);
            [
                rc(0, 0),
                rc(0, 1),
                rc(0, 2),
                rc(0, 3),
                rc(1, 0),
                rc(1, 1),
                rc(1, 2),
                rc(1, 3),
                rc(2, 0),
                rc(2, 1),
                rc(2, 2),
                rc(2, 3),
                rc(3, 0),
                rc(3, 1),
                rc(3, 2),
                rc(3, 3),
            ]
        },
    )
}

impl Transform {
    pub fn combine(trs: &Vec<Transform>) -> Transform {
        let mts = trs.iter().map(|t| t.0).collect();
        Transform(combine(&mts))
    }

    pub fn rotate(x: f32, y: f32, z: f32) -> Transform {
        let (sx, cx) = x.sin_cos();
        let (sy, cy) = y.sin_cos();
        let (sz, cz) = z.sin_cos();

        let matrix = combine(
            &(vec![
                [
                    cz, -sz, 0.0, 0.0, sz, cz, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
                ],
                [
                    1.0, 0.0, 0.0, 0.0, 0.0, cx, -sx, 0.0, 0.0, sx, cx, 0.0, 0.0, 0.0, 0.0, 1.0,
                ],
                [
                    cy, 0.0, sy, 0.0, 0.0, 1.0, 0.0, 0.0, -sy, 0.0, cy, 0.0, 0.0, 0.0, 0.0, 1.0,
                ],
            ]),
        );
        Transform(matrix)
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Transform {
        let matrix: Matrix = [
            x, 0.0, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 0.0, z, 0.0, 0.0, 0.0, 0.0, 1.0,
        ];
        Transform(matrix)
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Transform {
        let matrix: Matrix = [
            1.0, 0.0, 0.0, x, 0.0, 1.0, 0.0, y, 0.0, 0.0, 1.0, z, 0.0, 0.0, 0.0, 1.0,
        ];
        Transform(matrix)
    }

    pub fn apply(&self, pt: &Vector) -> Vector {
        let Vector(x, y, z) = pt;
        let Transform(m) = self;
        Vector(
            m[0] * x + m[1] * y + m[2] * z + m[3],
            m[4] * x + m[5] * y + m[6] * z + m[7],
            m[8] * x + m[9] * y + m[10] * z + m[11],
        )
    }
}
