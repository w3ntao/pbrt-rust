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
    fn intersect(&self, r: &Ray, t_hit: &mut f32, interaction: &mut SurfaceInteraction) -> bool {
        let mut o_error = Vector3::invalid();
        let mut d_error = Vector3::invalid();

        let ray = (self.object_to_world.inverse())(*r, &mut o_error, &mut d_error);

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
        let mut t1 = ErrorFloat::without_error(-f32::INFINITY);

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

        let mut p_hit = ray(t_shape_hit.value());
        p_hit *= self.radius / (p_hit - Point::new(0.0, 0.0, 0.0)).length();
        if p_hit.x == 0.0 && p_hit.y == 0.0 {
            // TODO: what does it do (in PBRT)?
            p_hit.x = 1e-5 * self.radius;
        }

        let p_error = gamma(5) * Vector3::from(p_hit).abs();

        let mut reverse_interaction = SurfaceInteraction::default();
        reverse_interaction.p = p_hit;
        reverse_interaction.p_error = p_error;

        let normal = Normal::from(Vector3::from(p_hit));
        if normal.dot(ray.d) < 0.0 {
            reverse_interaction.entering_material = true;
            reverse_interaction.n = normal;
        } else {
            reverse_interaction.entering_material = false;
            reverse_interaction.n = -normal;
        }

        *interaction = (self.object_to_world)(reverse_interaction);
        *t_hit = t_shape_hit.value();
        return true;
    }

    fn get_bounds(&self) -> Bounds {
        return self.bounds;
    }
}
