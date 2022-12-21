use chrono::{Local, NaiveDateTime};

pub fn current_naive_time() -> NaiveDateTime {
    Local::now().naive_local()
}
