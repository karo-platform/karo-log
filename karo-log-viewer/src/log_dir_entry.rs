use std::cmp::Ordering;

use chrono::{DateTime, FixedOffset};

#[derive(Eq, PartialEq)]
pub enum LogFileType {
    Rotated(DateTime<FixedOffset>),
    Live,
}

#[derive(Eq, PartialEq)]
pub struct LogFile {
    pub log_file_name: String,
    pub log_type: LogFileType,
}

impl Ord for LogFile {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.log_type, &other.log_type) {
            (LogFileType::Live, LogFileType::Live) => Ordering::Equal,
            (LogFileType::Rotated(_), LogFileType::Live) => Ordering::Less,
            (LogFileType::Live, LogFileType::Rotated(_)) => Ordering::Greater,
            (LogFileType::Rotated(this), LogFileType::Rotated(other)) => this.cmp(other),
        }
    }
}

impl PartialOrd for LogFile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
