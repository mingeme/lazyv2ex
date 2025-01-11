use chrono::{DateTime, Local, Utc};

pub mod time_formatting {
    use super::*;

    pub fn format_relative_time(time: DateTime<Utc>) -> String {
        let now = Local::now();
        let local_time = time.with_timezone(&now.timezone());
        let duration = now.signed_duration_since(local_time);

        if duration.num_days() > 0 {
            format!("{}天前", duration.num_days())
        } else if duration.num_hours() > 0 {
            let hours = duration.num_hours();
            let minutes = duration.num_minutes() % 60;
            if minutes > 0 {
                format!("{}小时{}分钟前", hours, minutes)
            } else {
                format!("{}小时前", hours)
            }
        } else if duration.num_minutes() > 0 {
            format!("{}分钟前", duration.num_minutes())
        } else {
            format!("{}秒前", duration.num_seconds())
        }
    }
}
