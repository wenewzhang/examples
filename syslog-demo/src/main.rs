// extern crate syslog;
//
// use syslog::{Facility, Formatter3164};
//
// fn main() {
//   let formatter = Formatter3164 {
//     facility: Facility::LOG_USER,
//     hostname: None,
//     process: "myprogram".into(),
//     pid: 42,
//   };
//
//   match syslog::unix(formatter) {
//     Err(e)         => println!("impossible to connect to syslog: {:?}", e),
//     Ok(mut writer) => {
//       writer.err("hello world").expect("could not write error message");
//     }
//   }
// }
extern crate syslog;
#[macro_use]
extern crate log;

use syslog::{Facility, Formatter3164, BasicLogger};
use log::{LevelFilter};

fn main() {
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "myprogram".into(),
        pid: 0,
    };

    let logger = syslog::unix(formatter).expect("could not connect to syslog");
    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
            .map(|()| log::set_max_level(LevelFilter::Info));

    info!("hello world");
}
