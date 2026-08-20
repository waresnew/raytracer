import package::{rng, camera};
import package::raytracer::Ray;
import package::raytracer::ray_cast;
import package::utils::Rgb;
import package::buffers::{gpu_work_state, raytrace_config, output_image, raytrace_stats};

@compute
@workgroup_size(16,16)
fn main(@builtin(global_invocation_id) global_invocation_id: vec3u) {
    let pixel_x=global_invocation_id.x;
    let local_pixel_y=global_invocation_id.y;
    var pixel_y=local_pixel_y+gpu_work_state.start_row;
    if pixel_x>=raytrace_config.image_width || local_pixel_y>=gpu_work_state.chunk_height {
        return;
    }

    let output_buffer_index=local_pixel_y*raytrace_config.image_width+pixel_x;
    var sum_colour=Rgb(0.0,0.0,0.0);
    for (var i=0u;i<raytrace_config.aa_samples;i++) {
        rng::init_rng(pixel_x, pixel_y, i);
        let sample_point=vec2f(rng::rand_range(f32(pixel_x)-0.5, f32(pixel_x)+0.5), rng::rand_range(f32(pixel_y)-0.5, f32(pixel_y)+0.5));
        let world_point=camera::screen_to_viewport(sample_point);
        let lens_point=camera::rand_lens_point();
        let ray_dir = world_point - lens_point;
        let ray = Ray(true,lens_point, normalize(ray_dir), Rgb(1.0,1.0,1.0));
        let result_colour=ray_cast(ray, output_buffer_index);
        sum_colour+=result_colour;
    }
    let ans=sum_colour/f32(raytrace_config.aa_samples);
    output_image[output_buffer_index]=ans;
}
