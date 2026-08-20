import package::hittable::dispatch::HitResult;
import package::structs::Material;
import package::rng;
import package::utils::{EPSILON,Rgb};
/// assume hit_result is valid
fn emit_diffuse_light(material:Material)->Rgb {
    return material.colour;
}
