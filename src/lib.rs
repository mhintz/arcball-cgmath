extern crate cgmath;
extern crate num;

use cgmath::*;

// Trait names are fully qualified to make it clear where they come from
pub struct ArcballCamera<T: cgmath::BaseFloat + num::Float> {
  p_mouse: Vector2<T>,
  target: Vector3<T>,
  rotation: Basis3<T>,
  distance: T,
  spin_speed: T,
  zoom_speed: T,
  pan_speed: T,
  rotating: bool,
  panning: bool,
}

/// Assumes all input x and y coordinates are in normalized screen coordinates [-1, 1] in x and y
impl<T: cgmath::BaseFloat + num::Float> ArcballCamera<T> {
  pub fn new() -> ArcballCamera<T> {
    ArcballCamera {
      p_mouse: Vector2::zero(),
      target: Vector3::zero(),
      rotation: Basis3::one(),
      distance: T::zero(),
      spin_speed: num::cast(5.0).unwrap(),
      zoom_speed: T::one(),
      pan_speed: T::one(),
      rotating: false,
      panning: false,
    }
  }

  pub fn get_transform_mat(& self) -> Matrix4<T> {
    let cam_position: Vector3<T> = -(self.target + self.rotation.rotate_vector(Vector3::new(T::zero(), T::zero(), self.distance)));
    let position_transform = Matrix4::from_translation(cam_position);
    let rotation_transform: Matrix3<T> = self.rotation.invert().into();
    // The normal order of operations (position * rotation * scale) is reversed here, because the matrix is inverted
    Matrix4::from(rotation_transform) * position_transform
  }

  pub fn set_distance(&mut self, distance: T) -> &mut Self {
    self.distance = distance.max(T::zero());
    self
  }

  pub fn set_rotation(&mut self, rotation: Basis3<T>) -> &mut Self {
    self.rotation = rotation;
    self
  }

  pub fn set_target(&mut self, target: Vector3<T>) -> &mut Self {
    self.target = target;
    self
  }

  pub fn set_spin_speed(&mut self, speed: T) -> &mut Self {
    self.spin_speed = speed;
    self
  }

  pub fn set_zoom_speed(&mut self, speed: T) -> &mut Self {
    self.zoom_speed = speed;
    self
  }

  pub fn set_pan_speed(&mut self, speed: T) -> &mut Self {
    self.pan_speed = speed;
    self
  }

  pub fn rotate_start(&mut self, pos: Vector2<T>) {
    self.rotating = true;
    self.p_mouse = pos;
  }

  pub fn rotate_end(&mut self) {
    self.rotating = false;
  }

  pub fn pan_start(&mut self, pos: Vector2<T>) {
    self.panning = true;
    self.p_mouse = pos;
  }

  pub fn pan_end(&mut self) {
    self.panning = false;
  }

  pub fn get_vec_on_ball(input: Vector2<T>) -> Vector3<T> {
    let dist = input.magnitude();
    let point_z = if dist <= T::one() { (T::one() - dist).sqrt() } else { T::zero() };
    Vector3::new(input.x, input.y, point_z).normalize()
  }

  pub fn update(&mut self, cur_mouse: Vector2<T>) {
    if self.rotating {
      let prev_pt = ArcballCamera::get_vec_on_ball(self.p_mouse);
      let cur_pt = ArcballCamera::get_vec_on_ball(cur_mouse);
      let angle = prev_pt.dot(cur_pt).min(T::one()).acos() * self.spin_speed;
      // The order of the cross product here gets you the correct rotation direction
      let rot_vec = cur_pt.cross(prev_pt).normalize();
      let rotation: Basis3<T> = Basis3::from_axis_angle(rot_vec, Rad::new(angle));
      self.rotation = self.rotation.concat(& rotation);
      self.p_mouse = cur_mouse;
    } else if self.panning {
      // Note that the direction of target point movement is the reverse of the direction of mouse movement
      let mouse_vec = -(cur_mouse - self.p_mouse).normalize_to(self.pan_speed);
      let left_vec = self.rotation.rotate_vector(Vector3::new(T::one(), T::zero(), T::zero())).normalize_to(mouse_vec.x);
      let up_vec = self.rotation.rotate_vector(Vector3::new(T::zero(), T::one(), T::zero())).normalize_to(mouse_vec.y);
      self.target = self.target + left_vec + up_vec;
      self.p_mouse = cur_mouse;
    }
  }

  pub fn zoom(&mut self, d: T) {
    self.distance = (self.distance + d * self.zoom_speed).max(T::zero());
  }
}
