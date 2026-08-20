import package::utils::Rgb;
import package::buffers::raytrace_config;
import package::material::dispatch::{dispatch_scatter_material, dispatch_emit_light};
import package::bvh;
import package::utils::F32_INFINITY;
fn ray_cast(orig_ray:Ray)->Rgb {
    var ray=orig_ray;
    var depth=0u;
    while (depth<raytrace_config.max_depth) {
        // min=0.001 to avoid shadow acne (eg. a reflected ray may start 0.00001 inside an object bc float imprecision - want to ignore those)
        let hit_result=bvh::ray_hit(ray,0.001, F32_INFINITY);
        if !hit_result.valid {
            return ray.attenuation*raytrace_config.sky_colour;
        }
        let scatter_result=dispatch_scatter_material(hit_result.material, hit_result);
        if scatter_result.valid {
            ray=scatter_result;
            depth+=1;
        } else {
            return ray.attenuation * dispatch_emit_light(hit_result.material);
        }
    }
    if depth==raytrace_config.max_depth {
        return Rgb(0.0,0.0,0.0);
    }
    //wgsl can't infer that this is unreachable
    return Rgb(0.0,0.0,0.0);
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
