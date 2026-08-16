use crate::{
    hittable::{HitResult, Hittable},
    ray::Ray,
};

pub struct World {
    objects: Vec<Box<dyn Hittable>>,
}
impl World {
    pub fn from_objects(objects: Vec<Box<dyn Hittable>>) -> Self {
        Self { objects }
    }
    pub fn ray_hit(&self, ray: Ray) -> Option<HitResult> {
        let mut ans = None;
        let mut ans_dis = f32::MAX;
        for object in &self.objects {
            if let Some(hit_result) = object.ray_hit(ray) {
                let dis = hit_result.point.distance_squared(ray.point);
                if dis < ans_dis {
                    ans_dis = dis;
                    ans = Some(hit_result);
                }
            }
        }
        ans
    }
}
