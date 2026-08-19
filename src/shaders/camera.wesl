import package::buffers::camera;
import package::rng;

fn screen_to_viewport(point:vec2f)->vec3f {
    let viewport_min = camera.config.look_at_centre - camera.viewport_dims.x / 2.0 * camera.basis_x
        + camera.viewport_dims.y / 2.0 * camera.basis_y;
    let dx = camera.viewport_dims.x / camera.image_dims.x;
    let dy = -camera.viewport_dims.y / camera.image_dims.y;
    let x_delta = dx * camera.basis_x;
    let y_delta = dy * camera.basis_y;
    return viewport_min + screen.x * x_delta + screen.y * y_delta;
}
fn rand_lens_point()->vec3f {
    let rand_unit = rng::rand_unit();
    return camera.config.lens_radius * rand_unit.y * camera.basis_x
        + camera.config.lens_radius * rand_unit.y * camera.basis_y
        + camera.config.centre;
}
