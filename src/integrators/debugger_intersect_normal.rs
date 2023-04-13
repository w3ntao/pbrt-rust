use crate::core::pbrt::*;

pub struct DebuggerIntersectNormal {}

impl Default for DebuggerIntersectNormal {
    fn default() -> Self {
        return Self {};
    }
}

impl Integrator for DebuggerIntersectNormal {
    fn get_radiance(&self, ray: Ray, scene: Arc<Scene>, _sampler: &mut dyn Sampler) -> Color {
        return match scene.intersect(&ray) {
            None => Color::black(),
            Some(si) => Vector3::from(si.n).softmax_color(),
        };
    }
}
