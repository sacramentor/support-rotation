use std::fmt::Display;

use chrono::{prelude::*, Duration};

enum SupportDev {
    Ricardo,
    Dimitrios,
}

impl SupportDev {
    fn toggle(&self) -> SupportDev {
        match self {
            SupportDev::Ricardo => SupportDev::Dimitrios,
            SupportDev::Dimitrios => SupportDev::Ricardo,
        }
    }
}

impl Display for SupportDev {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            SupportDev::Ricardo => write!(f, "Ricardo"),
            SupportDev::Dimitrios => write!(f, "Dimitrios"),
        };
    }
}

fn main() {
    let mut support = SupportDev::Dimitrios;
    let now = Utc::now();
    let mut start_date = Utc.with_ymd_and_hms(2024, 04, 02, 0, 0, 0).unwrap();

    while start_date.signed_duration_since(now).num_days() < 1 {
        let mut i = 0;
        while i < 8 {
            start_date = start_date + Duration::days(1);
            match start_date.weekday() {
                Weekday::Mon | Weekday::Tue | Weekday::Wed | Weekday::Thu | Weekday::Fri => {
                    i += 1;
                }
                _ => {}
            }
        }
        support = support.toggle();
        println!("{:?}", start_date);
        println!("{}", support);
        println!("-------------------");
    }
}
