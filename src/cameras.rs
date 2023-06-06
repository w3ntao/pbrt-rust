use crate::pbrt::*;

pub struct CameraTransform {
    pub renderFromCamera: Transform,
    pub worldFromRender: Transform,
}

impl CameraTransform {
    pub fn new(_world_from_camera: Transform, rendering_space: RenderingCoordinateSystem) -> Self {
        let _worldFromRender = match rendering_space {
            RenderingCoordinateSystem::Camera => _world_from_camera,

            RenderingCoordinateSystem::CameraWorld => {
                // the default option
                let p_camera = _world_from_camera.on_point(Point3f::new(0.0, 0.0, 0.0));

                Transform::translate(Vector3f::from(p_camera))
            }

            RenderingCoordinateSystem::World => Transform::identity(),

            _ => {
                panic!("unknown rendering_space");
            }
        };

        let renderFromWorld = _worldFromRender.inverse();
        let _renderFromCamera = renderFromWorld * _world_from_camera;

        return CameraTransform {
            worldFromRender: _worldFromRender,
            renderFromCamera: _renderFromCamera,
        };
    }
}

pub struct PerspectiveCamera {
    pub camera_transform: CameraTransform,
    pub fov: Float,
}

impl PerspectiveCamera {
    pub fn new(_camera_transform: CameraTransform, parameters: ParameterDict) -> Self {
        let _fov = parameters.get_one_float("fov", 90.0);
        return PerspectiveCamera {
            camera_transform: _camera_transform,
            fov: _fov,
        };
    }
}
