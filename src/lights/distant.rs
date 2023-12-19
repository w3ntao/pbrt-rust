use crate::pbrt::*;

pub struct DistantLight {
    light_base: LightBase,
    lemit: Arc<dyn Spectrum>,
    scale: f64,
}

impl Light for DistantLight {
    fn light_type(&self) -> LightType {
        return self.light_base.light_type;
    }
    fn le(&self, ray: &Ray, lambda: &SampledWavelengths) -> SampledSpectrum {
        //TODO: progress 2023/12/07 implementing DistantLight
        panic!("DistantLight::le() not implemented");
    }

    fn sample_li(
        &self,
        ctx: &LightSampleContext,
        u: Point2f,
        lambda: &SampledWavelengths,
        allow_incomplete_pdf: bool,
    ) -> Option<LightLiSample> {
        //TODO: progress 2023/12/07 implementing DistantLight
        panic!("DistantLight::sample_li() not implemented");
    }
}

impl DistantLight {
    pub fn new(
        render_from_light: &Transform,
        parameters: &ParameterDict,
        color_space: &RGBColorSpace,
    ) -> Self {
        let l = if parameters.has_rgb("L") {
            let rgb = parameters.get_rgb("L", None);
            RGBIlluminantSpectrum::new(rgb, color_space)
        } else {
            panic!("this part is not implemented");
        };

        let scale = {
            // Scale the light spectrum to be equivalent to 1 nit
            let _scale = parameters.get_one_float("scale", Some(1.0)) / l.to_photometric();
            // Adjust scale to meet target illuminance value
            // Like for IBLs we measure illuminance as incident on an upward-facing
            // patch.
            let e_v = parameters.get_one_float("illuminance", Some(-1.0));

            if e_v > 0.0 {
                _scale * e_v
            } else {
                _scale
            }
        };

        let from = parameters.get_one_point3("from", Some(Point3f::new(0.0, 0.0, 0.0)));
        let to = parameters.get_one_point3("to", Some(Point3f::new(0.0, 0.0, 1.0)));

        let w = (from - to).normalize();
        let (v1, v2) = w.coordinate_system();

        let t = Transform::from_array([
            [v1.x, v2.x, w.x, 0.0],
            [v1.y, v2.y, w.y, 0.0],
            [v1.z, v2.z, w.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let final_render_from_light = *render_from_light * t;

        return Self {
            lemit: Arc::new(l),
            scale,
            light_base: LightBase {
                light_type: LightType::DeltaDirection,
                render_from_light: final_render_from_light,
            },
        };
    }
}
