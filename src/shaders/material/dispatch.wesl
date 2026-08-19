import package::structs::{Material, MATERIAL_DIFFUSE, MATERIAL_METAL, MATERIAL_GLASS, MATERIAL_DIFFUSE_LIGHT};
import package::raytracer::Ray;
import package::hittable::dispatch::{HitResult, invalid_hit_result};
import package::utils::Rgb;
import package::material::diffuse::scatter_diffuse;
import package::material::metal::scatter_metal;
import package::material::glass::scatter_glass;
import package::material::diffuse_light::emit_diffuse_light;

fn dispatch_scatter_material(material:Material, hit_result:HitResult)->Ray {
    switch material.tag {
        case MATERIAL_DIFFUSE {
            return scatter_diffuse(material, hit_result);
        }
        case MATERIAL_METAL {
            return scatter_metal(material, hit_result);
        }
        case MATERIAL_GLASS {
            return scatter_glass(material, hit_result);
        }
        case MATERIAL_DIFFUSE_LIGHT {
            return invalid_hit_result();
        }
    }
}
fn dispatch_emit_light(material:Material)->Rgb {
    switch material.tag {
        case MATERIAL_DIFFUSE_LIGHT {
            return emit_diffuse_light(material);
        }
        default {
            return Rgb(0.0,0.0,0.0);
        }
    }
}
