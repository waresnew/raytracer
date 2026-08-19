import package::raytracer::Ray;
import package::hittable::dispatch::HitResult;
import package::structs::Material;
import package::rng;
import package::utils::EPSILON;
/// assume hit_result is valid
fn scatter_metal(material:Material, hit_result:HitResult)->Ray {
    let reflect_fuzz=material.reflect_fuzz_or_refraction_index;
    let reflected = reflect(hit_result.ray.dir,hit_result.normal);
    var dir = (reflected + rng::rand_unit() * reflect_fuzz);
    if length(dir)<EPSILON {
        dir=reflected;
    } else {
        dir=normalize(dir);
    }
    return Ray(
        true,
        hit_result.point,
        dir,
        material.colour * hit_result.ray.attenuation,
    );
}
