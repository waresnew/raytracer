import package::structs::{BvhNode, BVH_NODE_EMPTY, BVH_NODE_LEAF, BVH_NODE_BRANCH, Material};
import package::buffers::bvh;
import package::hittable::dispatch::{dispatch_ray_hittable, HitResult, invalid_hit_result};
import package::aabb::ray_intersects_aabb;
import package::raytracer::Ray;

const MAX_STACK_SIZE=64;
fn ray_hit(ray:Ray, t_min:f32, t_max:f32)->HitResult {
    var stack:array<u32,MAX_STACK_SIZE>;   
    var sp=0u; //stack pointer points to first unused index
    stack[sp]=0;
    sp+=1;
    var ans=invalid_hit_result();
    while (sp>0) {
        let cur_i=stack[sp-1];
        let cur=bvh[cur_i];
        sp-=1;
        switch cur.tag {
            case BVH_NODE_EMPTY { }
            case BVH_NODE_LEAF {
                let hit_result=dispatch_ray_hittable(cur.hittable, ray, t_min,t_max);
                if hit_result.valid && (!ans.valid || hit_result.t<ans.t) {
                    ans=hit_result;
                }
            }
            case BVH_NODE_BRANCH {
                let left=cur_i+1;
                let right=cur.right_index;
                if ray_intersects_aabb(bvh[left].aabb, ray, t_min, t_max) {
                    stack[sp]=left;
                    sp+=1;
                }
                if ray_intersects_aabb(bvh[right].aabb, ray, t_min, t_max) {
                    stack[sp]=right;
                    sp+=1;
                }
            }
            default {
                //unreachable
            }
        }

    }
    return ans;
    

}
