use crate::pbrt::*;

pub struct CameraTransform {
    renderFromCamera: Transform,
    worldFromRender: Transform,
}

impl CameraTransform {
    pub fn new(_world_from_camera: Transform, rendering_space: RenderingCoordinateSystem) -> Self {
        let _worldFromRender = match rendering_space {
            RenderingCoordinateSystem::Camera => _world_from_camera,

            RenderingCoordinateSystem::CameraWorld => {
                let p_camera = _world_from_camera.on_point(Point3f::new(0.0, 0.0, 0.0));

                Transform::translate(Vector3f::from(p_camera))
            }

            RenderingCoordinateSystem::World => Transform::identity(),

            _ => {
                panic!("unknown rendering_space");
            }
        };

        return CameraTransform {
            worldFromRender: _worldFromRender,
            renderFromCamera: _worldFromRender.inverse(),
        };
    }
}
