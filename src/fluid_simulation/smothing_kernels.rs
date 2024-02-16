use std::f32::consts::PI;


pub fn spiky_smoothing_kernel(distance: f32, smoothing_radius: f32) -> f32 {
  if distance >= smoothing_radius {
    return 0.0;
  }
  let volume = PI * smoothing_radius.powi(6) / 15.0;
  (smoothing_radius - distance).powi(3) / volume
}

pub fn cubic_spline_smoothing_kernel(distance: f32, smoothing_radius: f32) -> f32 {
  let ratio = distance / smoothing_radius;
  let normalization = 15.0 / (14.0 * PI * smoothing_radius.powi(2));
  if 0.0 <= ratio && ratio < 1.0 {
    return normalization * ((2.0 - ratio).powi(3) - 4.0 *(1.0 - ratio).powi(3)) 
  }
  if 1.0 <= ratio && ratio < 2.0 {
    return normalization * (2.0 - ratio).powi(3) 
  }
  return 0.0;
}

pub fn cubic_spline_smoothing_kernel_derivative(distance: f32, smoothing_radius: f32) -> f32 {
  let ratio = distance / smoothing_radius;
  let normalization = 15.0 / (14.0 * PI * smoothing_radius.powi(2));
  if 0.0 <= ratio && ratio < 1.0 {
    return normalization * (9.0 * ratio.powi(2) - 12.0 * ratio)
  }
  if 1.0 <= ratio && ratio < 2.0 {
    return 3.0 * normalization * (2.0 - ratio).powi(2) 
  }
  return 0.0;
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