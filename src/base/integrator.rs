use crate::pbrt::*;

pub struct IntegratorBase {
    pub camera: Arc<dyn Camera>,
    pub aggregate: Arc<dyn Primitive>,
    pub lights: Vec<Arc<dyn Light>>,
    pub infinite_lights: Vec<Arc<dyn Light>>,
}

impl IntegratorBase {
    pub fn new(
        aggregate: Arc<dyn Primitive>,
        camera: Arc<dyn Camera>,
        lights: Vec<Arc<dyn Light>>,
    ) -> Self {
        let mut infinite_lights = vec![];
        for _light in &lights {
            if _light.light_type() == LightType::Infinite {
                infinite_lights.push(_light.clone());
            }
        }

        return Self {
            camera,
            aggregate,
            lights,
            infinite_lights,
        };
    }
}

pub trait Integrator: Send + Sync {
    fn fast_intersect(&self, ray: &Ray, t_max: f64) -> bool;

    fn li(
        &self,
        ray: &DifferentialRay,
        lambda: &SampledWavelengths,
        sampler: &mut dyn Sampler,
    ) -> SampledSpectrum;
}

pub trait IntegratorDefaultInterface: Integrator {
    fn unoccluded(&self, p0: &Interaction, p1: &Interaction) -> bool {
        //TODO: this implementation is different from PBRT-v4
        return !self.fast_intersect(&p0.spawn_ray_to(p1), 0.6)
            && !self.fast_intersect(&p1.spawn_ray_to(p0), 0.6);
    }

    fn evaluate_pixel_sample(
        &self,
        p_pixel: Point2i,
        sampler: &mut dyn Sampler,
        camera: Arc<dyn Camera>,
        filter: Arc<dyn Filter>,
        film: &mut dyn Film,
    ) {
        let lu = sampler.get_1d();
        let lambda = SampledWavelengths::sample_visible(lu);

        let camera_sample = sampler.get_camera_sample(p_pixel.clone(), filter.clone());

        let camera_ray = camera.generate_camera_differential_ray(camera_sample);

        let l = camera_ray.weight * self.li(&camera_ray.ray, &lambda, sampler);

        film.add_sample(p_pixel, &l, &lambda, camera_sample.filter_weight);
    }
}

impl<T: ?Sized + Integrator> IntegratorDefaultInterface for T {}
