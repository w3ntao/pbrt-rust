use crate::pbrt::*;

pub struct AmbientOcclusion {}

impl AmbientOcclusion {
    pub fn new() -> Self {
        return AmbientOcclusion {};
    }
}

impl Integrator for AmbientOcclusion {
    fn Li(&self, ray: Ray, aggregate: Arc<BVHAggregate>, sampler: &mut dyn Sampler) -> RGBColor {
        // TODO: this is incomplete, consider BSDF only for now

        let si = match aggregate.intersect(&ray, Float::INFINITY) {
            None => {
                return RGBColor::black();
            }
            Some(_si) => _si,
        };

        let isect = si.interaction;

        let n = isect.n.face_forward(-ray.d);
        let u = sampler.get_2d();

        let local_wi = sample_cosine_hemisphere(u);
        let pdf = cosine_hemisphere_pdf(local_wi.z.abs());

        if pdf == 0.0 {
            return RGBColor::black();
        }

        let frame = Frame::from_z(Vector3f::from(n));
        let wi = frame.from_local(local_wi);

        // Divide by pi so that fully visible is one.
        let r = isect.spawn_ray(wi);
        if !aggregate.fast_intersect(&Ray::from(r), Float::INFINITY) {
            let grey = n.dot(wi) / (PI * pdf);

            return RGBColor::new(grey, grey, grey);
        }

        return RGBColor::black();
        // TODO: 2023/08/29 progress
    }

    fn fast_intersect(&self, ray: Ray, t_max: Float, aggregate: Arc<dyn Primitive>) -> bool {
        panic!("not implemented");
    }
}
