use crate::{hittable::HitResult, material::Material, ray::Ray, rgb::Rgb};

#[derive(Debug, Clone, Copy)]
pub struct DiffuseLight {
    pub colour: Rgb,
}
impl DiffuseLight {
    pub fn new(colour: Rgb) -> Self {
        Self { colour }
    }
}
impl Material for DiffuseLight {
    fn scatter_ray(&self, _hit_result: &HitResult) -> Option<Ray> {
        None
    }
    fn emit_light(&self) -> Rgb {
        self.colour
    }

    fn clone_mat(&self) -> Box<dyn Material> {
        Box::new(*self)
    }
}
