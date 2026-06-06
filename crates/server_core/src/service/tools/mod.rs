pub mod calendar;
pub mod timestamp;

#[derive(Clone)]
pub struct ToolsService {
    pub timestamp: timestamp::TimestampService,
    pub calendar: calendar::CalendarService,
}

impl ToolsService {
    pub fn new() -> Self {
        Self {
            timestamp: timestamp::TimestampService::new(),
            calendar: calendar::CalendarService::new(),
        }
    }
}
