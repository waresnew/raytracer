use std::ops::{Add, AddAssign, Div, Mul};

use glam::Vec3;

#[derive(Debug, Clone, Copy)]
/// f32 for precision; [0,1]
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
impl Rgb {
    pub const BLACK: Rgb = Rgb {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };
    pub const WHITE: Rgb = Rgb {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };
    pub fn from_vec3(vec3: Vec3) -> Self {
        Self {
            r: vec3.x,
            g: vec3.y,
            b: vec3.z,
        }
    }
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }
    pub fn into_raw(self) -> image::Rgb<u8> {
        image::Rgb([
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
        ])
    }
}
impl Add for Rgb {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Rgb {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}
impl AddAssign for Rgb {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl Div<f32> for Rgb {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Rgb {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}
impl Mul<f32> for Rgb {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Rgb {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}
impl Mul for Rgb {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Rgb {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}
