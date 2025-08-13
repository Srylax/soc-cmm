//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.

mod control;
mod overview;
mod sidebar;
mod ui;
mod chart;
mod import_export;
mod settings;
mod score;
mod profile;
mod report;
pub use control::ControlsListComponent;
pub use overview::OverviewComponent;
pub use sidebar::SidebarComponent;
pub use ui::*;
pub use chart::ChartComponent;
pub use import_export::ImportExportComponent;
pub use score::{ScoreComponent, SidebarScoreComponent, CompletenessScoreComponent};
pub use settings::{SettingsComponent, AppSettings};
pub use profile::{ProfileComponent};
pub use report::{PrintOverviewComponent, ProfileValuesComponent};
