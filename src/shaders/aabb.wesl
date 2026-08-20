import package::structs::Aabb;
import package::raytracer::Ray;
fn ray_intersects_aabb(aabb:Aabb, ray: Ray, t_min:f32, t_max:f32) -> bool {
    let x_t1 = (aabb.min.x - ray.point.x) / ray.dir.x;
    let x_t2 = (aabb.max.x - ray.point.x) / ray.dir.x;
    let y_t1 = (aabb.min.y - ray.point.y) / ray.dir.y;
    let y_t2 = (aabb.max.y - ray.point.y) / ray.dir.y;
    let z_t1 = (aabb.min.z - ray.point.z) / ray.dir.z;
    let z_t2 = (aabb.max.z - ray.point.z) / ray.dir.z;
    let x_t_min = min(x_t1,x_t2);
    let x_t_max=max(x_t1,x_t2);
    let y_t_min = min(y_t1,y_t2);
    let y_t_max=max(y_t1,y_t2);
    let z_t_min = min(z_t1,z_t2);
    let z_t_max=max(z_t1,z_t2);
    let t_interval_min = max(max(x_t_min,y_t_min),z_t_min);
    let t_interval_max = min(min(x_t_max,y_t_max),z_t_max);
    return t_interval_min <= t_interval_max && t_interval_max >= t_min && t_interval_min <= t_max;
}
