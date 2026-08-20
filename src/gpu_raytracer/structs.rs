//! this module holds gpu-compatible structs. manual tagged unions are used to replace enums

use encase::ShaderType;
use glam::{Vec2, Vec3};
use image::RgbImage;

use crate::{
    aabb::Aabb,
    bvh::BvhNode,
    camera::{Camera, CameraConfig},
    cpu_raytracer::RaytraceConfig,
    hittable::{Hit, Hittable},
    material::Material,
    rgb::Rgb,
};

#[derive(ShaderType, Default)]
pub struct BvhNodeGpu {
    tag: u32,
    aabb: AabbGpu,

    //leaf
    hittable: HittableGpu,

    //branch
    /// left_index is always i+1 where i is index of cur
    right_index: u32,
}
impl From<&BvhNode> for Vec<BvhNodeGpu> {
    fn from(value: &BvhNode) -> Self {
        /// returns the index of cur in ans
        fn dfs_bvh(cur: &BvhNode, ans: &mut Vec<BvhNodeGpu>) -> u32 {
            let index = ans.len();
            match cur {
                BvhNode::Empty => {
                    ans.push(BvhNodeGpu {
                        tag: cur.into(),
                        ..Default::default()
                    });
                }
                BvhNode::Leaf(hittable) => {
                    ans.push(BvhNodeGpu {
                        tag: cur.into(),
                        aabb: hittable.aabb().into(),
                        hittable: (*hittable).into(),
                        ..Default::default()
                    });
                }
                BvhNode::Branch(bvh_branch) => {
                    ans.push(BvhNodeGpu {
                        tag: cur.into(),
                        right_index: 0,
                        aabb: bvh_branch.aabb().into(),
                        ..Default::default()
                    });
                    dfs_bvh(&bvh_branch.left, ans);
                    let right_index = dfs_bvh(&bvh_branch.right, ans);
                    ans[index].right_index = right_index;
                }
            }
            index as u32
        }
        let mut ans = Vec::new();
        dfs_bvh(value, &mut ans);
        ans
    }
}

#[derive(ShaderType, Default)]
pub struct HittableGpu {
    tag: u32,
    material: MaterialGpu,
    centre_or_start: Vec3, //sphere or parallelogram

    //sphere
    radius: f32,

    //parallelogram
    side1: Vec3,
    side2: Vec3,
}
#[derive(ShaderType, Default)]
pub struct MaterialGpu {
    tag: u32,
    colour: Vec3,
    reflect_fuzz_or_refraction_index: f32, //metal or glass
}

impl From<Material> for MaterialGpu {
    fn from(value: Material) -> Self {
        match value {
            Material::Diffuse(diffuse) => Self {
                tag: value.into(),
                colour: Vec3::new(diffuse.colour.r, diffuse.colour.g, diffuse.colour.b),
                ..Default::default()
            },
            Material::Metal(metal) => Self {
                tag: value.into(),
                colour: Vec3::new(metal.colour.r, metal.colour.g, metal.colour.b),
                reflect_fuzz_or_refraction_index: metal.reflect_fuzz,
            },
            Material::Glass(glass) => Self {
                tag: value.into(),
                reflect_fuzz_or_refraction_index: glass.refraction_index,
                ..Default::default()
            },
            Material::DiffuseLight(diffuse_light) => Self {
                tag: value.into(),
                colour: Vec3::new(
                    diffuse_light.colour.r,
                    diffuse_light.colour.g,
                    diffuse_light.colour.b,
                ),
                ..Default::default()
            },
        }
    }
}
impl From<Hittable> for HittableGpu {
    fn from(value: Hittable) -> Self {
        match value {
            Hittable::Sphere(sphere) => Self {
                tag: value.into(),
                centre_or_start: sphere.centre,
                radius: sphere.radius,
                material: sphere.material.into(),
                ..Default::default()
            },
            Hittable::Parallelogram(parallelogram) => Self {
                tag: value.into(),
                centre_or_start: parallelogram.start,
                side1: parallelogram.side1,
                side2: parallelogram.side2,
                material: parallelogram.material.into(),
                ..Default::default()
            },
        }
    }
}
#[derive(ShaderType)]
pub struct CameraGpu {
    image_dims: Vec2,
    viewport_dims: Vec2,
    config: CameraConfigGpu,
    basis_x: Vec3,
    basis_y: Vec3,
    basis_z: Vec3,
}
#[derive(ShaderType)]
pub struct CameraConfigGpu {
    pub centre: Vec3,
    /// this determines focus distance for now
    pub look_at_centre: Vec3,
    pub vertical_fov: f32,
    pub lens_radius: f32,
}
#[derive(ShaderType)]
pub struct RaytraceConfigGpu {
    pub image_height: u32,
    pub image_width: u32,
    pub aa_samples: u32,
    pub max_depth: u32,
    pub sky_colour: Vec3,
}
impl From<CameraConfig> for CameraConfigGpu {
    fn from(
        CameraConfig {
            centre,
            look_at_centre,
            vertical_fov,
            lens_radius,
        }: CameraConfig,
    ) -> Self {
        Self {
            centre,
            look_at_centre,
            vertical_fov,
            lens_radius,
        }
    }
}
impl From<RaytraceConfig> for RaytraceConfigGpu {
    fn from(
        RaytraceConfig {
            image_height,
            image_width,
            aa_samples,
            max_depth,
            sky_colour,
        }: RaytraceConfig,
    ) -> Self {
        Self {
            image_height,
            image_width,
            aa_samples,
            max_depth,
            sky_colour: Vec3::new(sky_colour.r, sky_colour.g, sky_colour.b),
        }
    }
}
impl From<Camera> for CameraGpu {
    fn from(
        Camera {
            image_dims,
            viewport_dims,
            config,
            basis,
        }: Camera,
    ) -> Self {
        Self {
            image_dims,
            viewport_dims,
            config: config.into(),
            basis_x: basis.0,
            basis_y: basis.1,
            basis_z: basis.2,
        }
    }
}
pub struct RgbImageGpu {
    pub height: u32,
    pub width: u32,
    pub buffer: Vec<Vec3>,
}
impl From<RgbImageGpu> for RgbImage {
    fn from(value: RgbImageGpu) -> Self {
        RgbImage::from_raw(
            value.width,
            value.height,
            value
                .buffer
                .into_iter()
                .map(|x| Rgb::from(x).into_raw())
                .flat_map(|rgb| rgb.0)
                .collect(),
        )
        .unwrap()
    }
}

#[derive(ShaderType)]
pub struct GpuWorkState {
    pub start_row: u32,
    pub chunk_height: u32,
}
#[derive(ShaderType, Default)]
pub struct AabbGpu {
    pub min: Vec3,
    pub max: Vec3,
}
impl From<Aabb> for AabbGpu {
    fn from(value: Aabb) -> Self {
        Self {
            min: value.min,
            max: value.max,
        }
    }
}
