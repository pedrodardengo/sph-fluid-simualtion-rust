use std::f32::consts::PI;


pub fn spiky_smoothing_kernel(distance: f32, smoothing_radius: f32) -> f32 {
  if distance >= smoothing_radius {
    return 0.0;
  }
  let volume = PI * smoothing_radius.powi(6) / 15.0;
  (smoothing_radius - distance).powi(3) / volume
}

pub fn spiky_smoothing_kernel_derivative(distance: f32, smoothing_radius: f32) -> f32 {
  if distance >= smoothing_radius {
    return 0.0;
  }
  let scale = - 60.0 / (PI * smoothing_radius.powi(6));
  scale * (smoothing_radius - distance)
}

pub fn viscosity_smoothing_kernel_second_derivative(distance: f32, smoothing_radius: f32) -> f32 {
  if distance >= smoothing_radius {
    return 0.0;
  }
  let volume = (PI * smoothing_radius.powi(6)) / 45.0;
  (smoothing_radius - distance) / volume
}