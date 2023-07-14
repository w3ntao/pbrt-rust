use crate::pbrt::*;

pub struct PerspectiveCamera {
    pub camera_transform: CameraTransform,

    pub screenFromCamera: Transform,
    pub cameraFromRaster: Transform,
    pub rasterFromScreen: Transform,
    pub screenFromRaster: Transform,

    pub dxCamera: Vector3f,
    pub dyCamera: Vector3f,

    pub lens_radius: Float,

    pub film: Arc<Mutex<SimpleRGBFilm>>,
}

impl PerspectiveCamera {
    pub fn new(
        camera_transform: CameraTransform,
        parameters: ParameterDict,
        film: Arc<Mutex<SimpleRGBFilm>>,
    ) -> Self {
        let _fov = parameters.get_one_float("fov", Some(90.0));

        let resolution = film.lock().unwrap().resolution;

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

        let NDCFromScreen =
            Transform::scale(
                1.0 / (screen_window.p_max.x - screen_window.p_min.x),
                1.0 / (screen_window.p_max.y - screen_window.p_min.y),
                1.0,
            ) * Transform::translate(-screen_window.p_min.x, -screen_window.p_max.y, 0.0);

        let rasterFromNDC = Transform::scale(resolution.x as Float, -resolution.y as Float, 1.0);

        let rasterFromScreen = rasterFromNDC * NDCFromScreen;
        // rasterFromScreen verified

        let screenFromRaster = rasterFromScreen.inverse();

        let screenFromCamera = Transform::perspective(_fov, 1e-2, 1000.0);
        // screenFromCamera verified

        let cameraFromRaster = screenFromCamera.inverse() * screenFromRaster;
        // cameraFromRaster verified

        let dxCamera = cameraFromRaster.on_point3f(Point3f::new(1.0, 0.0, 0.0))
            - cameraFromRaster.on_point3f(Point3f::new(0.0, 0.0, 0.0));

        let dyCamera = cameraFromRaster.on_point3f(Point3f::new(0.0, 1.0, 0.0))
            - cameraFromRaster.on_point3f(Point3f::new(0.0, 0.0, 0.0));

        return PerspectiveCamera {
            camera_transform,
            rasterFromScreen,
            screenFromRaster,
            screenFromCamera,
            cameraFromRaster,
            dxCamera,
            dyCamera,
            film,
            lens_radius: 0.0,
        };
    }
}

impl Camera for PerspectiveCamera {
    fn get_film(&self) -> Arc<Mutex<SimpleRGBFilm>> {
        return self.film.clone();
    }

    fn generate_camera_ray(&self, sample: CameraSample) -> Ray {
        let pFilm = Point3f::new(sample.pFilm.x, sample.pFilm.y, 0.0);
        let pCamera = self.cameraFromRaster.on_point3f(pFilm);

        let ray = Ray::new(
            Point3f::new(0.0, 0.0, 0.0),
            Vector3f::from(pCamera).normalize(),
        );

        if self.lens_radius > 0.0 {
            panic!("not yet implemented");
        }

        // TODO: CameraRay not implemented

        let (camera_ray, _) = self.camera_transform.renderFromCamera.on_ray(ray);
        return camera_ray;
    }
}
