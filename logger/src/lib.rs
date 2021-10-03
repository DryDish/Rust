mod logger;

pub const SIZE: usize = 45;

pub use log::LevelFilter;
pub use log::{error as crate_error, info as crate_info, warn as crate_warn};

