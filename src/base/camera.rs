use crate::pbrt::*;

pub enum RenderingCoordinateSystem {
    Camera,
    CameraWorld,
    World,
}

#[derive(Copy, Clone)]
pub struct CameraTransform {
    pub render_from_camera: Transform,
    pub camera_from_render: Transform,
    pub world_from_render: Transform,
}

impl CameraTransform {
    pub fn nan() -> Self {
        return Self {
            render_from_camera: Transform::nan(),
            camera_from_render: Transform::nan(),
            world_from_render: Transform::nan(),
        };
    }

    pub fn new(_world_from_camera: Transform, rendering_space: RenderingCoordinateSystem) -> Self {
        let world_from_render = match rendering_space {
            RenderingCoordinateSystem::Camera => _world_from_camera,

            RenderingCoordinateSystem::CameraWorld => {
                // the default option
                let p_camera = _world_from_camera.on_point3f(Point3f::new(0.0, 0.0, 0.0));

                Transform::translate(p_camera.x, p_camera.y, p_camera.z)
            }

            RenderingCoordinateSystem::World => Transform::identity(),
        };

        let render_from_world = world_from_render.inverse();
        let render_from_camera = render_from_world * _world_from_camera;

        return CameraTransform {
            world_from_render,
            render_from_camera,
            camera_from_render: render_from_camera.inverse(),
        };
    }

    pub fn render_from_world(&self) -> Transform {
        return self.world_from_render.inverse();
    }

    pub fn camera_from_world(&self) -> Transform {
        return (self.world_from_render * self.render_from_camera).inverse();
    }
}

#[derive(Clone)]
pub struct CameraBase {
    pub camera_transform: CameraTransform,
    pub min_pos_differential_x: Vector3f,
    pub min_pos_differential_y: Vector3f,
    pub min_dir_differential_x: Vector3f,
    pub min_dir_differential_y: Vector3f,
}

impl CameraBase {
    pub fn find_minimum_differentials(&mut self, camera: &dyn Camera, resolution: Point2i) {
        self.min_pos_differential_x = Vector3f::infinity();
        self.min_pos_differential_y = Vector3f::infinity();
        self.min_dir_differential_x = Vector3f::infinity();
        self.min_dir_differential_y = Vector3f::infinity();

        let mut sample = CameraSample {
            p_film: Point2f::new(Float::NAN, Float::NAN),
            p_lens: Point2f::new(0.5, 0.5),
            filter_weight: 1.0,
        };

        let n = 512;
        for i in 0..n {
            sample.p_film = Point2f::new(
                ((i as Float) / (n - 1) as Float) * (resolution.x as Float),
                ((i as Float) / (n - 1) as Float) * (resolution.y as Float),
            );

            let crd = camera.generate_camera_differential_ray(sample);

            let ray = crd.ray;
            let dox = self
                .camera_transform
                .camera_from_render
                .on_vector3f(ray.rx_origin - ray.ray.o);
            if dox.length() < self.min_pos_differential_x.length() {
                self.min_pos_differential_x = dox;
            }

            let doy = self
                .camera_transform
                .camera_from_render
                .on_vector3f(ray.ry_origin - ray.ray.o);
            if doy.length() < self.min_pos_differential_y.length() {
                self.min_pos_differential_y = doy;
            }

            let mut ray = ray;
            ray.ray.d = ray.ray.d.normalize();
            ray.rx_direction = ray.rx_direction.normalize();
            ray.ry_direction = ray.ry_direction.normalize();

            let f = Frame::from_z(ray.ray.d);
            let df = f.to_local(ray.ray.d);
            let dxf = f.to_local(ray.rx_direction).normalize();
            let dyf = f.to_local(ray.ry_direction).normalize();

            if (dxf - df).length() < self.min_dir_differential_x.length() {
                self.min_dir_differential_x = dxf - df;
            }

            if (dyf - df).length() < self.min_dir_differential_y.length() {
                self.min_dir_differential_y = dyf - df;
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct CameraSample {
    pub p_film: Point2f,
    pub p_lens: Point2f,
    pub filter_weight: Float,
}

impl CameraSample {
    pub fn new(p_film: Point2f, p_lens: Point2f, filter_weight: Float) -> Self {
        return CameraSample {
            p_film,
            p_lens,
            filter_weight,
        };
    }
}

pub struct CameraRay {
    pub ray: Ray,
    pub weight: SampledSpectrum,
}

pub struct CameraDifferentialRay {
    pub ray: DifferentialRay,
    pub weight: SampledSpectrum,
}

pub trait Camera: Send + Sync {
    fn generate_camera_ray(&self, sample: CameraSample) -> CameraRay;

    fn generate_camera_differential_ray(&self, sample: CameraSample) -> CameraDifferentialRay;

    fn approximate_dp_dxy(
        &self,
        p: Point3f,
        n: Normal3f,
        samples_per_pixel: usize,
    ) -> (Vector3f, Vector3f);
}
