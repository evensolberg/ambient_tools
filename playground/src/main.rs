use chrono::TimeZone;
use chrono_tz::OffsetComponents;

fn main() {
    // let tz_name = iana_time_zone::get_timezone().unwrap_or_default();
    let tz_name = String::from("Europe/London");

    let offset = get_offset_from_tz(&tz_name).unwrap_or_default();

    println!("Timezone: {tz_name}");
    println!("Offset:   {offset}");
}

/// Calculate the offset from UTC for a given timezone based on the IANA timezone input as a string
fn get_offset_from_tz(tz_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let tz: chrono_tz::Tz = tz_name.parse()?;
    let local_time = chrono::Local::now();
    let tz_offset = tz.offset_from_utc_datetime(&local_time.naive_utc());
    let offset = tz_offset.base_utc_offset() + tz_offset.dst_offset();
    let offset_secs = offset.num_seconds();
    let offset_hrs = offset_secs / 3600;
    let offset_mins = (offset_secs % 3600) / 60;

    Ok(format!("{offset_hrs:>+03}:{offset_mins:02}"))
}
