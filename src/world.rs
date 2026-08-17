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
        let mut ans_t = f32::MAX;
        for object in &self.objects {
            if let Some(hit_result) = object.ray_hit(ray, 0.001..f32::INFINITY) // min=0.001 to avoid shadow acne (eg. a reflected ray may start 0.00001 inside an object bc float imprecision - want to ignore those)
                && hit_result.t < ans_t
            {
                ans_t = hit_result.t;
                ans = Some(hit_result);
            }
        }
        ans
    }
}
