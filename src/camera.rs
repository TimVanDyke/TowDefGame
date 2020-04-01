use nalgebra as na;

pub struct Camera {
    pub target: na::Point3<f32>,
    projection: na::Orthographic3<f32>,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            target: na::Point3::origin(),
            projection: na::geometry::Orthographic3::new(-8.0, 8.0, -4.5, 4.5, -1.0, 1.0),
        }
    }

    pub fn get_p_matrix(&self) -> na::Matrix4<f32> {
        self.projection.into_inner()
    }
}
