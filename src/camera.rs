use nalgebra as na;

pub struct Camera {
    pub target: na::Point3<f32>,
    distance: f32,
    projection: na::Orthographic3<f32>,
}

impl Camera {
    pub fn new(_initial_tilt: f32, initial_distance: f32) -> Camera {
        Camera {
            target: na::Point3::origin(),
            distance: initial_distance,
            projection: na::geometry::Orthographic3::new(-45.0, 45.0, -35.0, 35.0, -1.0, 1.0),
        }
    }

    pub fn get_view_matrix(&self) -> na::Matrix4<f32> {
        (na::Translation3::<f32>::from(self.target.coords)
            * na::Translation3::<f32>::from(na::Vector3::z() * self.distance))
        .inverse()
        .to_homogeneous()
    }

    pub fn get_p_matrix(&self) -> na::Matrix4<f32> {
        self.projection.into_inner()
    }
}
