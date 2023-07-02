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
                // the default option
                let p_camera = _world_from_camera.on_point3f(Point3f::new(0.0, 0.0, 0.0));

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

    pub fn RenderFromWorld(&self) -> Transform {
        return self.worldFromRender.inverse();
    }

    pub fn CameraFromWorld(&self) -> Transform {
        return (self.worldFromRender * self.renderFromCamera).inverse();
    }
}

pub struct CameraSample {
    pFilm: Point2f,
    pLens: Point2f,
    filterWeight: Float,
}

impl CameraSample {
    pub fn new(pFilm: Point2f, pLens: Point2f, filterWeight: Float) -> Self {
        return CameraSample {
            pFilm,
            pLens,
            filterWeight,
        };
    }
}

pub fn get_camera_sample(
    sampler: &mut SimpleSampler,
    pPixel: Point2i,
    filter: Arc<BoxFilter>,
) -> CameraSample {
    let fs = filter.sample(sampler.get_pixel_2d());
    return CameraSample::new(
        Point2f::from(pPixel) + fs.p + Vector2f::new(0.5, 0.5),
        sampler.get_2d(),
        1.0,
    );
}

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
        let _fov = parameters.get_one_float_with_default("fov", 90.0);

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

    pub fn generate_camera_ray(&self, sample: CameraSample) -> Ray {
        let pFilm = Point3f::new(sample.pFilm.x, sample.pFilm.y, 0.0);
        let pCamera = self.cameraFromRaster.on_point3f(pFilm);

        let ray = Ray::new(
            Point3f::new(0.0, 0.0, 0.0),
            Vector3f::from(pCamera).normalize(),
        );

        if self.lens_radius > 0.0 {
            panic!("not yet implemented");
        }

        let (camera_ray, _) = self.camera_transform.renderFromCamera.on_ray(ray);
        return camera_ray;

        // TODO: CameraRay not implemented
    }
}
