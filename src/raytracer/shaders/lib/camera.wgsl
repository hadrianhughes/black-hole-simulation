struct Camera {
  origin: Vec3,
  bottom_left: Vec3,
  horizontal: Vec3,
  vertical: Vec3,
}

fn get_ray(camera: Camera, u: f32, v: f32) -> Ray {
  return Ray(
    camera.origin,
    camera.bottom_left + u * camera.horizontal + v * camera.vertical - camera.origin
  )
}
