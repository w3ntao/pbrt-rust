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
    pub pFilm: Point2f,
    pub pLens: Point2f,
    pub filterWeight: Float,
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

pub trait Camera {
    fn get_film(&self) -> Arc<Mutex<SimpleRGBFilm>>;

    fn generate_camera_ray(&self, sample: CameraSample) -> Ray;
}
