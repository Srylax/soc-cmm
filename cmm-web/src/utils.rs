use cmm_core::{data::SOCData, schema::Schema, score::Stats};
use dioxus::{hooks::use_context, signals::Signal};

use crate::components::AppSettings;

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

pub fn use_soc_data() -> Signal<SOCData> {
    use_context::<(Signal<SOCData>, Signal<SOCData>)>().0
}

pub fn use_soc_compare_data() -> Signal<SOCData> {
    use_context::<(Signal<SOCData>, Signal<SOCData>)>().1
}

pub fn use_app_settings() -> Signal<AppSettings> {
    use_context::<Signal<AppSettings>>()
}

pub fn use_schema() -> Schema {
    use_context::<Schema>()
}

pub fn use_stats() -> (Signal<Stats>, Signal<Stats>) {
    use_context::<(Signal<Stats>, Signal<Stats>)>()
}
