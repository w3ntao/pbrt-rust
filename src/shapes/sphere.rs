use crate::core::pbrt::*;

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub material: Arc<dyn Material>,
    bounds: Bounds,
}

impl Sphere {
    pub fn new(_center: Point, _radius: f32) -> Self {
        let min = _center + Vector3::new(-_radius, -_radius, -_radius);
        let max = _center + Vector3::new(_radius, _radius, _radius);
        return Self {
            center: _center,
            radius: _radius,
            bounds: Bounds::build(&[min, max]),
            material: Arc::new(NullMaterial {}),
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
    fn intersect(&self, ray: &Ray, interaction: &mut SurfaceInteraction) -> bool {
        let oc = ray.o - self.center;
        let a = ray.d.length_squared();
        let half_b = oc.dot(ray.d);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrt_d = discriminant.sqrt();

        let mut root = (-half_b - sqrt_d) / a;
        if root < ray.t_min || root > ray.t_max {
            root = (-half_b + sqrt_d) / a;
            if root < ray.t_min || root > ray.t_max {
                return false;
            }
        }
        let root = root;
        let hit_point = ray(root);
        let normal = (hit_point - self.center) / self.radius;

        *interaction =
            SurfaceInteraction::new(root, hit_point, Normal::from(normal), self.material.clone());

        if ray.d.dot(normal) > 0.0 {
            interaction.entering_material = false;
            interaction.n = -interaction.n;
        }

        let (u, v) = get_sphere_uv(Point::from((hit_point - self.center).normalize()));
        interaction.u = u;
        interaction.v = v;

        return true;
    }

    fn get_bounds(&self) -> Bounds {
        return self.bounds;
    }

    fn set_material(&mut self, material: Arc<dyn Material>) {
        self.material = material;
    }
}
