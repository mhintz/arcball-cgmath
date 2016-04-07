extern crate cgmath;

use cgmath::*;

pub struct ArcballCamera {
  p_mouse: Vector2<f32>,
  target: Vector3<f32>,
  rotation: Basis3<f32>,
  distance: f32,
  spin_speed: f32,
  zoom_speed: f32,
  pan_speed: f32,
  rotating: bool,
  panning: bool,
}

/// Assumes all input x and y coordinates are in normalized screen coordinates [-1, 1] in x and y
impl ArcballCamera {
  pub fn new() -> ArcballCamera {
    ArcballCamera {
      p_mouse: Vector2::zero(),
      target: Vector3::zero(),
      rotation: Basis3::one(),
      distance: 0.0,
      spin_speed: 5.0,
      zoom_speed: 1.0,
      pan_speed: 1.0,
      rotating: false,
      panning: false,
    }
  }

  pub fn get_transform_mat(& self) -> Matrix4<f32> {
    let cam_position: Vector3<f32> = -(self.target + self.rotation.rotate_vector(Vector3::new(0.0, 0.0, self.distance)));
    let position_transform = Matrix4::from_translation(cam_position);

    let rotation_transform: Matrix3<f32> = self.rotation.invert().into();
    Matrix4::from(rotation_transform) * position_transform
  }

  pub fn set_distance(&mut self, distance: f32) -> &mut Self {
    self.distance = distance.max(0.0);
    self
  }

  pub fn set_rotation(&mut self, rotation: Basis3<f32>) -> &mut Self {
    self.rotation = rotation;
    self
  }

  pub fn set_target(&mut self, target: Vector3<f32>) -> &mut Self {
    self.target = target;
    self
  }

  pub fn rotate_start(&mut self, pos: Vector2<f32>) {
    self.rotating = true;
    self.p_mouse = pos;
  }

  pub fn rotate_end(&mut self) {
    self.rotating = false;
  }

  pub fn pan_start(&mut self, pos: Vector2<f32>) {
    self.panning = true;
    self.p_mouse = pos;
  }

  pub fn pan_end(&mut self) {
    self.panning = false;
  }

  pub fn get_vec_on_ball(input: Vector2<f32>) -> Vector3<f32> {
    let dist = input.magnitude();
    let point_z = if dist <= 1.0 { (1.0 - dist).sqrt() } else { 0.0 };
    Vector3::new(input.x, input.y, point_z).normalize()
  }

  pub fn update(&mut self, cur_mouse: Vector2<f32>) {
    if self.rotating {
      let prev_pt = ArcballCamera::get_vec_on_ball(self.p_mouse);
      let cur_pt = ArcballCamera::get_vec_on_ball(cur_mouse);
      let angle = prev_pt.dot(cur_pt).min(1.0).acos() * self.spin_speed;
      let rot_vec = prev_pt.cross(cur_pt).normalize();
      let rotation: Basis3<f32> = Basis3::from_axis_angle(rot_vec, Rad::new(angle));
      self.rotation = self.rotation.concat(& rotation);
      self.p_mouse = cur_mouse;
    } else if self.panning {
      // Note that the direction of target point movement is the reverse of the direction of mouse movement
      let mouse_vec = -(cur_mouse - self.p_mouse).normalize_to(self.pan_speed);
      let left_vec = self.rotation.rotate_vector(Vector3::new(1.0, 0.0, 0.0)).normalize_to(mouse_vec.x);
      let up_vec = self.rotation.rotate_vector(Vector3::new(0.0, 1.0, 0.0)).normalize_to(mouse_vec.y);
      self.target = self.target + left_vec + up_vec;
      self.p_mouse = cur_mouse;
    }
  }

  pub fn zoom(&mut self, d: f32) {
    self.distance = (self.distance + d * self.zoom_speed).max(0.0);
  }
}
