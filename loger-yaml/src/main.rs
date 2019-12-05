#[macro_use]
extern crate log;
extern crate log4rs;

use std::default::Default;
use std::thread;
use std::time::Duration;

fn main() {
    log4rs::init_file("config/log4rs.yaml",Default::default()).unwrap();
    for i in 1..2{
        info!("booting up {}",i);
        error!("error test {}",i);
    }
	//无限循环，不断记录日志
    loop{
        thread::sleep(Duration::from_secs(1));
        info!("booting up ");
        error!("error test");
        // error!("Goes to stderr and file");
        warn!("Goes to stderr and file");
        // info!("Goes to stderr and file");
        debug!("Goes to file only");
        trace!("Goes to file only");
    }
}
