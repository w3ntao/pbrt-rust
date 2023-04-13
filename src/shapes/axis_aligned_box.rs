use crate::core::pbrt::*;

pub struct AxisAlignedBox {
    bounds: Bounds,
    triangles: Vec<Arc<Triangle>>,
    // representing AxisAlignedBox with triangles
    // is not smart yet simple and easy
}

impl AxisAlignedBox {
    #[rustfmt::skip]
    pub fn new(corner_0: Point, corner_1: Point) -> Self {
        let p_min = min_of(&[corner_0, corner_1]);
        let p_max = max_of(&[corner_0, corner_1]);

        let x_span = Vector3::new(p_max.x - p_min.x, 0.0, 0.0);
        let y_span = Vector3::new(0.0, p_max.y - p_min.y, 0.0);

        let points = vec![
            p_min,
            p_min + x_span,
            p_min + x_span + y_span,
            p_min + y_span,
            p_max - x_span - y_span,
            p_max - y_span,
            p_max,
            p_max - x_span,
        ];
        let index = vec![
            0, 2, 1, 0, 2, 3,
            0, 7, 4, 0, 7, 3,
            0, 5, 4, 0, 5, 1,
            3, 6, 7, 3, 6, 2,
            1, 6, 5, 1, 6, 2,
            4, 6, 5, 4, 6, 7,
        ];

        let mesh = TriangleMesh::new(points, index);

        return Self {
            bounds: Bounds::build(&[p_min, p_max]),
            triangles: mesh.build_triangle(),
        };
    }
}

impl Shape for AxisAlignedBox {
    fn intersect(&self, ray: &Ray, t_hit: &mut f32) -> Option<SurfaceInteraction> {
        let mut intersected = false;
        let mut best_si = SurfaceInteraction::default();
        for triangle in &self.triangles {
            let mut temp_t = f32::INFINITY;

            match triangle.intersect(ray, &mut temp_t) {
                None => {
                    continue;
                }
                Some(si) => {
                    if temp_t >= *t_hit {
                        continue;
                    }

                    intersected = true;
                    *t_hit = temp_t;
                    best_si = si;
                }
            }
        }

        if !intersected {
            return None;
        }

        return Some(best_si);
    }

    fn get_bounds(&self) -> Bounds {
        return self.bounds;
    }
}
