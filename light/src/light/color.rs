use std::ops;

#[derive(Debug, Clone, Default)]
pub struct Color(pub f32, pub f32, pub f32); //r,g,b

impl ops::Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Color {
        let Color(r, g, b) = self;
        Color(r * rhs, g * rhs, b * rhs)
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Color {
        let Color(ar, ag, ab) = self;
        let Color(br, bg, bb) = rhs;
        Color(ar * br, ag * bg, ab * bb)
    }
}

impl ops::Mul<f32> for &Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Color {
        let Color(ar, ag, ab) = self;
        Color(ar * rhs, ag * rhs, ab * rhs)
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Color {
        let Color(ar, ag, ab) = self;
        let Color(br, bg, bb) = rhs;
        Color(ar + br, ag + bg, ab + bb)
    }
}
