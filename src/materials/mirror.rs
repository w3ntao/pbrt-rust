use crate::core::pbrt::*;

pub struct Mirror {}

impl Mirror {
    pub fn new() -> Mirror {
        Mirror {}
    }
}

impl Material for Mirror {
    fn scatter(&self, incoming_ray: Ray, intersection: &Intersection) -> (bool, Ray, Color) {
        let scattered_ray = Ray::new(
            intersection.hit_point,
            incoming_ray.d.reflect(intersection.normal),
        );

        return (true, scattered_ray, Color::new(1.0, 1.0, 1.0));
        // TODO: I am implementing perfect glass for the time being
        // TODO: that reflects everything
    }

    fn is_specular(&self) -> bool {
        return true;
    }
}
