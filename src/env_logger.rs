use env_logger::Env;
use log::{debug, error, info, warn};


#[allow(dead_code)]
fn main() {
    // env_logger::init();
    env_logger::Builder::from_env(
        Env::default().default_filter_or("warn")
    ).init();

    debug!("Mary has a little lamb");
    error!("{}", "Its fleece was white as snow");
    info!("{:?}", "And every where that Mary went");
    warn!("{:#?}", "The lamb was sure to go");
}