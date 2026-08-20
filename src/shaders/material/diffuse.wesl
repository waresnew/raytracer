import package::raytracer::Ray;
import package::hittable::dispatch::HitResult;
import package::structs::Material;
import package::rng;
/// assume hit_result is valid
fn scatter_diffuse(material:Material, hit_result:HitResult)->Ray {
    // lambertian distribution
    let dir = cosine_weighted_vector(hit_result.normal);
    return Ray(
        true,
        hit_result.point,
        dir,
        material.colour * hit_result.ray.attenuation,
    );
}
fn cosine_weighted_vector(normal: vec3f) ->vec3f {
    loop {
        let rand_unit = rng::rand_unit();
        let cos_theta = dot(rand_unit,normal);
        if dot(rand_unit,normal) > 0.0 && rng::rand_frac() < cos_theta {
            return rand_unit;
        }
    }
    return vec3f(0.0); //wgsl can't infer that this is unreachable
}
