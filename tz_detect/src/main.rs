use chrono::{self, Offset};

fn main() {
    let local_time = chrono::Local::now();
    println!("Local time: {local_time}");

    let utc_time = chrono::Utc::now();
    println!("UTC time:   {utc_time}\n");

    let local_offset = local_time.offset().fix().local_minus_utc();
    println!("Local offset:          {local_offset} seconds");

    let local_tz = local_time.offset();
    println!("Local timezone offset: {local_tz} (HH:MM)");

    if let Ok(tz_name) = iana_time_zone::get_timezone() {
        println!("Local timezone name:   {tz_name}");
    } else {
        println!("Unable to determine local timezone name.");
    }
}
