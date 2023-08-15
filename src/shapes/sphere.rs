use crate::pbrt::*;

pub struct Sphere {
    radius: Float,
    zMin: Float,
    zMax: Float,
    thetaZMin: Float,
    thetaZMax: Float,
    phiMax: Float,
    renderFromObject: Transform,
    objectFromRender: Transform,
    reverseOrientation: bool,
}

impl Sphere {
    pub fn new(
        renderFromObject: Transform,
        objectFromRender: Transform,
        reverseOrientation: bool,
        radius: Float,
        zMin: Float,
        zMax: Float,
        phiMax: Float,
    ) -> Self {
        let zMin = clamp(zMin, -radius, radius);
        let zMax = clamp(zMax, -radius, radius);
        let thetaZMin = clamp(zMin.min(zMax) / radius, -1.0, 1.0).acos();
        let thetaZMax = clamp(zMin.max(zMax) / radius, -1.0, 1.0).acos();
        let phiMax = degree_to_radian(clamp(phiMax, 0.0, 360.0));

        return Sphere {
            renderFromObject,
            objectFromRender,
            reverseOrientation,
            radius,
            zMin,
            zMax,
            phiMax,
            thetaZMin,
            thetaZMax,
        };
    }

    pub fn basic_intersect(&self, r: &Ray, t_max: Float) -> Option<QuadricIntersection> {
        // Transform _Ray_ origin and direction to object space
        let oi = self.objectFromRender.on_point3fi(Point3fi::from(r.o));
        let di = self.objectFromRender.on_vector3fi(Vector3fi::from(r.d));

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
        let rootDiscrim = discrim.sqrt();
        let q = if b.midpoint() < 0.0 {
            -0.5 * (b - rootDiscrim)
        } else {
            -0.5 * (b + rootDiscrim)
        };

        let t0 = q / a;
        let t1 = c / q;
        // Swap quadratic $t$ values so that _t0_ is the lesser
        let (t0, t1) = if t0.low > t1.low { (t1, t0) } else { (t0, t1) };

        // Check quadric shape _t0_ and _t1_ for nearest intersection
        if t0.high > t_max || t1.low <= 0.0 {
            return None;
        }

        let mut tShapeHit = t0;
        if tShapeHit.low <= 0.0 {
            tShapeHit = t1;
            if tShapeHit.low > t_max {
                return None;
            }
        }

        // Compute sphere hit position and $\phi$
        let mut pHit = Point3f::from(oi) + tShapeHit.midpoint() * Vector3f::from(di);
        // Refine sphere intersection point
        pHit *= self.radius / (pHit - Point3f::new(0.0, 0.0, 0.0)).length();

        if pHit.x == 0.0 && pHit.y == 0.0 {
            pHit.x = 1e-5 * self.radius;
        }

        let mut phi = pHit.y.atan2(pHit.x);
        if phi < 0.0 {
            phi += 2.0 * PI;
        }

        // Test sphere intersection against clipping parameters
        if (self.zMin > -self.radius && pHit.z < self.zMin)
            || (self.zMax < self.radius && pHit.z > self.zMax)
            || phi > self.phiMax
        {
            if tShapeHit == t1 {
                return None;
            }
            if t1.high > t_max {
                return None;
            }

            tShapeHit = t1;
            // Compute sphere hit position and $\phi$
            pHit = Point3f::from(oi) + tShapeHit.midpoint() * Vector3f::from(di);
            // Refine sphere intersection point
            pHit *= self.radius / (pHit - Point3f::new(0.0, 0.0, 0.0)).length();

            if pHit.x == 0.0 && pHit.y == 0.0 {
                pHit.x = 1e-5 * self.radius;
            }
            phi = pHit.y.atan2(pHit.x);
            if phi < 0.0 {
                phi += 2.0 * PI;
            }

            if (self.zMin > -self.radius && pHit.z < self.zMin)
                || (self.zMax < self.radius && pHit.z > self.zMax)
                || phi > self.phiMax
            {
                return None;
            }
        }

        // Return _QuadricIntersection_ for sphere intersection
        return Some(QuadricIntersection {
            tHit: tShapeHit.midpoint(),
            pObj: pHit,
            phi,
        });
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray, t_max: Float) -> Option<ShapeIntersection> {
        return match self.basic_intersect(ray, t_max) {
            None => None,
            Some(isect) => {
                // TODO: convert intersection to interaction
                let hit_point = self.renderFromObject.on_point3f(isect.pObj);
                let origin_world_coord = self
                    .renderFromObject
                    .on_point3f(Point3f::new(0.0, 0.0, 0.0));
                let normal = (hit_point - origin_world_coord).normalize();
                let normal = if normal.dot(ray.d) > 0.0 {
                    -normal
                } else {
                    normal
                };
                // TODO: convert intersection to interaction

                Some(ShapeIntersection {
                    normal: Normal3f::from(normal),
                    t_hit: isect.tHit,
                })
            }
        };
    }

    fn bounds(&self) -> Bounds3f {
        let point_0 = Point3f::new(-self.radius, -self.radius, self.zMin);
        let point_1 = Point3f::new(self.radius, self.radius, self.zMax);

        let bounds = Bounds3f::from_multiple_points(&[point_0, point_1]);

        return self.renderFromObject.on_bounds(bounds);
    }
}