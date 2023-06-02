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
                //let pCamera = _world_from_camera(Point3f::new(0.0, 0.0, 0.0));
                panic!("implementing");
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
