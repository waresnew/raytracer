use std::ops::Range;

use crate::{
    aabb::Aabb,
    hittable::{HitResult, Hittable},
    ray::Ray,
};

pub enum BvhNode {
    Branch(BvhBranch),
    Leaf(Hittable),
    Empty,
}
pub struct BvhBranch {
    left: Box<BvhNode>,
    right: Box<BvhNode>,
    aabb: Aabb,
}
impl BvhBranch {
    fn new(left: BvhNode, right: BvhNode, aabb: Aabb) -> Self {
        Self {
            left: Box::new(left),
            right: Box::new(right),
            aabb,
        }
    }
}
impl BvhNode {
    pub fn from_objects(mut objects: Vec<Hittable>) -> Self {
        if objects.is_empty() {
            return BvhNode::Empty;
        }
        if objects.len() == 1 {
            return BvhNode::Leaf(objects.pop().unwrap());
        }
        let mut aabb = Aabb::empty_box();
        for object in &objects {
            aabb = Aabb::combine(aabb, Self::hittable_aabb(object));
        }
        let split_axis = aabb.max_range_axis();
        objects.sort_by(|a, b| {
            Self::hittable_aabb(a).min[split_axis]
                .total_cmp(&Self::hittable_aabb(b).min[split_axis])
        });
        let right = objects.split_off(objects.len() / 2);
        let left = objects;
        BvhNode::Branch(BvhBranch::new(
            BvhNode::from_objects(left),
            BvhNode::from_objects(right),
            aabb,
        ))
    }
    pub fn ray_hit(&self, ray: Ray, t_bounds: &Range<f32>) -> Option<HitResult> {
        match self {
            BvhNode::Branch(branch) => {
                if !branch.aabb.ray_intersects(ray, t_bounds) {
                    return None;
                }
                let left = branch.left.ray_hit(ray, t_bounds);
                let right_t_bounds = if let Some(left) = &left {
                    &(t_bounds.start..left.t)
                } else {
                    t_bounds
                };
                let right = branch.right.ray_hit(ray, right_t_bounds);
                match (&left, &right) {
                    (None, None) => None,
                    (Some(_), None) => left,
                    (None, Some(_)) => right,
                    (Some(l), Some(r)) => {
                        if l.t < r.t {
                            left
                        } else {
                            right
                        }
                    }
                }
            }
            BvhNode::Leaf(hittable) => Self::hittable_ray_hit(hittable, ray, t_bounds),
            BvhNode::Empty => None,
        }
    }
    fn hittable_aabb(hittable: &Hittable) -> Aabb {
        match hittable {
            Hittable::Sphere(sphere) => sphere.aabb(),
            Hittable::Parallelogram(parallelogram) => parallelogram.aabb(),
        }
    }
    fn hittable_ray_hit(hittable: &Hittable, ray: Ray, t_bounds: &Range<f32>) -> Option<HitResult> {
        match hittable {
            Hittable::Sphere(sphere) => sphere.ray_hit(ray, t_bounds),
            Hittable::Parallelogram(parallelogram) => parallelogram.ray_hit(ray, t_bounds),
        }
    }
}
