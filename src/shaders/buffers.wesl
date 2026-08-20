import package::structs::{GpuWorkState, RaytraceConfig, Camera, BvhNode, RaytraceStats};
import package::utils::Rgb;
@group(0) @binding(0) var<uniform> gpu_work_state: GpuWorkState;
@group(0) @binding(1) var<uniform> raytrace_config: RaytraceConfig;
@group(0) @binding(2) var<uniform> camera: Camera;
@group(0) @binding(3) var<storage,read> bvh: array<BvhNode>;
@group(0) @binding(4) var<storage, read_write> output_image: array<Rgb>;
@group(0) @binding(5) var<storage, read_write> raytrace_stats: array<RaytraceStats>;
