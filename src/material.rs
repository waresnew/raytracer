use crate::material::{diffuse::Diffuse, diffuse_light::DiffuseLight, glass::Glass, metal::Metal};

pub mod diffuse;
pub mod diffuse_light;
pub mod glass;
pub mod metal;
#[derive(Clone, Copy)]
pub enum Material {
    Diffuse(Diffuse),
    Metal(Metal),
    Glass(Glass),
    DiffuseLight(DiffuseLight),
}
