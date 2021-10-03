#[macro_export]
macro_rules! error {
    ($message:expr) => {
        logger::crate_error!("{}", $message);
    };
    ($message:expr, $var:expr) => {
        logger::crate_error!("{:<size$}{:?}", $message, $var, size = SIZE);
    };
}

#[macro_export]
macro_rules! warn {
    ($message:expr) => {
        logger::crate_warn!("{}", $message);
    };
    ($message:expr, $var:expr) => {
        logger::crate_warn!("{:<size$}{:?}", $message, $var, size = SIZE);
    };
}

#[macro_export]
macro_rules! info {
    ($message:expr) => {
        logger::crate_info!("{}", $message);
    };
    ($message:expr, $var:expr) => {
        logger::crate_info!("{:<size$}{}", $message, $var, size = SIZE);
    };
}

pub fn init_logger(){
    env_logger::builder().filter_level(log::LevelFilter::Debug).init();
}