use cgmath::{num_traits::clamp, Matrix4};

use crate::{
    maths::{Matrix, Vector},
    vector,
};

#[derive(Debug)]
pub struct Camera {
    aspect: f32,
    speed: f32,
    position: Vector,
    world_up: Vector,
    front: Vector,
    right: Vector,
    up: Vector,
    rotation_sensitivity: f32,
    yaw: f32,
    pitch: f32,
    near: f32,
    far: f32,
    fov: f32,
    min_fov: f32,
    max_fov: f32,
}

impl Camera {
    pub fn new(
        aspect: f32,
        speed: f32,
        position: Vector,
        world_up: Vector,
        rotation_sensitivity: f32,
        yaw: f32,
        pitch: f32,
        near: f32,
        far: f32,
        fov: f32,
        min_fov: f32,
        max_fov: f32,
    ) -> Camera {
        let mut c = Camera {
            aspect,
            speed,
            position,
            world_up,
            front: vector![0.0, 0.0, -1.0],
            right: vector![1.0, 0.0, 0.0],
            up: vector![0.0, 1.0, 0.0],
            rotation_sensitivity,
            yaw,
            pitch,
            near,
            far,
            fov,
            min_fov,
            max_fov,
        };
        c.update_vectors();
        c
    }

    pub fn pos(&self) -> Vector {
        self.position.clone()
    }

    pub fn orientation(&self) -> (f32, f32) {
        (self.yaw, self.pitch)
    }

    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed
    }

    pub fn set_yaw(&mut self, yaw: f32) {
        self.yaw = yaw;
        self.clamp_yaw();
        self.update_vectors();
    }

    pub fn set_yaw_w_offset(&mut self, yaw_offset: f32) {
        self.yaw += yaw_offset;
        self.clamp_yaw();
        self.update_vectors();
    }

    fn clamp_yaw(&mut self) {
        match self.yaw {
            y if y > 360.0 => {
                while self.yaw > 360.0 {
                    self.yaw -= 360.0
                }
            }
            y if y < 0.0 => {
                while self.yaw < 0.0 {
                    self.yaw += 360.0
                }
            }
            _ => (),
        }
    }

    pub fn set_pitch(&mut self, pitch: f32) {
        self.pitch = pitch;
        self.clamp_pitch();
        self.update_vectors();
    }

    pub fn set_pitch_w_offset(&mut self, pitch_offset: f32) {
        self.pitch += pitch_offset;
        self.clamp_pitch();
        self.update_vectors();
    }

    fn clamp_pitch(&mut self) {
        self.pitch = clamp(self.pitch, -89.0, 89.0)
    }

    fn update_vectors(&mut self) {
        self.front = vector![
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos()
        ]
        .unit();
        self.right = self.front.cross(&self.world_up).unit();
        self.up = self.right.cross(&self.front).unit();
    }

    pub fn view(&self) -> Matrix {
        Matrix::look_at(&self.position, &(&self.position + &self.front), &self.up)
    }

    pub fn proj(&self) -> Matrix {
        Matrix::projection_perspective(self.fov.to_radians(), self.aspect, self.near, self.far)
    }

    pub fn update_pos(&mut self, time_delta: f64, window: &glfw::Window) {
        let speed = self.speed * time_delta as f32;
        if window.get_key(glfw::Key::W) == glfw::Action::Press {
            self.position += speed * &self.front;
        }
        if window.get_key(glfw::Key::S) == glfw::Action::Press {
            self.position -= speed * &self.front;
        }
        if window.get_key(glfw::Key::A) == glfw::Action::Press {
            self.position -= speed * &self.right;
        }
        if window.get_key(glfw::Key::D) == glfw::Action::Press {
            self.position += speed * &self.right;
        }
    }

    pub fn cursor_pos_callback(&mut self, x_offset: f64, y_offset: f64) {
        self.set_yaw_w_offset(x_offset as f32 * self.rotation_sensitivity);
        self.set_pitch_w_offset(y_offset as f32 * self.rotation_sensitivity);
    }

    pub fn scroll_callback(&mut self, _x_offset: f64, y_offset: f64) {
        self.fov -= y_offset as f32;
        self.fov = clamp(self.fov, self.min_fov, self.max_fov)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            1920.0 / 1080.0,
            3.0,
            vector![0.0, 0.0, 5.0],
            vector![0.0, 1.0, 0.0],
            0.05,
            -90.0,
            0.0,
            0.1,
            100.0,
            70.0,
            1.0,
            70.0,
        )
    }
}
