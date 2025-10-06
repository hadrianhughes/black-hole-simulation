pub fn in_range(min: f64, x: f64, max: f64) -> bool {
    x > min && x < max
}

pub fn clamp(min: f64, x: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
