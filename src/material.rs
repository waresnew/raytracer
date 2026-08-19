use crate::{
    hittable::HitResult,
    macros::gen_struct_enum,
    material::{diffuse::Diffuse, diffuse_light::DiffuseLight, glass::Glass, metal::Metal},
    ray::Ray,
    rgb::Rgb,
};

pub mod diffuse;
pub mod diffuse_light;
pub mod glass;
pub mod metal;

gen_struct_enum!(Material {
    Diffuse,
    Metal,
    Glass,
    DiffuseLight
});

pub trait Scatter {
    fn scatter_ray(&self, hit_result: HitResult) -> Option<Ray>;
    fn emit_light(&self) -> Rgb;
}
impl Scatter for Material {
    fn scatter_ray(&self, hit_result: HitResult) -> Option<Ray> {
        match self {
            Material::Diffuse(diffuse) => diffuse.scatter_ray(hit_result),
            Material::Metal(metal) => metal.scatter_ray(hit_result),
            Material::Glass(glass) => glass.scatter_ray(hit_result),
            Material::DiffuseLight(_) => None,
        }
    }

    fn emit_light(&self) -> Rgb {
        match self {
            Material::DiffuseLight(diffuse_light) => diffuse_light.emit_light(),
            _ => Rgb::BLACK,
        }
    }
}
