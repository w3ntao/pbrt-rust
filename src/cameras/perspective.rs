use crate::pbrt::*;

pub struct PerspectiveCamera {
    pub camera_transform: CameraTransform,

    pub screen_from_camera: Transform,
    pub camera_from_raster: Transform,
    pub raster_from_screen: Transform,
    pub screen_from_raster: Transform,

    pub dx_camera: Vector3f,
    pub dy_camera: Vector3f,

    pub lens_radius: Float,
}

impl PerspectiveCamera {
    pub fn new(
        camera_transform: CameraTransform,
        parameters: ParameterDict,
        resolution: Point2i,
    ) -> Self {
        let _fov = parameters.get_one_float("fov", Some(90.0));

        let frame_aspect_ratio = (resolution.x as Float) / (resolution.y as Float);

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

        let ndc_from_screen =
            Transform::scale(
                1.0 / (screen_window.p_max.x - screen_window.p_min.x),
                1.0 / (screen_window.p_max.y - screen_window.p_min.y),
                1.0,
            ) * Transform::translate(-screen_window.p_min.x, -screen_window.p_max.y, 0.0);

        let raster_from_ndc = Transform::scale(resolution.x as Float, -resolution.y as Float, 1.0);

        let raster_from_screen = raster_from_ndc * ndc_from_screen;

        let screen_from_raster = raster_from_screen.inverse();

        let screen_from_camera = Transform::perspective(_fov, 1e-2, 1000.0);

        let camera_from_raster = screen_from_camera.inverse() * screen_from_raster;

        let dx_camera = camera_from_raster.on_point3f(Point3f::new(1.0, 0.0, 0.0))
            - camera_from_raster.on_point3f(Point3f::new(0.0, 0.0, 0.0));

        let dy_camera = camera_from_raster.on_point3f(Point3f::new(0.0, 1.0, 0.0))
            - camera_from_raster.on_point3f(Point3f::new(0.0, 0.0, 0.0));

        return PerspectiveCamera {
            camera_transform,
            raster_from_screen: raster_from_screen,
            screen_from_raster: screen_from_raster,
            screen_from_camera: screen_from_camera,
            camera_from_raster: camera_from_raster,
            dx_camera: dx_camera,
            dy_camera: dy_camera,
            lens_radius: 0.0,
        };
    }
}

impl Camera for PerspectiveCamera {
    fn generate_camera_ray(&self, sample: CameraSample) -> SimpleRay {
        let p_film = Point3f::new(sample.p_film.x, sample.p_film.y, 0.0);
        let p_camera = self.camera_from_raster.on_point3f(p_film);

        let ray = SimpleRay::new(
            Point3f::new(0.0, 0.0, 0.0),
            Vector3f::from(p_camera).normalize(),
        );

        if self.lens_radius > 0.0 {
            panic!("not yet implemented");
        }

        // TODO: CameraRay not implemented

        let (camera_ray, _) = self.camera_transform.render_from_camera.on_ray(ray);
        return camera_ray;
    }
}
