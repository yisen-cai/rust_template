use log::{debug, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::Config;
// use log4rs::encode::pattern::PatternEncoder;
use log::error;
use log::info;
use log::warn;

#[allow(dead_code)]
fn main() {
    let stdout = ConsoleAppender::builder().build();
    let info_log = FileAppender::builder()
        // .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("logs/info.log").unwrap();
    let debug_log = FileAppender::builder()
        .build("logs/debug.log").unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("info_log", Box::new(info_log)))
        .appender(Appender::builder().build("debug_log", Box::new(debug_log)))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("info_log")
                .appender("debug_log")
                .build(LevelFilter::Trace))
        .unwrap();


    let _handle = log4rs::init_config(config).unwrap();

    debug!("Mary has a little lamb");
    error!("{}", "Its fleece was white as snow");
    info!("{:?}", "And every where that Mary went");
    warn!("{:#?}", "The lamb was sure to go");
}