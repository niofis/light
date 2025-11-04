use crate::{
    float::Float,
    geometry::{Point, Vector},
};

type Matrix = [Float; 16];
const IDENTITY: Matrix = [
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
];

#[derive(Clone, Debug)]
pub struct Transform(pub Matrix);

fn add(mts: &[Matrix]) -> Matrix {
    mts.iter().fold(
        [
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        ],
        |a, b| {
            [
                a[0] + b[0],
                a[1] + b[1],
                a[2] + b[2],
                a[3] + b[3],
                a[4] + b[4],
                a[5] + b[5],
                a[6] + b[6],
                a[7] + b[7],
                a[8] + b[8],
                a[9] + b[9],
                a[10] + b[10],
                a[11] + b[11],
                a[12] + b[12],
                a[13] + b[13],
                a[14] + b[14],
                a[15] + b[15],
            ]
        },
    )
}

fn multiply(mts: &[Matrix]) -> Matrix {
    let multiply_row_by_col = |a: &Matrix, b: &Matrix, r: usize, c: usize| {
        a[r * 4] * b[c]
            + a[1 + r * 4] * b[4 + c]
            + a[2 + r * 4] * b[8 + c]
            + a[3 + r * 4] * b[12 + c]
    };
    mts.iter().rev().fold(
        [
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ],
        |a, b| {
            let rc = |r, c| multiply_row_by_col(&a, b, r, c);
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
    pub fn default() -> Transform {
        Transform(IDENTITY)
    }
    pub fn combine(trs: &[Transform]) -> Transform {
        let mts: Vec<Matrix> = trs.iter().map(|t| t.0).collect();
        Transform(multiply(&mts))
    }

    pub fn rotate(x: Float, y: Float, z: Float) -> Transform {
        let (sx, cx) = x.sin_cos();
        let (sy, cy) = y.sin_cos();
        let (sz, cz) = z.sin_cos();

        let matrix = multiply(
            &([
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

    pub fn rotate_around_vector(o: Float, v: Vector) -> Transform {
        // https://en.wikipedia.org/wiki/Rotation_matrix#Rotation_matrix_from_axis_and_angle
        // https://mathworld.wolfram.com/RodriguesRotationFormula.html
        let (s, c) = o.sin_cos();
        let cr = 1.0 - c;
        let r: Matrix = [
            v.0 * v.0 * cr + c,
            v.0 * v.1 * cr - v.2 * s,
            v.0 * v.2 * cr + v.1 * s,
            0.0,
            v.0 * v.1 * cr + v.2 * s,
            v.1 * v.1 * cr + c,
            v.1 * v.2 * cr - v.0 * s,
            0.0,
            v.0 * v.2 * cr - v.1 * s,
            v.1 * v.2 * cr + v.0 * s,
            v.2 * v.2 * cr + c,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ];

        Transform(r)
    }

    pub fn scale(x: Float, y: Float, z: Float) -> Transform {
        let matrix: Matrix = [
            x, 0.0, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 0.0, z, 0.0, 0.0, 0.0, 0.0, 1.0,
        ];
        Transform(matrix)
    }

    pub fn translate(x: Float, y: Float, z: Float) -> Transform {
        let matrix: Matrix = [
            1.0, 0.0, 0.0, x, 0.0, 1.0, 0.0, y, 0.0, 0.0, 1.0, z, 0.0, 0.0, 0.0, 1.0,
        ];
        Transform(matrix)
    }

    pub fn apply(&self, pt: &Point) -> Point {
        let Point(x, y, z) = pt;
        let Transform(m) = self;
        Point(
            m[0] * x + m[1] * y + m[2] * z + m[3],
            m[4] * x + m[5] * y + m[6] * z + m[7],
            m[8] * x + m[9] * y + m[10] * z + m[11],
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        float::PI,
        geometry::{Point, Vector},
        transform::Transform,
    };

    #[test]
    fn rotation_works() {
        let dg = PI / 100.0;
        let pt = Point(1.0, 1.0, 0.0);
        let expected = Transform::rotate(dg, 0.0, 0.0).apply(&pt);
        let result = Transform::rotate_around_vector(-dg, Vector(1.0, 0.0, 0.0)).apply(&pt);
        assert_eq!(expected, result);
    }
}
