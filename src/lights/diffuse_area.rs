use crate::pbrt::*;
use crate::FilterFunction::Point;

pub struct DiffuseAreaLight {
    base: LightBase,
    shape: Arc<dyn Shape>,
    area: f64,
    two_sided: bool,
    lemit: DenselySampledSpectrum,
    scale: f64,
}

impl Light for DiffuseAreaLight {
    fn light_type(&self) -> LightType {
        return self.base.light_type;
    }

    fn le(&self, ray: &Ray, lambda: &SampledWavelengths) -> SampledSpectrum {
        //TODO: progress 2023/12/20 implement DiffuseAreaLight
        panic!("DiffuseAreaLight::le() not implemented");
    }

    fn l(
        &self,
        p: Point3f,
        n: Normal3f,
        uv: Point2f,
        w: Vector3f,
        lambda: &SampledWavelengths,
    ) -> SampledSpectrum {
        // Check for zero emitted radiance from point on area light
        if !self.two_sided && n.dot(w) < 0.0 {
            return SampledSpectrum::same_value(0.0);
        }

        return self.scale * self.lemit.sample(lambda);
    }

    fn sample_li(
        &self,
        ctx: &LightSampleContext,
        u: Point2f,
        lambda: &SampledWavelengths,
        allow_incomplete_pdf: bool,
    ) -> Option<LightLiSample> {
        // Sample point on shape for _DiffuseAreaLight_
        let shape_ctx = ShapeSampleContext {
            pi: ctx.pi,
            n: ctx.n,
            ns: ctx.ns,
        };

        let ss = match self.shape.sample_with_context(&shape_ctx, u) {
            None => {
                return None;
            }
            Some(_ss) => _ss,
        };

        if ss.pdf == 0.0
            || (Point3f::from(ss.interaction.pi) - Point3f::from(ctx.pi)).length_squared() == 0.0
        {
            return None;
        }

        // Return _LightLiSample_ for sampled point on shape
        let wi = (Point3f::from(ss.interaction.pi) - Point3f::from(ctx.pi)).normalize();

        let Le = self.l(
            ss.interaction.pi.into(),
            ss.interaction.n,
            ss.interaction.uv,
            -wi,
            lambda,
        );

        if !Le.is_positive() {
            return None;
        }

        return Some(LightLiSample {
            l: Le,
            wi,
            pdf: ss.pdf,
            p_light: ss.interaction,
        });
    }
}

impl DiffuseAreaLight {
    pub fn new(
        render_from_light: Transform,
        parameters: &ParameterDict,
        shape: Arc<dyn Shape>,
    ) -> Self {
        let rgb_l = parameters.get_rgb("L", None);
        let spectrum_l = RGBIlluminantSpectrum::new(rgb_l);
        let mut scale = parameters.get_one_float("scale", Some(1.0));
        let two_sided = parameters.get_one_bool("twosided", Some(false));

        if parameters.has_string("filename") {
            panic!("this part was not implemented");
        }

        scale /= spectrum_l.to_photometric();

        let phi_v = parameters.get_one_float("power", Some(-1.0));
        if phi_v > 0.0 {
            panic!("this part was not implemented");
        }

        let area = shape.area();
        return Self {
            base: LightBase {
                light_type: LightType::DeltaDirection,
                render_from_light,
            },
            shape,
            area,
            two_sided,
            lemit: DenselySampledSpectrum::from_spectrum(&spectrum_l),
            scale,
        };
    }
}
