

#[macro_export]
macro_rules! error {
    ($message:expr) => {
        logger::crate_error!("{}", $message);
    };
    ($message:expr, $($var:expr)?) => {
        logger::crate_error!("{:<size$}{:?}", $message, $($var)?, size = SIZE);
    };
}

#[macro_export]
macro_rules! warn {
    ($message:expr) => {
        logger::crate_warn!("{:<size$}", $message, size = SIZE);
    };
    ($message:expr, $var:expr) => {
        logger::crate_warn!("{:<size$}{:?}", $message, $var, size = SIZE);
    };
}

#[macro_export]
macro_rules! info {
    ($message:expr) => {
        logger::crate_info!("{:<size$}", $message, size = SIZE);
    };
    ($message:expr, $var:expr) => {
        logger::crate_info!("{:<size$}{}", $message, $var, size = SIZE);
    };
}
