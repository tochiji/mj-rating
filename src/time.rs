use chrono_tz::Asia::Tokyo;

use chrono::TimeZone;

pub(crate) fn get_unix_time(year: i32) -> i64 {
    let d = Tokyo.with_ymd_and_hms(year, 1, 1, 0, 0, 0);
    let d = d.single().unwrap();
    d.timestamp()
}
