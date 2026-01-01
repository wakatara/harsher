use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq)]
pub struct Habit {
    /// Optional section heading (e.g., "! Dailies")
    pub heading: Option<String>,

    /// Name of the habit (e.g., "Meditated")
    pub name: String,

    /// Raw frequency string (e.g., "1", "3/7", "1w")
    pub frequency: String,

    /// Parsed target count (e.g., 3 in "3/7")
    pub target: i32,

    /// Parsed interval days (e.g., 7 in "3/7")
    pub interval: i32,

    /// Date of first recorded entry for this habit
    pub first_record: Option<NaiveDate>,
}
