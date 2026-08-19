import package::structs::Hittable;
import package::hittable::dispatch::{HitResult, invalid_hit_result};
import package::raytracer::{Ray, ray_at};
import package::utils::EPSILON;

fn ray_hit_parallelogram(parallelogram:Hittable, ray:Ray, t_min:f32, t_max:f32)->HitResult {
    let start=parallelogram.centre_or_start;
    let n = plane_normal(parallelogram);
    let denom = dot(n,ray.dir);
    if abs(denom) < EPSILON {
        return invalid_hit_result(); // no/inf intersection
    }
    let t = dot(-n, ray.point) + dot(n,start) / denom;
    let point = ray_at(ray, t);
    if t>=t_min && t<=t_max && point_in_parallelogram(point, parallelogram) {
        return HitResult (
            true,
            point,
            select(n, -n, dot(ray.dir,n)>0.0), // select(falseValue, trueValue, condition)
            parallelogram.material,
            t,
            ray,
            false,
        );
    } else {
        return invalid_hit_result();
    }
}

fn point_in_parallelogram(point: vec3f, region: Hittable) -> bool {
    // to check if the point is in the face's bounds, represent the point using side1 and
    // side2 as basis vectors. the basis does not (should not) need to be orthogonal and normalized.
    let d = point - region.start;
    let n = plane_normal(region);
    let s = dot(n,cross(region.side2,d)) / dot(n,cross(region.side2,region.side1));
    let t = dot(n,cross(region.side1,d)) / dot(n,cross(region.side1,region.side2));
    return s<=1 && s>=0 && t<=1 && t>=0;
}

fn plane_normal(parallelogram:Hittable)->vec3f {
    return normalize(cross(parallelogram.side1, parallelogram.side2));
}
