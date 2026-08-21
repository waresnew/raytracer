import package::structs::Hittable;
import package::hittable::dispatch::{HitResult, invalid_hit_result};
import package::raytracer::{Ray, ray_at};

fn ray_hit_sphere(sphere:Hittable, ray:Ray, t_min:f32, t_max:f32)->HitResult {
    let centre=sphere.centre_or_start;
    let a = dot(ray.dir, ray.dir);
    let b = 2.0 * dot(ray.dir,ray.point - centre);
    let c = dot(centre, centre) - 2.0 * dot(centre,ray.point)
        + dot(ray.point,ray.point)
        - sphere.radius * sphere.radius;
    let discrim = b * b - 4.0 * a * c;
    if discrim < 0.0 {
        return invalid_hit_result();
    }
    let t_small = (-b - sqrt(discrim)) / (2.0 * a);
    let t_large = (-b + sqrt(discrim)) / (2.0 * a);
    var t=t_small;
    // test both t's and pick the smallest valid one. need to test t_large as well since
    // in the case of a refracted ray inside the sphere, t_small will be <0
    if t<t_min || t>t_max {
        t=t_large;
    }
    if t<t_min || t>t_max {
        return invalid_hit_result();
    }
    let hit_point = ray_at(ray, t);
    var normal = normalize(hit_point - centre);
    var back_face = false;
    if dot(ray.dir,normal) > 0.0 {
        normal = -normal;
        back_face = true;
    }
    return HitResult(
        true,
        hit_point,
        normal,
        sphere.material,
        t,
        ray,
        back_face,
    );
}
