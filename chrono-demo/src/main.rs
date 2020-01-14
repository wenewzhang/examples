extern crate chrono;
// extern crate time;
use chrono::prelude::*;
use chrono::{Utc, TimeZone};
// use time::Duration;

fn main() {
    let utc: DateTime<Utc> = Utc::now();       // e.g. `2014-11-28T12:45:59.324310806Z`
    // let local: DateTime<Local> = Local::now();
    println!("{}",utc);

    let utc2: DateTime<Utc> = Utc::now();
    println!("{}",utc2);
    if utc2 > utc {
        println!("Yes great than!")
    }
    println!("{}",utc.timestamp());

    let utc4 =  Utc::now().date();
    println!("{}",utc4);

    let tms = Utc.timestamp(61, 0);
    println!("{}",tms);

    let tms_now = Utc::now().timestamp();
    println!("tmps: {}",tms_now);

    let days =  Utc::now().timestamp() / (24*60*60);
    println!("days:{}", days);

    let tms_now2 = tms_now + 1*24*60*60;

    println!("tmps 2: {}",tms_now2);

    let utc_after1 = Utc.timestamp(tms_now,0);
    println!("utc_after1: {}",utc_after1);

    let utc_after2 = Utc.timestamp(tms_now2,0);
    println!("utc_after2: {}",utc_after2);

    // let dt = Utc.timestamp_millis(1111111);
    // println!("{}",dt);
    let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(tms_now2, 0), Utc);
    println!("{}",dt);
    // let mut date = TimeZone.from_utc_datetime(&"2017-10-28T22:00:00Z".parse::<DateTime<Utc>>().unwrap().naive_utc());
    // assume this returned `2014-11-28T21:45:59.324310806+09:00`:
    // let dt = FixedOffset::east(9*3600).ymd(2014, 11, 28).and_hms_nano(21, 45, 59, 324310806);

    // property accessors
    // assert_eq!((dt.year(), dt.month(), dt.day()), (2014, 11, 28));
    // let utc3 = Utc.ymd(Utc.today(utc),10,10).and_hms(1,1,1);
}
