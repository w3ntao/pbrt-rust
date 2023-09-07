use crate::pbrt::*;

pub struct AmbientOcclusion {
    aggregate: Arc<dyn Primitive>,
}

impl AmbientOcclusion {
    pub fn new(aggregate: Arc<dyn Primitive>) -> Self {
        return AmbientOcclusion { aggregate };
    }
}

impl Integrator for AmbientOcclusion {
    fn Li(&self, ray: &dyn Ray, sampler: &mut dyn Sampler) -> RGBColor {
        // TODO: this is incomplete, consider BSDF only for now

        let si = match self.aggregate.intersect(ray, Float::INFINITY) {
            None => {
                return RGBColor::black();
            }
            Some(_si) => _si,
        };

        let isect = si.interaction;

        let n = isect.n.face_forward(-ray.get_d());
        let u = sampler.get_2d();

        let local_wi = sample_cosine_hemisphere(u);
        let pdf = cosine_hemisphere_pdf(local_wi.z.abs());

        if pdf == 0.0 {
            return RGBColor::black();
        }

        let frame = Frame::from_z(Vector3f::from(n));
        let wi = frame.from_local(local_wi);

        // Divide by pi so that fully visible is one.
        let differential_ray = isect.spawn_ray(wi);
        if !self.fast_intersect(&differential_ray, Float::INFINITY) {
            let grey = n.dot(wi) / (PI * pdf);

            return RGBColor::new(grey, grey, grey);
        }

        return RGBColor::black();
    }

    fn fast_intersect(&self, ray: &dyn Ray, t_max: Float) -> bool {
        return self.aggregate.fast_intersect(ray, t_max);
    }
}
