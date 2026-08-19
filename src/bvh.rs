use std::ops::Range;

use crate::{
    aabb::Aabb,
    hittable::{Hit, HitResult, Hittable},
    ray::Ray,
};

pub enum BvhNode {
    Empty,
    Leaf(Hittable),
    Branch(BvhBranch),
}
pub struct BvhBranch {
    pub left: Box<BvhNode>,
    pub right: Box<BvhNode>,
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
    pub fn aabb(&self) -> Aabb {
        self.aabb
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
            aabb = Aabb::combine(aabb, object.aabb());
        }
        let split_axis = aabb.max_range_axis();
        objects.sort_by(|a, b| a.aabb().min[split_axis].total_cmp(&b.aabb().min[split_axis]));
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
                let right = branch.right.ray_hit(ray, t_bounds);
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
            BvhNode::Leaf(hittable) => hittable.ray_hit(ray, t_bounds),
            BvhNode::Empty => None,
        }
    }
}
impl From<&BvhNode> for u32 {
    fn from(value: &BvhNode) -> Self {
        match value {
            BvhNode::Empty => 0,
            BvhNode::Leaf(_) => 1,
            BvhNode::Branch(_) => 2,
        }
    }
}
