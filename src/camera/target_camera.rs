// use super::WasdMovement;
use nalgebra as na;

pub struct TargetCamera {
    pub target: na::Point3<f32>,
    distance: f32,
    pub rotation: na::UnitQuaternion<f32>,
    projection: na::Orthographic3<f32>,
}

impl TargetCamera {
    pub fn new(_initial_tilt: f32, initial_distance: f32) -> TargetCamera {
        TargetCamera {
            target: na::Point3::origin(),
            distance: initial_distance,
            rotation: na::UnitQuaternion::from_axis_angle(&na::Vector3::x_axis(), 0.0),
            projection: na::geometry::Orthographic3::new(-45.0, 45.0, -35.0, 35.0, 0.0, 10.0),
        }
    }

    pub fn get_view_matrix(&self) -> na::Matrix4<f32> {
        println!("{}", self.rotation);

        (na::Translation3::<f32>::from(self.target.coords)
            * self.rotation
            * na::Translation3::<f32>::from(na::Vector3::z() * self.distance))
        .inverse()
        .to_homogeneous()
    }

    pub fn get_p_matrix(&self) -> na::Matrix4<f32> {
        self.projection.into_inner()
    }
}
