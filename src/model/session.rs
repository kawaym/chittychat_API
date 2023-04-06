use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct Session {
    pub id: i32,
    pub date_of_creation: DateTime<Utc>,
    pub hash: String,
}