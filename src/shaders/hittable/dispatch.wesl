import package::structs::{Hittable, HITTABLE_SPHERE, HITTABLE_PARALLELOGRAM, Material};
import package::hittable::sphere;
import package::raytracer::Ray;
import package::hittable::parallelogram;
import package::utils::Rgb;
struct HitResult {
    valid:bool,

    //valid
    point: vec3f,
    /// convention: always points against ray_dir
    normal: vec3,
    material: Material,
    t: f32,
    ray: Ray,
    back_face: bool,
}
fn invalid_hit_result() {
    return HitResult(false,vec3f(0.0), vec3f(0.0),Material(0,Rgb(0.0), 0.0), 0.0, Ray(false,vec3f(0.0),vec3f(0.0)), false);
}
fn dispatch_ray_hittable(hittable:Hittable,ray:Ray, t_min:f32, t_max:f32)->HitResult {
    switch hittable.tag {
        case HITTABLE_SPHERE {
            return sphere::ray_hit_sphere(hittable, ray, t_min, t_max);
        }
        case HITTABLE_PARALLELOGRAM {
            return parallelogram::ray_hit_parallelogram(hittable, ray, t_min, t_max);
        }
        default {
            //unreachable
        }
    }
}
