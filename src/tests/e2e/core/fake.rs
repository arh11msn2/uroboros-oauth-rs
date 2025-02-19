use chrono::Utc;

pub fn get_timestamp_postfix() -> String {
    format!(" {}", Utc::now().timestamp())
}
