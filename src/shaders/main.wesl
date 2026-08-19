import package::{rng, camera};
import package::raytracer::Ray;
import package::raytracer::ray_cast;
import package::utils::Rgb;

@compute
@workgroup_size(16,16)
fn main(@builtin(global_invocation_id) global_invocation_id: vec3u) {
    let pixel_x=global_invocation_id.x;
    var pixel_y=global_invocation_id.y+gpu_work_state.start_row;
    if pixel_x>=raytrace_config.image_width || pixel_y>=raytrace_config.image_height {
        return;
    }

    var sum_colour=Rgb(0.0,0.0,0.0);
    for (var i=0;i<raytrace_config.aa_samples;i++) {
        let sample_point=vec2f(rng::rand_range(pixel_x-0.5, pixel_x+0.5), rng::rand_range(pixel_y-0.5, pixel_y+0.5));
        let world_point=camera::screen_to_viewport(sample_point);
        let lens_point=camera::rand_lens_point();
        let ray_dir = world_point - lens_point;
        let ray = Ray(true,lens_point, normalize(ray_dir), Rgb(1.0,1.0,1.0));
        let result_colour=ray_cast(ray);
        sum_colour+=result_colour;
    }
    return sum_colour/raytrace_config.aa_samples;
}

