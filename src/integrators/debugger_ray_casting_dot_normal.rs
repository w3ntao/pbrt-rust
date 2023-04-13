use crate::core::pbrt::*;

pub struct DebuggerRayCastingDotNormal {}

impl Default for DebuggerRayCastingDotNormal {
    fn default() -> Self {
        return Self {};
    }
}

impl Integrator for DebuggerRayCastingDotNormal {
    fn get_radiance(&self, ray: Ray, scene: Arc<Scene>, _sampler: &mut dyn Sampler) -> Color {
        return match scene.intersect(&ray) {
            None => Color::black(),
            Some(si) => {
                let normal = si.n.normalize();
                let grey = normal.dot(-ray.d).max(0.0);
                Color::new(grey, grey, grey)
            }
        };
    }
}
