import package::utils::Rgb;
//! this holds structs that are used with the cpu

struct GpuWorkState {
    start_row: u32,
    chunk_height:u32
}
const BVH_NODE_EMPTY=0u;
const BVH_NODE_LEAF=1u;
const BVH_NODE_BRANCH=2u;
struct BvhNode{
    tag: u32,
    aabb:Aabb,

    //leaf
    hittable: Hittable,

    //branch
    right_index: u32,
}
struct Camera {
    image_dims: vec2f,
    viewport_dims: vec2f,
    config: CameraConfig,
    basis_x: vec3f,
    basis_y: vec3f,
    basis_z: vec3f,
}
struct CameraConfig {
    centre: vec3f,
    look_at_centre: vec3f,
    vertical_fov: f32,
    lens_radius: f32,
}
struct RaytraceConfig {
    image_height: u32,
    image_width: u32,
    aa_samples: u32,
    max_depth: u32,
    sky_colour: Rgb,
}
const HITTABLE_SPHERE=0u;
const HITTABLE_PARALLELOGRAM=1u;
struct Hittable {
    tag: u32,
    material: Material,
    centre_or_start: vec3f,

    //sphere
    radius: f32,

    //parallelogram
    side1: vec3f,
    side2: vec3f,
}
const MATERIAL_DIFFUSE=0u;
const MATERIAL_METAL=1u;
const MATERIAL_GLASS=2u;
const MATERIAL_DIFFUSE_LIGHT=3u;
struct Material {
    tag: u32,
    colour: Rgb,
    reflect_fuzz_or_refraction_index: f32, //metal or glass
}
struct Aabb {
    min:vec3f,
    max:vec3f
}
