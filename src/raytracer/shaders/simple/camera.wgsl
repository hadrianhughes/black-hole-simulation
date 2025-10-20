struct Camera {
  origin: vec3<f32>,
  bottom_left: vec3<f32>,
  horizontal: vec3<f32>,
  vertical: vec3<f32>,
}

fn get_ray(camera: Camera, u: u32, v: u32) -> Ray {
  return Ray(
    camera.origin,
    camera.bottom_left + f32(u) * camera.horizontal + f32(v) * camera.vertical - camera.origin
  );
}
