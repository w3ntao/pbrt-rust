use crate::pbrt::*;

pub struct LayeredBxDF<TypeTopBxDF: BxDF, TypeBottomBxDF: BxDF, const TWO_SIDED: bool> {
    top: Arc<TypeTopBxDF>,
    bottom: Arc<TypeBottomBxDF>,
    thickness: f64,
    albedo: SampledSpectrum,
    g: f64,
    max_depth: usize,
    n_samples: usize,
}

impl<TypeTopBxDF: BxDF, TypeBottomBxDF: BxDF, const TWO_SIDED: bool> Clone
    for LayeredBxDF<TypeTopBxDF, TypeBottomBxDF, TWO_SIDED>
{
    fn clone(&self) -> Self {
        return LayeredBxDF::<TypeTopBxDF, TypeBottomBxDF, TWO_SIDED> {
            top: self.top.clone(),
            bottom: self.bottom.clone(),
            thickness: self.thickness,
            albedo: self.albedo,
            g: self.g,
            max_depth: self.max_depth,
            n_samples: self.n_samples,
        };
    }
}

impl<TypeTopBxDF: BxDF + 'static, TypeBottomBxDF: BxDF + 'static, const TWO_SIDED: bool> BxDF
    for LayeredBxDF<TypeTopBxDF, TypeBottomBxDF, TWO_SIDED>
{
    fn fork(&self) -> Arc<dyn BxDF> {
        let forked_self = self.clone();
        return Arc::new(forked_self);
    }

    fn flags(&self) -> BxDFFlags {
        let top_flags = self.top.flags();
        let bottom_flags = self.bottom.flags();
        let mut flags = BxDFFlags::Reflection as isize;
        if top_flags.is_specular() {
            flags |= BxDFFlags::Specular as isize;
        }

        if top_flags.is_diffuse() || bottom_flags.is_diffuse() || self.albedo.is_positive() {
            flags |= BxDFFlags::Diffuse as isize;
        } else if top_flags.is_glossy() || bottom_flags.is_glossy() {
            flags |= BxDFFlags::Glossy as isize;
        }

        if top_flags.is_transmissive() && bottom_flags.is_transmissive() {
            flags |= BxDFFlags::Transmission as isize;
        }

        return BxDFFlags::from(flags);
    }

    fn f(&self, wo: Vector3f, wi: Vector3f, mode: TransportMode) -> SampledSpectrum {
        let mut f = SampledSpectrum::same_value(0.0);
        // Estimate _LayeredBxDF_ value _f_ using random sampling
        // Set _wo_ and _wi_ for layered BSDF evaluation
        let (wo, wi) = if TWO_SIDED && wo.z < 0.0 {
            (-wo, -wi)
        } else {
            (wo, wi)
        };

        // Determine entrance interface for layered BSDF
        let entered_top = TWO_SIDED || wo.z > 0.0;
        let enter_interface = if entered_top {
            TopOrBottomBxDF::<TypeTopBxDF, TypeBottomBxDF> {
                top_bxdf: Some(self.top.clone()),
                bottom_bxdf: None,
            }
        } else {
            TopOrBottomBxDF::<TypeTopBxDF, TypeBottomBxDF> {
                top_bxdf: None,
                bottom_bxdf: Some(self.bottom.clone()),
            }
        };

        // Determine exit interface and exit $z$ for layered BSDF

        let (exit_interface, non_exit_interface) = if wo.same_hemisphere(wi) ^ entered_top {
            (
                TopOrBottomBxDF::<TypeTopBxDF, TypeBottomBxDF> {
                    top_bxdf: None,
                    bottom_bxdf: Some(self.bottom.clone()),
                },
                TopOrBottomBxDF::<TypeTopBxDF, TypeBottomBxDF> {
                    top_bxdf: Some(self.top.clone()),
                    bottom_bxdf: None,
                },
            )
        } else {
            (
                TopOrBottomBxDF::<TypeTopBxDF, TypeBottomBxDF> {
                    top_bxdf: Some(self.top.clone()),
                    bottom_bxdf: None,
                },
                TopOrBottomBxDF::<TypeTopBxDF, TypeBottomBxDF> {
                    top_bxdf: None,
                    bottom_bxdf: Some(self.bottom.clone()),
                },
            )
        };

        let exit_z = if wo.same_hemisphere(wi) ^ entered_top {
            0.0
        } else {
            self.thickness
        };

        // Account for reflection at the entrance interface
        if wo.same_hemisphere(wi) {
            f = (self.n_samples as f64) * enter_interface.f(wo, wi, mode);
        }

        // TODO: build a deterministic RNG like PBRT-v4
        let mut rng = StdRng::from_entropy();
        let mut r = || -> f64 {
            return rng.gen::<f64>();
        };

        for _ in 0..self.n_samples {
            // Sample random walk through layers to estimate BSDF value
            // Sample transmission direction through entrance interface
            let wos = match enter_interface.sample_f(
                wo,
                r(),
                Point2f::new(r(), r()),
                mode,
                BxDFReflTransFlags::Transmission,
            ) {
                None => {
                    continue;
                }
                Some(bsdf_sample) => bsdf_sample,
            };

            if !wos.f.is_positive() || wos.pdf == 0.0 || wos.wi.z == 0.0 {
                continue;
            }

            // Sample BSDF for virtual light from _wi_
            let wis = match exit_interface.sample_f(
                wi,
                r(),
                Point2f::new(r(), r()),
                mode.not(),
                BxDFReflTransFlags::Transmission,
            ) {
                None => {
                    continue;
                }
                Some(bsdf_sample) => bsdf_sample,
            };
            if !wis.f.is_positive() || wis.pdf == 0.0 || wis.wi.z == 0.0 {
                continue;
            }

            // Declare state for random walk through BSDF layers
            let mut beta = wos.f * wos.wi.abs_cos_theta() / wos.pdf;
            let mut z = if entered_top { self.thickness } else { 0.0 };
            let mut w = wos.wi;
            let phase = HGPhaseFunction::new(self.g);

            for depth in 0..self.max_depth {
                // Sample next event for layered BSDF evaluation random walk
                // Possibly terminate layered BSDF random walk with Russian roulette

                if depth < 3 && beta.max_component_value() < 0.25 {
                    let q = (1.0 - beta.max_component_value()).max(0.0);
                    if r() < q {
                        break;
                    }
                    beta /= 1.0 - q;
                }

                // Account for media between layers and possibly scatter
                if !self.albedo.is_positive() {
                    // Advance to next layer boundary and update _beta_ for transmittance
                    z = if z == self.thickness {
                        0.0
                    } else {
                        self.thickness
                    };
                    beta *= self.tr(self.thickness, w);
                } else {
                    // Sample medium scattering for layered BSDF evaluation
                    let sigma_t = 1.0;
                    let dz = sample_exponential(r(), sigma_t / w.z.abs());
                    let zp = if w.z > 0.0 { z + dz } else { z - dz };
                    if z == zp {
                        continue;
                    }

                    if 0.0 < zp && zp < self.thickness {
                        // Handle scattering event in layered BSDF medium
                        // Account for scattering through _exitInterface_ using _wis_
                        let mut wt = 1.0;
                        if !exit_interface.flags().is_specular() {
                            wt = power_heuristic(1, wis.pdf, 1, phase.pdf(-w, -wis.wi));
                        }

                        f += beta
                            * self.albedo
                            * phase.p(-w, -wis.wi)
                            * wt
                            * self.tr(zp - exit_z, wis.wi)
                            * wis.f
                            / wis.pdf;

                        // Sample phase function and update layered path state
                        let u = Point2f::new(r(), r());
                        let ps = match phase.sample_p(-w, u) {
                            None => {
                                continue;
                            }
                            Some(_ps) => _ps,
                        };
                        if ps.pdf == 0.0 || ps.wi.z == 0.0 {
                            continue;
                        }
                        beta *= self.albedo * ps.p / ps.pdf;
                        w = ps.wi;
                        z = zp;

                        // Possibly account for scattering through _exitInterface_
                        if (z < exit_z && w.z > 0.0)
                            || (z > exit_z && w.z < 0.0) && !exit_interface.flags().is_specular()
                        {
                            // Account for scattering through _exitInterface_
                            let f_exit = exit_interface.f(-w, wi, mode);
                            if f_exit.is_positive() {
                                let exit_pdf = exit_interface.pdf(
                                    -w,
                                    wi,
                                    mode,
                                    BxDFReflTransFlags::Transmission,
                                );
                                let wt = power_heuristic(1, ps.pdf, 1, exit_pdf);
                                f += beta * self.tr(zp - exit_z, ps.wi) * f_exit * wt;
                            }
                        }
                        continue;
                    }
                    z = zp.clamp(0.0, self.thickness);
                }

                // Account for scattering at appropriate interface
                if z == exit_z {
                    let uc = r();
                    let bs = match exit_interface.sample_f(
                        -w,
                        uc,
                        Point2f::new(r(), r()),
                        mode,
                        BxDFReflTransFlags::Reflection,
                    ) {
                        None => {
                            break;
                        }
                        Some(_bs) => _bs,
                    };

                    if !bs.f.is_positive() || bs.pdf == 0.0 || bs.wi.z == 0.0 {
                        break;
                    }
                    beta *= bs.f * bs.wi.abs_cos_theta() / bs.pdf;
                    w = bs.wi;
                } else {
                    // Account for scattering at _nonExitInterface_
                    if !non_exit_interface.flags().is_specular() {
                        // Add NEE contribution along presampled _wis_ direction
                        let mut wt = 1.0;
                        if !exit_interface.flags().is_specular() {
                            wt = power_heuristic(
                                1,
                                wis.pdf,
                                1,
                                non_exit_interface.pdf(-w, -wis.wi, mode, BxDFReflTransFlags::All),
                            );
                        }

                        f += beta
                            * non_exit_interface.f(-w, -wis.wi, mode)
                            * wis.wi.abs_cos_theta()
                            * wt
                            * self.tr(self.thickness, wis.wi)
                            * wis.f
                            / wis.pdf;
                    }
                }
            }
        }

        return f / (self.n_samples as f64);
    }

    fn sample_f(
        &self,
        wo: Vector3f,
        uc: f64,
        u: Point2f,
        mode: TransportMode,
        sample_flags: BxDFReflTransFlags,
        // default args: BxDFReflTransFlags sampleFlags = BxDFReflTransFlags::All
    ) -> Option<BSDFSample> {
        // Set _wo_ for layered BSDF sampling
        let (wo, flip_wi) = if TWO_SIDED && wo.z < 0.0 {
            (-wo, true)
        } else {
            (wo, false)
        };
        // Sample BSDF at entrance interface to get initial direction _w_

        let entered_top = TWO_SIDED || wo.z > 0.0;
        let some_bs = if entered_top {
            self.top.sample_f(wo, uc, u, mode, BxDFReflTransFlags::All)
        } else {
            self.bottom
                .sample_f(wo, uc, u, mode, BxDFReflTransFlags::All)
        };
        let mut bs = match some_bs {
            None => {
                return None;
            }
            Some(_bs) => _bs,
        };
        if !bs.f.is_positive() || bs.pdf == 0.0 || bs.wi.z == 0.0 {
            return None;
        }

        if bs.is_reflection() {
            if flip_wi {
                bs.wi = -bs.wi;
            }
            bs.pdf_is_proportional = true;
            return Some(bs);
        }

        let mut w = bs.wi;
        let mut specular_path = bs.is_specular();

        // TODO: build a deterministic RNG like PBRT-v4
        let mut rng = StdRng::from_entropy();
        let mut r = || -> f64 {
            return rng.gen::<f64>();
        };

        // Declare common variables for layered BSDF sampling
        let mut f = bs.f * bs.wi.abs_cos_theta();
        let mut pdf = bs.pdf;
        let mut z = if entered_top { self.thickness } else { 0.0 };
        let phase = HGPhaseFunction::new(self.g);

        for depth in 0..self.max_depth {
            // Follow random walk through layers to sample layered BSDF
            // Possibly terminate layered BSDF sampling with Russian Roulette
            let rr_beta = f.max_component_value() / pdf;
            if depth > 3 && rr_beta < 0.25 {
                let q = (1.0 - rr_beta).max(0.0);
                if r() < q {
                    return None;
                }
                pdf *= 1.0 - q;
            }

            if w.z == 0.0 {
                return None;
            }

            if self.albedo.is_positive() {
                // Sample potential scattering event in layered medium
                let sigma_t = 1.0;
                let dz = sample_exponential(r(), sigma_t / w.abs_cos_theta());
                let zp = if w.z > 0.0 { z + dz } else { z - dz };
                if zp == z {
                    return None;
                }

                if 0.0 < zp && zp < self.thickness {
                    // Update path state for valid scattering event between interfaces
                    let ps = match phase.sample_p(-w, Point2f::new(r(), r())) {
                        None => {
                            return None;
                        }
                        Some(_ps) => _ps,
                    };
                    if ps.pdf == 0.0 || ps.wi.z == 0.0 {
                        return None;
                    }
                    f *= self.albedo * ps.p;
                    pdf *= ps.pdf;
                    specular_path = false;
                    w = ps.wi;
                    z = zp;
                    continue;
                }
                z = zp.clamp(0.0, self.thickness);
            } else {
                // Advance to the other layer interface
                z = if z == self.thickness {
                    0.0
                } else {
                    self.thickness
                };
                f *= self.tr(self.thickness, w);
            }

            let interface = if z == 0.0 {
                TopOrBottomBxDF {
                    top_bxdf: None,
                    bottom_bxdf: Some(self.bottom.clone()),
                }
            } else {
                TopOrBottomBxDF {
                    top_bxdf: Some(self.top.clone()),
                    bottom_bxdf: None,
                }
            };

            // Sample interface BSDF to determine new path direction
            let uc = r();
            let u = Point2f::new(r(), r());
            let bs = match interface.sample_f(-w, uc, u, mode, BxDFReflTransFlags::All) {
                None => {
                    return None;
                }
                Some(_bs) => _bs,
            };
            if !bs.f.is_positive() || bs.pdf == 0.0 || bs.wi.z == 0.0 {
                return None;
            }

            f *= bs.f;
            pdf *= bs.pdf;
            specular_path &= bs.is_specular();
            w = bs.wi;

            // Return _BSDFSample_ if path has left the layers
            if bs.is_transmission() {
                let mut flags = if wo.same_hemisphere(w) {
                    BxDFFlags::Reflection as isize
                } else {
                    BxDFFlags::Transmission as isize
                };

                flags |= if specular_path {
                    BxDFFlags::Specular as isize
                } else {
                    BxDFFlags::Glossy as isize
                };
                if flip_wi {
                    w = -w;
                }

                return Some(BSDFSample {
                    f,
                    wi: w,
                    pdf,
                    flags: BxDFFlags::from(flags),
                    eta: 1.0,
                    pdf_is_proportional: true,
                });
            }

            // Scale _f_ by cosine term after scattering at the interface
            f *= bs.wi.abs_cos_theta();
        }

        return None;
    }

    fn pdf(
        &self,
        wo: Vector3f,
        wi: Vector3f,
        mode: TransportMode,
        sample_flags: BxDFReflTransFlags,
        // default args: BxDFReflTransFlags sampleFlags = BxDFReflTransFlags::All
    ) -> f64 {
        // Set _wo_ and _wi_ for layered BSDF evaluation
        let (wo, wi) = if TWO_SIDED && wo.z < 0.0 {
            (-wo, -wi)
        } else {
            (wo, wi)
        };

        // TODO: build a deterministic RNG like PBRT-v4
        let mut rng = StdRng::from_entropy();
        let mut r = || -> f64 {
            return rng.gen::<f64>();
        };

        let entered_top = TWO_SIDED || wo.z > 0.0;
        let mut pdf_sum = 0.0;
        if wo.same_hemisphere(wi) {
            let refl_flag = BxDFReflTransFlags::Reflection;
            pdf_sum += (self.n_samples as f64)
                * if entered_top {
                    self.top.pdf(wo, wi, mode, refl_flag)
                } else {
                    self.bottom.pdf(wo, wi, mode, refl_flag)
                };
        }

        for _ in 0..self.n_samples {
            // Evaluate layered BSDF PDF sample
            if wo.same_hemisphere(wi) {
                // Evaluate TRT term for PDF estimate

                let (r_interface, t_interface) = if entered_top {
                    (
                        TopOrBottomBxDF {
                            top_bxdf: None,
                            bottom_bxdf: Some(self.bottom.clone()),
                        },
                        TopOrBottomBxDF {
                            top_bxdf: Some(self.top.clone()),
                            bottom_bxdf: None,
                        },
                    )
                } else {
                    (
                        TopOrBottomBxDF {
                            top_bxdf: Some(self.top.clone()),
                            bottom_bxdf: None,
                        },
                        TopOrBottomBxDF {
                            top_bxdf: None,
                            bottom_bxdf: Some(self.bottom.clone()),
                        },
                    )
                };

                // Sample _tInterface_ to get direction into the layers
                let trans = BxDFReflTransFlags::Transmission;
                let wos = t_interface.sample_f(wo, r(), Point2f::new(r(), r()), mode, trans);
                let wis = t_interface.sample_f(wi, r(), Point2f::new(r(), r()), mode.not(), trans);

                match (wos, wis) {
                    (Some(wos), Some(wis)) => {
                        if wos.f.is_positive()
                            && wos.pdf > 0.0
                            && wis.f.is_positive()
                            && wis.pdf > 0.0
                        {
                            if !t_interface.flags().is_non_specular() {
                                pdf_sum += r_interface.pdf(
                                    -wos.wi,
                                    -wis.wi,
                                    mode,
                                    BxDFReflTransFlags::All,
                                );
                            } else {
                                // Use multiple importance sampling to estimate PDF product
                                match r_interface.sample_f(
                                    -wos.wi,
                                    r(),
                                    Point2f::new(r(), r()),
                                    mode,
                                    BxDFReflTransFlags::All,
                                ) {
                                    None => {}
                                    Some(rs) => {
                                        if rs.f.is_positive() && rs.pdf > 0.0 {
                                            if !r_interface.flags().is_non_specular() {
                                                pdf_sum += t_interface.pdf(
                                                    -rs.wi,
                                                    wi,
                                                    mode,
                                                    BxDFReflTransFlags::All,
                                                );
                                            } else {
                                                // Compute MIS-weighted estimate of Equation
                                                // (\ref{eq:pdf-triple-canceled-one})
                                                let r_pdf = r_interface.pdf(
                                                    -wos.wi,
                                                    -wis.wi,
                                                    mode,
                                                    BxDFReflTransFlags::All,
                                                );
                                                let wt_r = power_heuristic(1, wis.pdf, 1, r_pdf);
                                                pdf_sum += wt_r * r_pdf;
                                                let t_pdf = t_interface.pdf(
                                                    -rs.wi,
                                                    wi,
                                                    mode,
                                                    BxDFReflTransFlags::All,
                                                );
                                                let wt_t = power_heuristic(1, rs.pdf, 1, t_pdf);
                                                pdf_sum += wt_t * t_pdf;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                };
            } else {
                // Evaluate TT term for PDF estimate
                let (to_interface, ti_interface) = if entered_top {
                    (
                        TopOrBottomBxDF {
                            top_bxdf: Some(self.top.clone()),
                            bottom_bxdf: None,
                        },
                        TopOrBottomBxDF {
                            top_bxdf: None,
                            bottom_bxdf: Some(self.bottom.clone()),
                        },
                    )
                } else {
                    (
                        TopOrBottomBxDF {
                            top_bxdf: None,
                            bottom_bxdf: Some(self.bottom.clone()),
                        },
                        TopOrBottomBxDF {
                            top_bxdf: Some(self.top.clone()),
                            bottom_bxdf: None,
                        },
                    )
                };
                /*
                                Float uc = r();
                Point2f u(r(), r());
                pstd::optional<BSDFSample> wos = toInterface.Sample_f(wo, uc, u, mode);
                if (!wos || !wos->f || wos->pdf == 0 || wos->wi.z == 0 ||
                    wos->IsReflection())
                    continue;
                 */

                let uc = r();
                let u = Point2f::new(r(), r());
                let wos = match to_interface.sample_f(wo, uc, u, mode, BxDFReflTransFlags::All) {
                    None => {
                        continue;
                    }
                    Some(_wos) => _wos,
                };
                if !wos.f.is_positive() || wos.pdf == 0.0 || wos.wi.z == 0.0 || wos.is_reflection()
                {
                    continue;
                }

                let uc = r();
                let u = Point2f::new(r(), r());
                let wis =
                    match ti_interface.sample_f(wi, uc, u, mode.not(), BxDFReflTransFlags::All) {
                        None => {
                            continue;
                        }
                        Some(_wis) => _wis,
                    };

                if !wis.f.is_positive() || wis.pdf == 0.0 || wis.wi.z == 0.0 || wis.is_reflection()
                {
                    continue;
                }

                if to_interface.flags().is_specular() {
                    pdf_sum += ti_interface.pdf(-wos.wi, wi, mode, BxDFReflTransFlags::All);
                } else if ti_interface.flags().is_specular() {
                    pdf_sum += to_interface.pdf(wo, -wis.wi, mode, BxDFReflTransFlags::All);
                } else {
                    pdf_sum += (to_interface.pdf(wo, -wis.wi, mode, BxDFReflTransFlags::All)
                        + ti_interface.pdf(-wos.wi, wi, mode, BxDFReflTransFlags::All))
                        / 2.0;
                }
            }
        }

        return lerp(0.9, INV_4PI, pdf_sum / (self.n_samples as f64));
    }
}

impl<TypeTopBxDF: BxDF, TypeBottomBxDF: BxDF, const TWO_SIDED: bool>
    LayeredBxDF<TypeTopBxDF, TypeBottomBxDF, TWO_SIDED>
{
    pub fn new(
        top_bxdf: Arc<TypeTopBxDF>,
        bottom_bxdf: Arc<TypeBottomBxDF>,
        thickness: f64,
        albedo: SampledSpectrum,
        g: f64,
        max_depth: usize,
        n_samples: usize,
    ) -> Self {
        return Self {
            top: top_bxdf,
            bottom: bottom_bxdf,
            thickness: thickness.max(f64::MIN_POSITIVE),
            albedo,
            g,
            max_depth,
            n_samples,
        };
    }

    pub fn tr(&self, dz: f64, w: Vector3f) -> f64 {
        if dz.abs() <= f64::MIN_POSITIVE {
            return 1.0;
        }

        return (-(dz / w.z).abs()).exp();
    }
}
