use crate::math::bounds::Bounds2f;
use crate::pbrt::*;
use std::iter::Flatten;

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

                Transform::translate(p_camera.x, p_camera.y, p_camera.z)
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

    pub screenFromCamera: Transform,
    pub cameraFromRaster: Transform,
    pub rasterFromScreen: Transform,
    pub screenFromRaster: Transform,
}

impl PerspectiveCamera {
    pub fn new(_camera_transform: CameraTransform, parameters: ParameterDict) -> Self {
        let _fov = parameters.get_one_float("fov", 90.0);

        let frame_aspect_ratio = (X_RESOLUTION as Float) / (Y_RESOLUTION as Float);

        let screen_window = if frame_aspect_ratio > 1.0 {
            Bounds2f::new(&[
                Point2f::new(-frame_aspect_ratio, -1.0),
                Point2f::new(frame_aspect_ratio, 1.0),
            ])
        } else {
            Bounds2f::new(&[
                Point2f::new(-1.0, -1.0 / frame_aspect_ratio),
                Point2f::new(1.0, 1.0 / frame_aspect_ratio),
            ])
        };

        let NDCFromScreen =
            Transform::scale(
                1.0 / (screen_window.p_max.x - screen_window.p_min.x),
                1.0 / (screen_window.p_max.y - screen_window.p_min.y),
                1.0,
            ) * Transform::translate(-screen_window.p_min.x, -screen_window.p_max.y, 0.0);

        let rasterFromNDC = Transform::scale(X_RESOLUTION as Float, -Y_RESOLUTION as Float, 1.0);

        let _rasterFromScreen = rasterFromNDC * NDCFromScreen;
        // rasterFromScreen verified

        let _screenFromRaster = _rasterFromScreen.inverse();

        let _screenFromCamera = Transform::perspective(_fov, 1e-2, 1000.0);
        // screenFromCamera verified

        let _cameraFromRaster = _screenFromCamera.inverse() * _screenFromRaster;
        // cameraFromRaster verified

        // TODO: progress: 2023/06/07

        return PerspectiveCamera {
            camera_transform: _camera_transform,
            rasterFromScreen: _rasterFromScreen,
            screenFromRaster: _screenFromRaster,
            screenFromCamera: _screenFromCamera,
            cameraFromRaster: _cameraFromRaster,
        };
    }
}
