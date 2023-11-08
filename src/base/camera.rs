use crate::pbrt::*;

pub enum RenderingCoordinateSystem {
    Camera,
    CameraWorld,
    World,
}

#[derive(Copy, Clone)]
pub struct CameraTransform {
    pub render_from_camera: Transform,
    pub world_from_render: Transform,
}

impl CameraTransform {
    pub fn nan() -> Self {
        return Self {
            render_from_camera: Transform::nan(),
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
        };
    }

    pub fn render_from_world(&self) -> Transform {
        return self.world_from_render.inverse();
    }

    pub fn camera_from_world(&self) -> Transform {
        return (self.world_from_render * self.render_from_camera).inverse();
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
    pub ray: SimpleRay,
    pub weight: SampledSpectrum,
}

pub trait Camera: Send + Sync {
    fn generate_camera_ray(&self, sample: CameraSample) -> CameraRay;
}
