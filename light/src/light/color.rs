use std::ops;

#[derive(Debug, Copy, Clone, Default)]
pub struct Color(pub f32, pub f32, pub f32); //r,g,b

pub const BLACK: Color = Color(0., 0., 0.);
pub const WHITE: Color = Color(1., 1., 1.);

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

impl ops::Div<f32> for Color {
    type Output = Color;
    fn div(self, rhs: f32) -> Color {
        let Color(ar, ag, ab) = self;
        Color(ar / rhs, ag / rhs, ab / rhs)
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

impl Color {
    pub fn as_gamma_corrected_rgb_u8(&self) -> (u8, u8, u8) {
        let Color(red, green, blue) = self;
        (
            (255.0 * (red.powf(1.0 / 2.2))).min(255.0) as u8,
            (255.0 * (green.powf(1.0 / 2.2))).min(255.0) as u8,
            (255.0 * (blue.powf(1.0 / 2.2))).min(255.0) as u8,
        )
    }
}
