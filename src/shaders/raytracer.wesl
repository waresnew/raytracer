import package::utils::Rgb;
import package::buffers::{raytrace_config, raytrace_stats};
import package::material::dispatch::{dispatch_scatter_material, dispatch_emit_light};
import package::bvh;
import package::structs::RaytraceStats;
import package::utils::F32_INFINITY;
fn ray_cast(orig_ray:Ray, stats_buffer_index:u32)->Rgb {
    var ray=orig_ray;
    var depth=0u;
    var ray_cnt=0u;
    var ans=Rgb(0.0,0.0,0.0);
    while (depth<raytrace_config.max_depth) { //good thing the cpu impl was tail recursive
        ray_cnt+=1;
        // min=0.001 to avoid shadow acne (eg. a reflected ray may start 0.00001 inside an object bc float imprecision - want to ignore those)
        let hit_result=bvh::ray_hit(ray,0.001, F32_INFINITY);
        if !hit_result.valid {
            ans= ray.attenuation*raytrace_config.sky_colour;
            break;
        }
        let scatter_result=dispatch_scatter_material(hit_result.material, hit_result);
        if scatter_result.valid {
            ray=scatter_result;
            depth+=1;
        } else {
            ans= ray.attenuation * dispatch_emit_light(hit_result.material);
            break;
        }
    }
    raytrace_stats[stats_buffer_index].total_rays+=ray_cnt; // don't need atomic bc each sample is sequential per pixel
    return ans;
}
fn ray_at(ray:Ray, t:f32)->vec3f {
    return ray.point+t*ray.dir;
}
struct Ray {
    valid:bool,

    //valid
    point:vec3f,
    dir:vec3f,
    attenuation:Rgb
}

fn invalid_ray() ->Ray {
    return Ray(false,vec3f(0.0),vec3f(0.0),Rgb(0.0));
}
