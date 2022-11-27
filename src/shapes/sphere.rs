use crate::core::pbrt::*;

pub struct Sphere {
    radius: f32,
    bounds: Bounds,
    object_to_world: Transform,
}

impl Sphere {
    pub fn new(_center: Point, _radius: f32) -> Self {
        let min = _center + Vector3::new(-_radius, -_radius, -_radius);
        let max = _center + Vector3::new(_radius, _radius, _radius);

        let mut transform = Transform::identity();
        transform.translate(Vector3::from(_center));

        return Self {
            radius: _radius,
            bounds: Bounds::build(&[min, max]),
            object_to_world: transform,
        };
    }
}

fn get_sphere_uv(p: Point) -> (f32, f32) {
    // p: a given point on the sphere of radius one, centered at the origin.
    // u: returned value [0,1] of angle around the Y axis from X=-1.
    // v: returned value [0,1] of angle from Y=-1 to Y=+1.
    //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
    //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
    //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

    let theta = (-p.y).acos();
    let phi = (-p.z / p.x).atan() + PI;

    return (phi / (2.0 * PI), theta / PI);
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray, t_hit: &mut f32, interaction: &mut SurfaceInteraction) -> bool {
        let mut o_error = Vector3::invalid();
        let mut d_error = Vector3::invalid();

        let ray = (self.object_to_world.inverse())(*ray, &mut o_error, &mut d_error);

        // Compute quadratic sphere coefficients

        // Initialize _EFloat_ ray coordinate values
        let ox = ErrorFloat::with_error(ray.o.x, o_error.x);
        let oy = ErrorFloat::with_error(ray.o.y, o_error.y);
        let oz = ErrorFloat::with_error(ray.o.z, o_error.z);

        let dx = ErrorFloat::with_error(ray.d.x, d_error.x);
        let dy = ErrorFloat::with_error(ray.d.y, d_error.y);
        let dz = ErrorFloat::with_error(ray.d.z, d_error.z);

        let a = dx * dx + dy * dy + dz * dz;
        let b = 2.0 * (dx * ox + dy * oy + dz * oz);
        let c = ox * ox + oy * oy + oz * oz
            - ErrorFloat::without_error(self.radius) * ErrorFloat::without_error(self.radius);

        let mut t0 = ErrorFloat::without_error(f32::INFINITY);
        let mut t1 = ErrorFloat::without_error(f32::INFINITY);

        if !error_float_quadratic(a, b, c, &mut t0, &mut t1) {
            return false;
        }

        if t0.upper_bound() > ray.t_max || t1.lower_bound() <= 0.0 {
            return false;
        }
        let mut t_shape_hit = t0.clone();

        if t_shape_hit.lower_bound() <= 0.0 {
            t_shape_hit = t1.clone();
            if t_shape_hit.upper_bound() > ray.t_max {
                return false;
            }
        }

        *t_hit = t_shape_hit.value();

        let mut p_hit = ray(t_shape_hit.value());
        p_hit *= self.radius / (p_hit - Point::new(0.0, 0.0, 0.0)).length();

        // TODO: make everything easier by rewriting
        // TODO: Transform(SurfaceInteraction)

        p_hit = (self.object_to_world)(p_hit);

        interaction.p = p_hit;

        let center = (self.object_to_world)(Point::new(0.0, 0.0, 0.0));
        let normal = (p_hit - center) / self.radius;
        interaction.p_error = gamma(5) * Vector3::from(p_hit).abs();

        if ray.d.dot(normal) < 0.0 {
            interaction.entering_material = true;
            interaction.n = Normal::from(normal);
        } else {
            interaction.entering_material = false;
            interaction.n = -Normal::from(normal);
        }

        return true;

        let center = (self.object_to_world)(Point::new(0.0, 0.0, 0.0));

        let oc = ray.o - center;
        let a = ray.d.length_squared();
        let half_b = oc.dot(ray.d);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrt_d = discriminant.sqrt();

        let mut root = (-half_b - sqrt_d) / a;
        if root < 0.0 || root > ray.t_max {
            root = (-half_b + sqrt_d) / a;
            if root < 0.0 || root > ray.t_max {
                return false;
            }
        }

        *t_hit = root;
        let hit_point = ray(*t_hit);
        let normal = (hit_point - center) / self.radius;

        interaction.p = hit_point;
        interaction.n = Normal::from(normal);
        interaction.p_error = gamma(5) * Vector3::from(hit_point).abs();
        // taken from PBRT

        if ray.d.dot(normal) > 0.0 {
            interaction.entering_material = false;
            interaction.n = -interaction.n;
        }

        let (u, v) = get_sphere_uv(Point::from((hit_point - center).normalize()));
        interaction.u = u;
        interaction.v = v;

        return true;
    }

    fn get_bounds(&self) -> Bounds {
        return self.bounds;
    }
}
