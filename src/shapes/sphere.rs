use crate::pbrt::*;

pub struct Sphere {
    radius: Float,
    z_min: Float,
    z_max: Float,
    theta_z_min: Float,
    theta_z_max: Float,
    phi_max: Float,
    render_from_object: Transform,
    object_from_render: Transform,
    reverse_orientation: bool,
}

impl Sphere {
    pub fn new(
        render_from_object: Transform,
        object_from_render: Transform,
        reverse_orientation: bool,
        radius: Float,
        z_min: Float,
        z_max: Float,
        phi_max: Float,
    ) -> Self {
        let z_min = clamp(z_min, -radius, radius);
        let z_max = clamp(z_max, -radius, radius);
        let theta_z_min = clamp(z_min.min(z_max) / radius, -1.0, 1.0).acos();
        let theta_z_max = clamp(z_min.max(z_max) / radius, -1.0, 1.0).acos();
        let phi_max = degree_to_radian(clamp(phi_max, 0.0, 360.0));

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

    fn basic_intersect(&self, r: &dyn Ray, t_max: Float) -> Option<QuadricIntersection> {
        // Transform _Ray_ origin and direction to object space
        let oi = self
            .object_from_render
            .on_point3fi(Point3fi::from(r.get_o()));
        let di = self
            .object_from_render
            .on_vector3fi(Vector3fi::from(r.get_d()));

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

    fn build_interaction(
        &self,
        quadric_intersection: &QuadricIntersection,
        wo: Vector3f,
    ) -> SurfaceInteraction {
        let p_hit = quadric_intersection.p_obj;
        let p_error = gamma(5) * Vector3f::from(p_hit).abs();

        let n = Normal3f::from(Vector3f::from(p_hit).normalize());

        let local_interaction = SurfaceInteraction {
            pi: Point3fi::from_value_and_error(p_hit, p_error),
            n,
            wo,
        };

        return self
            .render_from_object
            .on_surface_interaction(local_interaction);
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: &dyn Ray, t_max: Float) -> Option<ShapeIntersection> {
        return match self.basic_intersect(ray, t_max) {
            None => None,
            Some(quadric_intersection) => {
                let interaction = self.build_interaction(&quadric_intersection, -ray.get_d());

                Some(ShapeIntersection {
                    t_hit: quadric_intersection.t_hit,
                    interaction,
                })
            }
        };
    }

    fn fast_intersect(&self, ray: &dyn Ray, t_max: Float) -> bool {
        return self.basic_intersect(ray, t_max).is_some();
    }

    fn bounds(&self) -> Bounds3f {
        let point_0 = Point3f::new(-self.radius, -self.radius, self.z_min);
        let point_1 = Point3f::new(self.radius, self.radius, self.z_max);

        let bounds = Bounds3f::from_multiple_points(&[point_0, point_1]);

        return self.render_from_object.on_bounds(bounds);
    }
}
