use crate::pbrt::*;

pub struct Sphere {
    radius: f64,
    z_min: f64,
    z_max: f64,
    theta_z_min: f64,
    theta_z_max: f64,
    phi_max: f64,
    render_from_object: Transform,
    object_from_render: Transform,
    reverse_orientation: bool,
}

impl Sphere {
    pub fn new(
        render_from_object: Transform,
        object_from_render: Transform,
        reverse_orientation: bool,
        radius: f64,
        z_min: f64,
        z_max: f64,
        phi_max: f64,
    ) -> Self {
        let z_min = z_min.clamp(-radius, radius);
        let z_max = z_max.clamp(-radius, radius);

        let theta_z_min = (z_min.min(z_max) / radius).clamp(-1.0, 1.0).acos();
        let theta_z_max = (z_min.max(z_max) / radius).clamp(-1.0, 1.0).acos();

        let phi_max = degree_to_radian(phi_max.clamp(0.0, 360.0));

        return Sphere {
            render_from_object,
            object_from_render,
            reverse_orientation,
            radius,
            z_min,
            z_max,
            phi_max,
            theta_z_min,
            theta_z_max,
        };
    }

    fn basic_intersect(&self, r: &Ray, t_max: f64) -> Option<QuadricIntersection> {
        // Transform _Ray_ origin and direction to object space
        let oi = self.object_from_render.on_point3fi(Point3fi::from(r.o));
        let di = self.object_from_render.on_vector3fi(Vector3fi::from(r.d));

        // Compute sphere quadratic coefficients
        let a = di.x.sqr() + di.y.sqr() + di.z.sqr();
        let b = 2.0 * (di.x * oi.x + di.y * oi.y + di.z * oi.z);

        let c = oi.x.sqr() + oi.y.sqr() + oi.z.sqr() - Interval::from(self.radius).sqr();

        // Compute sphere quadratic discriminant _discrim_
        let v = Vector3fi::from(oi - b / (2.0 * a) * di);
        let length = v.length();
        let discrim = 4.0
            * a
            * (Interval::from(self.radius) + length)
            * (Interval::from(self.radius) - length);

        if discrim.low < 0.0 {
            return None;
        }

        // Compute quadratic $t$ values
        let root_discrim = discrim.sqrt();
        let q = if b.midpoint() < 0.0 {
            -0.5 * (b - root_discrim)
        } else {
            -0.5 * (b + root_discrim)
        };

        let t0 = q / a;
        let t1 = c / q;
        // Swap quadratic $t$ values so that _t0_ is the lesser
        let (t0, t1) = if t0.low > t1.low { (t1, t0) } else { (t0, t1) };

        // Check quadric shape _t0_ and _t1_ for nearest intersection
        if t0.high > t_max || t1.low <= 0.0 {
            return None;
        }

        let mut t_shape_hit = t0;
        if t_shape_hit.low <= 0.0 {
            t_shape_hit = t1;
            if t_shape_hit.low > t_max {
                return None;
            }
        }

        // Compute sphere hit position and $\phi$
        let mut p_hit = Point3f::from(oi) + t_shape_hit.midpoint() * Vector3f::from(di);
        // Refine sphere intersection point
        p_hit *= self.radius / (p_hit - Point3f::new(0.0, 0.0, 0.0)).length();

        if p_hit.x == 0.0 && p_hit.y == 0.0 {
            p_hit.x = 1e-5 * self.radius;
        }

        let mut phi = p_hit.y.atan2(p_hit.x);
        if phi < 0.0 {
            phi += 2.0 * PI;
        }

        // Test sphere intersection against clipping parameters
        if (self.z_min > -self.radius && p_hit.z < self.z_min)
            || (self.z_max < self.radius && p_hit.z > self.z_max)
            || phi > self.phi_max
        {
            if t_shape_hit == t1 {
                return None;
            }
            if t1.high > t_max {
                return None;
            }

            t_shape_hit = t1;
            // Compute sphere hit position and $\phi$
            p_hit = Point3f::from(oi) + t_shape_hit.midpoint() * Vector3f::from(di);
            // Refine sphere intersection point
            p_hit *= self.radius / (p_hit - Point3f::new(0.0, 0.0, 0.0)).length();

            if p_hit.x == 0.0 && p_hit.y == 0.0 {
                p_hit.x = 1e-5 * self.radius;
            }
            phi = p_hit.y.atan2(p_hit.x);
            if phi < 0.0 {
                phi += 2.0 * PI;
            }

            if (self.z_min > -self.radius && p_hit.z < self.z_min)
                || (self.z_max < self.radius && p_hit.z > self.z_max)
                || phi > self.phi_max
            {
                return None;
            }
        }

        // Return _QuadricIntersection_ for sphere intersection
        return Some(QuadricIntersection {
            t_hit: t_shape_hit.midpoint(),
            p_obj: p_hit,
            phi,
        });
    }

    fn interaction_from_intersection(
        &self,
        isect: &QuadricIntersection,
        wo: Vector3f,
    ) -> SurfaceInteraction {
        let p_hit = isect.p_obj;
        let phi = isect.phi;
        // Find parametric representation of sphere hit

        let u = phi / self.phi_max;
        let cos_theta = p_hit.z / self.radius;
        let theta = safe_acos(cos_theta);
        let v = (theta - self.theta_z_min) / (self.theta_z_max - self.theta_z_min);

        // Compute sphere $\dpdu$ and $\dpdv$
        let z_radius = (sqr(p_hit.x) + sqr(p_hit.y)).sqrt();
        let cos_phi = p_hit.x / z_radius;
        let sin_phi = p_hit.y / z_radius;

        let dpdu = Vector3f::new(-self.phi_max * p_hit.y, self.phi_max * p_hit.x, 0.0);
        let sin_theta = safe_sqrt(1.0 - sqr(cos_theta));
        let dpdv = (self.theta_z_max - self.theta_z_min)
            * Vector3f::new(
                p_hit.z * cos_phi,
                p_hit.z * sin_phi,
                -self.radius * sin_theta,
            );

        // Compute sphere $\dndu$ and $\dndv$
        let d2Pduu = -sqr(self.phi_max) * Vector3f::new(p_hit.x, p_hit.y, 0.0);
        let d2Pduv = (self.theta_z_max - self.theta_z_min)
            * p_hit.z
            * self.phi_max
            * Vector3f::new(-sin_phi, cos_phi, 0.0);
        let d2Pdvv = -sqr(self.theta_z_max - self.theta_z_min) * Vector3f::from(p_hit);
        // Compute coefficients for fundamental forms

        let E = dpdu.dot(dpdu);
        let F = dpdu.dot(dpdv);
        let G = dpdv.dot(dpdv);
        let n = dpdu.cross(dpdv).normalize();
        let e = n.dot(d2Pduu);
        let f = n.dot(d2Pduv);
        let g = n.dot(d2Pdvv);

        // Compute $\dndu$ and $\dndv$ from fundamental form coefficients
        let EGF2 = difference_of_products(E, G, F, F);
        let invEGF2 = if EGF2 == 0.0 { 0.0 } else { 1.0 / EGF2 };

        let dndu =
            Normal3f::from((f * F - e * G) * invEGF2 * dpdu + (e * F - f * E) * invEGF2 * dpdv);
        let dndv =
            Normal3f::from((g * F - f * G) * invEGF2 * dpdu + (f * F - g * E) * invEGF2 * dpdv);

        // Compute error bounds for sphere intersection
        let p_error = gamma(5) * Vector3f::from(p_hit).abs();

        let wo_object = self.object_from_render.on_vector3f(wo);
        return self
            .object_from_render
            .on_surface_interaction(SurfaceInteraction::new(
                Point3fi::from_value_and_error(p_hit, p_error),
                Point2f::new(u, v),
                wo_object,
                dpdu,
                dpdv,
                dndu,
                dndv,
            ));
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray, t_max: f64) -> Option<ShapeIntersection> {
        return match self.basic_intersect(ray, t_max) {
            None => None,
            Some(quadric_intersection) => {
                let interaction = self.interaction_from_intersection(&quadric_intersection, -ray.d);

                Some(ShapeIntersection {
                    t_hit: quadric_intersection.t_hit,
                    surface_interaction: interaction,
                })
            }
        };
    }

    fn fast_intersect(&self, ray: &Ray, t_max: f64) -> bool {
        return self.basic_intersect(ray, t_max).is_some();
    }

    fn bounds(&self) -> Bounds3f {
        let point_0 = Point3f::new(-self.radius, -self.radius, self.z_min);
        let point_1 = Point3f::new(self.radius, self.radius, self.z_max);

        let bounds = Bounds3f::from_multiple_points(&[point_0, point_1]);

        return self.render_from_object.on_bounds(bounds);
    }

    fn area(&self) -> f64 {
        let r = self.radius;
        return 4.0 * PI * r * r;
    }

    fn sample(&self, u: Point2f) -> Option<ShapeSample> {
        panic!("Sphere::sample() not implemented");
    }

    fn sample_with_context(&self, ctx: &ShapeSampleContext, u: Point2f) -> Option<ShapeSample> {
        panic!("Sphere::sample_with_context() not implemented");
    }
}
