import package::raytracer::Ray;
import package::hittable::dispatch::HitResult;
import package::structs::Material;
import package::rng;
import package::utils::EPSILON;
/// assume hit_result is valid
fn scatter_glass(material:Material, hit_result:HitResult)->Ray {
    let refraction_index=material.reflect_fuzz_or_refraction_index;
    //snell's law
    let n = hit_result.normal;
    let incident = hit_result.ray.dir;
    var incident_perp = (incident - dot(incident,n) * n);
    if length(incident_perp)<EPSILON {
        incident_perp=vec3f(0.0);
    } else {
        incident_perp=normalize(incident_perp);
    }
    let cos_theta = abs(dot(incident,n)); // incident.dot(n) can be -ve
    let sin_theta = sqrt(1.0 - cos_theta*cos_theta);
    var n1=1.0;
    var n2=refraction_index;
    if hit_result.back_face {
        n1=refraction_index;
        n2=1.0;
    }
    let sin_theta_refracted = n1 / n2 * sin_theta;

    var ray_dir=vec3f(0.0);
    if sin_theta_refracted > 1.0
        || rng::rand_frac() < reflect_chance(n1, n2, cos_theta)
    {
        // total internal reflection
        ray_dir=reflect(incident, hit_result.normal);
    } else {
        let refract_perp = incident_perp * sin_theta_refracted;
        let refract_par = sqrt(max(0.0,1.0 - dot(refract_perp,refract_perp))) * -n;
        ray_dir=(refract_perp + refract_par);
        if length(ray_dir)<EPSILON {
            ray_dir=incident;
        } else {
            ray_dir=normalize(ray_dir);
        }
    };
    return Ray(
        true,
        hit_result.point,
        ray_dir,
        hit_result.ray.attenuation,
    );
}
fn reflect_chance(n1: f32, n2: f32, cos_theta: f32) -> f32 {
    // schlick's approximation
    let r0 = pow((n1 - n2) / (n1 + n2),2);
    return r0 + (1.0 - r0) * pow(1.0 - cos_theta,5);
}
