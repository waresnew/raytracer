use crate::rgb::Rgb;

#[derive(Debug, Clone, Copy)]
pub struct DiffuseLight {
    pub colour: Rgb,
}
impl DiffuseLight {
    pub fn new(colour: Rgb) -> Self {
        Self { colour }
    }
    pub fn emit_light(&self) -> Rgb {
        self.colour
    }
}
