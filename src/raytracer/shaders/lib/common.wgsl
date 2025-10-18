fn rand(seed: u32) -> f32 {
    var x = seed;
    x = (x ^ 61u) ^ (x >> 16u);
    x *= 9u;
    x = x ^ (x >> 4u);
    x *= 0x27d4eb2du;
    x = x ^ (x >> 15u);
    return f32(x & 0x00FFFFFFu) / f32(0x01000000u);
}

// Generate a random unit vector using spherical coordinates
fn random_unit_vec3(seed: u32) -> vec3<f32> {
    let theta = 2.0 * 3.14159265359 * rand(seed + 1u);
    let z = 2.0 * rand(seed + 2u) - 1.0; // in [-1,1]
    let r = sqrt(1.0 - z*z);
    let x = r * cos(theta);
    let y = r * sin(theta);
    return vec3<f32>(x, y, z);
}
