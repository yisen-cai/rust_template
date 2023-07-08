mod functions;
mod add_ons;

use log::{debug, info, warn, error};
use add_ons::{};

#[allow(dead_code)]
fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    while true {
        debug!("Mary has a little lamb");
        debug!(target: "debug.log", "Specify debug log!!!");
        error!("{}", "Its fleece was white as snow");
        info!("{:?}", "And every where that Mary went");
        warn!("{:#?}", "The lamb was sure to go");
    }
}