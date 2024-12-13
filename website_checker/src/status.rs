use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct WebsiteStatus {
    pub url: String,
    pub status: Result<u16, String>, // HTTP status code or error message
    pub response_time: std::time::Duration,
    pub timestamp: DateTime<Utc>,
}