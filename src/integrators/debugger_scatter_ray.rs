use crate::core::pbrt::*;

pub struct DebuggerScatterRay {}

impl Default for DebuggerScatterRay {
    fn default() -> Self {
        return Self {};
    }
}

impl Integrator for DebuggerScatterRay {
    fn get_radiance(&self, ray: Ray, scene: Arc<Scene>, sampler: &mut dyn Sampler) -> Color {
        return match scene.intersect(&ray) {
            None => Color::black(),
            Some(si) => match si.material.clone() {
                None => Color::black(),
                Some(material) => {
                    let mut scattered_direction = Vector3::invalid();
                    let mut attenuation = Color::black();

                    if material.scatter(
                        ray,
                        &si,
                        &mut scattered_direction,
                        &mut attenuation,
                        sampler,
                    ) {
                        scattered_direction.softmax_color()
                    } else {
                        Color::black()
                    }
                }
            },
        };
    }
}
