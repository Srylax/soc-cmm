/// Round to precision
///
/// ```
/// assert_eq!(round(3.149, 1), 3.1);
/// assert_eq!(round(3.14, 0), 3.0);
/// assert_eq!(round(3.1, 5), 3.1);
/// assert_eq!(round(3.149, 2), 3.15);
/// ```
pub fn round(float: f64, precision: u32) -> f64 {
    (float * 10_f64.powf(precision as f64)).round() / 10_f64.powf(precision as f64)
}
