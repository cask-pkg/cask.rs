#![deny(warnings)]

use chrono::prelude::{DateTime, Utc};

pub fn iso8601(st: &std::time::SystemTime) -> String {
    let dt: DateTime<Utc> = (*st).into();
    format!("{}", dt.format("%+"))
    // formats like "2001-07-08T00:34:60.026490+09:30"
}
