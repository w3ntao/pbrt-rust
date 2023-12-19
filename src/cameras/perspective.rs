use crate::pbrt::*;

pub struct PerspectiveCamera {
    pub camera_base: CameraBase,

    pub screen_from_camera: Transform,
    pub camera_from_raster: Transform,
    pub raster_from_screen: Transform,
    pub screen_from_raster: Transform,

    pub dx_camera: Vector3f,
    pub dy_camera: Vector3f,

    pub lens_radius: f64,
}

impl PerspectiveCamera {
    fn build_camera_without_differential(
        camera_transform: CameraTransform,
        parameters: ParameterDict,
        resolution: Point2i,
    ) -> Self {
        let _fov = parameters.get_one_float("fov", Some(90.0));

        let frame_aspect_ratio = (resolution.x as f64) / (resolution.y as f64);

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

        let raster_from_ndc = Transform::scale(resolution.x as f64, -resolution.y as f64, 1.0);

        let raster_from_screen = raster_from_ndc * ndc_from_screen;

        let screen_from_raster = raster_from_screen.inverse();

        let screen_from_camera = Transform::perspective(_fov, 1e-2, 1000.0);

        let camera_from_raster = screen_from_camera.inverse() * screen_from_raster;

        let dx_camera = camera_from_raster.on_point3f(Point3f::new(1.0, 0.0, 0.0))
            - camera_from_raster.on_point3f(Point3f::new(0.0, 0.0, 0.0));

        let dy_camera = camera_from_raster.on_point3f(Point3f::new(0.0, 1.0, 0.0))
            - camera_from_raster.on_point3f(Point3f::new(0.0, 0.0, 0.0));

        let camera_base = CameraBase {
            camera_transform,
            min_pos_differential_x: Vector3f::nan(),
            min_pos_differential_y: Vector3f::nan(),
            min_dir_differential_x: Vector3f::nan(),
            min_dir_differential_y: Vector3f::nan(),
        };

        return Self {
            camera_base,
            raster_from_screen,
            screen_from_raster,
            screen_from_camera,
            camera_from_raster,
            dx_camera,
            dy_camera,
            lens_radius: 0.0,
        };
    }

    pub fn new(
        camera_transform: CameraTransform,
        parameters: ParameterDict,
        resolution: Point2i,
    ) -> Self {
        let mut perspective_camera = PerspectiveCamera::build_camera_without_differential(
            camera_transform,
            parameters,
            resolution,
        );

        let mut camera_base = perspective_camera.camera_base.clone();
        camera_base.find_minimum_differentials(&perspective_camera, resolution);
        perspective_camera.camera_base = camera_base;

        return perspective_camera;
    }
}

impl Camera for PerspectiveCamera {
    fn generate_camera_ray(&self, sample: CameraSample) -> CameraRay {
        // Compute raster and camera sample positions
        let p_film = Point3f::new(sample.p_film.x, sample.p_film.y, 0.0);
        let p_camera = self.camera_from_raster.on_point3f(p_film);

        let ray = Ray::new(
            Point3f::new(0.0, 0.0, 0.0),
            Vector3f::from(p_camera).normalize(),
        );

        if self.lens_radius == 0.0 {
            let (transformed_ray, _) = self
                .camera_base
                .camera_transform
                .render_from_camera
                .on_ray(&ray);

            return CameraRay {
                ray: transformed_ray,
                weight: SampledSpectrum::new([1.0; NUM_SPECTRUM_SAMPLES]),
            };
        }

        //self.lens_radius > 0.0
        panic!("not implemented");
    }

    fn generate_camera_differential_ray(&self, sample: CameraSample) -> CameraDifferentialRay {
        // Compute raster and camera sample positions
        let p_film = Point3f::new(sample.p_film.x, sample.p_film.y, 0.0);
        let p_camera = self.camera_from_raster.on_point3f(p_film);

        let ray = Ray::new(
            Point3f::new(0.0, 0.0, 0.0),
            Vector3f::from(p_camera).normalize(),
        );

        if self.lens_radius == 0.0 {
            let rx_origin = ray.o;
            let ry_origin = ray.o;

            let rx_direction = (Vector3f::from(p_camera) + self.dx_camera).normalize();
            let ry_direction = (Vector3f::from(p_camera) + self.dy_camera).normalize();

            let differential_ray = DifferentialRay {
                ray,
                has_differentials: true,
                rx_origin,
                ry_origin,
                rx_direction,
                ry_direction,
            };

            let (transformed_differential_ray, _) = self
                .camera_base
                .camera_transform
                .render_from_camera
                .on_differential_ray(&differential_ray);

            return CameraDifferentialRay {
                ray: transformed_differential_ray,
                weight: SampledSpectrum::new([1.0; NUM_SPECTRUM_SAMPLES]),
            };
        }

        //self.lens_radius > 0.0
        panic!("not implemented");
    }

    fn approximate_dp_dxy(
        &self,
        p: Point3f,
        n: Normal3f,
        samples_per_pixel: usize,
    ) -> (Vector3f, Vector3f) {
        // Compute tangent plane equation for ray differential intersections
        let p_camera = self
            .camera_base
            .camera_transform
            .camera_from_render
            .on_point3f(p);
        let down_z_from_camera =
            Transform::rotate_from_to(Vector3f::from(p_camera), Vector3f::new(0.0, 0.0, 1.0));

        let p_down_z = down_z_from_camera.on_point3f(p_camera);
        let n_down_z = down_z_from_camera.on_normal3f(n);

        let d = n_down_z.z * p_down_z.z;

        // Find intersection points for approximated camera differential rays
        let x_ray = Ray::new(
            Point3f::new(0.0, 0.0, 0.0) + self.camera_base.min_pos_differential_x,
            Vector3f::new(0.0, 0.0, 1.0) + self.camera_base.min_dir_differential_x,
        );
        let tx = -(n_down_z.dot(Vector3f::from(x_ray.o)) - d) / n_down_z.dot(x_ray.d);

        let y_ray = Ray::new(
            Point3f::new(0.0, 0.0, 0.0) + self.camera_base.min_pos_differential_y,
            Vector3f::new(0.0, 0.0, 1.0) + self.camera_base.min_dir_differential_y,
        );

        let ty = -(n_down_z.dot(Vector3f::from(y_ray.o)) - d) / n_down_z.dot(y_ray.d);
        let px = x_ray.at(tx);
        let py = y_ray.at(ty);

        // Estimate $\dpdx$ and $\dpdy$ in tangent plane at intersection point

        let spp_scale = (0.125 as f64).max(1.0 / (samples_per_pixel as f64).sqrt());

        let dpdx = spp_scale
            * self
                .camera_base
                .camera_transform
                .render_from_camera
                .on_vector3f(down_z_from_camera.inverse_on_vector3f(px - p_down_z));

        let dpdy = spp_scale
            * self
                .camera_base
                .camera_transform
                .render_from_camera
                .on_vector3f(down_z_from_camera.inverse_on_vector3f(py - p_down_z));

        return (dpdx, dpdy);
    }
}
