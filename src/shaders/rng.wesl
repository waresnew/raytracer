import package::buffers::raytrace_config;
import package::utils::{MAX_U32,EPSILON};
struct Rng {
    state:u32
}
var<private> rng:Rng;
// xorshift algo
fn init_rng(pixel_x:u32,pixel_y:u32,sample_i:u32) {
    let seed=pixel_x+pixel_y*raytrace_config.image_width+sample_i*raytrace_config.image_width*raytrace_config.image_height;
    rng.state=seed;
}
// [0,1)
// PCG from https://www.reedbeta.com/blog/hash-functions-for-gpu-rendering/
fn rand_frac() ->f32 {
    rng.state = rng.state * 747796405u + 2891336453u;
    let word = ((rng.state >> ((rng.state >> 28u) + 4u)) ^ rng.state) * 277803737u;
    let rand_u32=(word >> 22u) ^ word;
    return f32(rand_u32)/(f32(MAX_U32)+1.0);
}


fn rand_range(min:f32,max:f32)->f32 {
    return min+rand_frac()*(max-min);
}

fn rand_unit()->vec3f {
    var vec = vec3f(0.0);
    while length(vec) < EPSILON || length(vec) > 1.0 {
        vec = vec3f(
            rand_range(-1.0,1.0),
            rand_range(-1.0,1.0),
            rand_range(-1.0,1.0),
        );
    }
    return normalize(vec);
}

