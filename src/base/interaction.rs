use crate::pbrt::*;

pub struct Interaction {
    pub pi: Point3fi,
    pub n: Normal3f,
    pub wo: Vector3f,
    pub uv: Point2f,
}

fn offset_ray_origin(pi: Point3fi, n: Normal3f, w: Vector3f) -> Point3f {
    // Find vector _offset_ to corner of error bounds and compute initial _po_
    let d = Vector3f::from(n).abs().dot(pi.error());
    let _offset = d * Vector3f::from(n);
    let offset = if n.dot(w) < 0.0 { -_offset } else { _offset };
    let mut po = Point3f::from(pi) + offset;

    // Round offset point _po_ away from _p_
    for i in 0..3 {
        if offset[i] > 0.0 {
            po[i] = next_float_up(po[i]);
        } else if offset[i] < 0.0 {
            po[i] = next_float_down(po[i]);
        }
    }

    return po;
}

impl Interaction {
    pub fn offset_ray_origin(&self, w: Vector3f) -> Point3f {
        return offset_ray_origin(self.pi, self.n, w);
    }

    pub fn spawn_ray(&self, d: Vector3f) -> DifferentialRay {
        return DifferentialRay::new(self.offset_ray_origin(d), d);
    }

    pub fn spawn_ray_to(&self, it: &Interaction) -> Ray {
        //TODO: medium is not implemented here
        return spawn_ray_to(self.pi, self.n, it.pi, it.n);
    }
}

pub struct SurfaceInteraction {
    pub interaction: Interaction,

    pub dpdx: Vector3f,
    pub dpdy: Vector3f,
    pub dpdu: Vector3f,
    pub dpdv: Vector3f,

    pub dndu: Normal3f,
    pub dndv: Normal3f,
    pub dudx: Float,
    pub dvdx: Float,
    pub dudy: Float,
    pub dvdy: Float,

    pub shading: Shading,

    pub material: Option<Arc<dyn Material>>,
    pub area_light: Option<Arc<dyn Light>>,
}

impl SurfaceInteraction {
    pub fn new(
        pi: Point3fi,
        uv: Point2f,
        wo: Vector3f,
        dpdu: Vector3f,
        dpdv: Vector3f,
        dndu: Normal3f,
        dndv: Normal3f,
    ) -> Self {
        return Self {
            interaction: Interaction {
                pi,
                n: Normal3f::from(dpdu.cross(dpdv).normalize()),
                uv,
                wo: wo.normalize(),
            },

            dpdx: Vector3::nan(),
            dpdy: Vector3::nan(),
            dpdu,
            dpdv,
            dndu,
            dndv,
            dudx: 0.0,
            dvdx: 0.0,
            dudy: 0.0,
            dvdy: 0.0,
            shading: Shading::nan(),
            material: None,
            area_light: None,
        };
    }

    pub fn offset_ray_origin(&self, w: Vector3f) -> Point3f {
        // TODO: SurfaceInteraction::offset_ray_origin()
        return self.interaction.offset_ray_origin(w);
    }

    pub fn spawn_ray(&self, d: Vector3f) -> DifferentialRay {
        // TODO: SurfaceInteraction::spawn_ray()

        return DifferentialRay::new(self.offset_ray_origin(d), d);
    }

    pub fn set_intersection_properties(
        &mut self,
        material: Arc<dyn Material>,
        area_light: Option<Arc<dyn Light>>,
    ) {
        self.material = Some(material);
        self.area_light = area_light;
    }

    pub fn compute_differentials(
        &mut self,
        ray: &DifferentialRay,
        camera: &dyn Camera,
        samples_per_pixel: usize,
    ) {
        let n = self.interaction.n;

        if ray.has_differentials && n.dot(ray.rx_direction) != 0.0 && n.dot(ray.ry_direction) != 0.0
        {
            // Estimate screen-space change in $\pt{}$ using ray differentials
            // Compute auxiliary intersection points with plane, _px_ and _py_
            let p = Point3f::from(self.interaction.pi);

            let d = -n.dot(Vector3f::from(p));
            let tx = (-n.dot(Vector3::from(ray.rx_origin)) - d) / n.dot(ray.rx_direction);
            let px = ray.rx_origin + tx * ray.rx_direction;

            let ty = (-n.dot(Vector3f::from(ray.ry_origin)) - d) / n.dot(ray.ry_direction);

            let py = ray.ry_origin + ty * ray.ry_direction;

            self.dpdx = px - p;
            self.dpdy = py - p;
        } else {
            // Approximate screen-space change in $\pt{}$ based on camera projection
            (self.dpdx, self.dpdy) = camera.approximate_dp_dxy(
                Point3f::from(self.interaction.pi),
                self.interaction.n,
                samples_per_pixel,
            );
        }

        // Estimate screen-space change in $(u,v)$
        // Compute $\transpose{\XFORM{A}} \XFORM{A}$ and its determinant
        let ata00 = self.dpdu.dot(self.dpdu);
        let ata01 = self.dpdu.dot(self.dpdv);
        let ata11 = self.dpdv.dot(self.dpdv);

        let inv_det = {
            let _inv_det = 1.0 / difference_of_products(ata00, ata11, ata01, ata01);
            if _inv_det.is_finite() {
                _inv_det
            } else {
                0.0
            }
        };

        // Compute $\transpose{\XFORM{A}} \VEC{b}$ for $x$ and $y$
        let atb0x = self.dpdu.dot(self.dpdx);
        let atb1x = self.dpdv.dot(self.dpdx);
        let atb0y = self.dpdu.dot(self.dpdy);
        let atb1y = self.dpdv.dot(self.dpdy);

        // Compute $u$ and $v$ derivatives with respect to $x$ and $y$
        self.dudx = difference_of_products(ata11, atb0x, ata01, atb1x) * inv_det;
        self.dvdx = difference_of_products(ata00, atb1x, ata01, atb0x) * inv_det;
        self.dudy = difference_of_products(ata11, atb0y, ata01, atb1y) * inv_det;
        self.dvdy = difference_of_products(ata00, atb1y, ata01, atb0y) * inv_det;

        // Clamp derivatives of $u$ and $v$ to reasonable values
        let local_clamp = |x: Float| {
            if x.is_finite() {
                clamp_float(x, -1e8, 1e8)
            } else {
                0.0
            }
        };

        self.dudx = local_clamp(self.dudx);
        self.dvdx = local_clamp(self.dvdx);
        self.dudy = local_clamp(self.dudy);
        self.dvdy = local_clamp(self.dvdy);
    }

    pub fn get_bsdf(
        &mut self,
        ray: &DifferentialRay,
        lambda: &SampledWavelengths,
        camera: &dyn Camera,
        sampler: &mut dyn Sampler,
    ) -> BSDF {
        // Estimate $(u,v)$ and position differentials at intersection point
        self.compute_differentials(ray, camera, sampler.samples_per_pixel());

        // Resolve _MixMaterial_ if necessary
        let material = match &self.material {
            None => {
                return BSDF {
                    bxdf: None,
                    shading_frame: Frame::nan(),
                };
            }
            Some(_material) => _material.clone(),
        };

        if material.is_mix_material() {
            panic!("this part is not implemented");
        }
        // TODO: displacement and normal_map are not implemented for get_bsdf()

        let material_eval_context = MaterialEvalContext::new(&self);

        return material.get_bsdf(&material_eval_context, lambda);
    }

    pub fn le(&self, w: Vector3f, lambda: &SampledWavelengths) -> SampledSpectrum {
        return match &self.area_light {
            None => SampledSpectrum::same_value(0.0),
            Some(are_light) => are_light.l(
                Point3f::from(self.interaction.pi),
                self.interaction.n,
                self.interaction.uv,
                w,
                lambda,
            ),
        };
    }
}
