use nalgebra::{Matrix4, Point3, Vector3};

pub struct Camera {
    pub position: Vector3<f32>,
    pub yaw: f32,
    pub pitch: f32,
    pub up: Vector3<f32>, // what is up
    pub movement_speed: f32
}

impl Camera {
    pub fn new(position: Vector3<f32>, yaw: f32, pitch: f32, movement_speed: f32) -> Self {
        Self {
            position,
            yaw,
            pitch,
            up: Vector3::y(),
            movement_speed
        }
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        // Calculate the camera direction vector based on yaw and pitch
        let direction = Vector3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        );

        // Calculate the right and adjusted up vectors
        let right = self.up.cross(&direction).normalize();
        let up = direction.cross(&right).normalize();

        // Create the look_at_rh matrix
        Matrix4::look_at_rh(
            &Point3::from(self.position),
            &Point3::from(self.position + direction),
            &up,
        )
    }

    pub fn update(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.yaw += delta_yaw;
        self.pitch += delta_pitch;

        // Constrain the pitch to avoid flipping the camera
        if self.pitch > 89.0_f32 {
            self.pitch = 89.0_f32;
        } else if self.pitch < -89.0_f32 {
            self.pitch = -89.0_f32;
        }
    }

    pub fn process_keyboard(&mut self, direction: Direction, velo: f32, delta_time: f32) {
        let velocity = velo * delta_time;
        let direction_vector = Vector3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            0.0,
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        ).normalize();

        let right_vector = self.up.cross(&direction_vector).normalize();

        match direction {
            Direction::X => self.position += right_vector * velocity,
            Direction::Z => self.position -= direction_vector * velocity,
        } // we need mouse yea
    }
}

pub enum Direction {
    X,
    Z
}