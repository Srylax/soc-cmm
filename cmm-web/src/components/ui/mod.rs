//! Components::UI is used to defined common UI elements like buttons, forms, and modals.
#![allow(unused_imports)]

mod toggle;
mod star_btn;
mod small_btn;
mod progress_bar;
mod domain_icon;
mod section_title;
pub use toggle::ToggleComponent;
pub use star_btn::StarButtonComponent;
pub use small_btn::SmallButtonComponent;
pub use progress_bar::{BadToGoodProgressBarComponent, ProgressBarComponent};
pub use domain_icon::DomainIconComponent;
pub use section_title::SectionTitleComponent;
