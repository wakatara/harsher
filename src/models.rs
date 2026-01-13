use anyhow::{Result, anyhow};
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
    pub target: u32,
    /// Parsed interval days (e.g., 7 in "3/7")
    pub interval: u32,
    /// Date of first recorded entry for this habit
    pub first_record: Option<NaiveDate>,
    /// Date to end tracking on this habit
    pub end_date: Option<NaiveDate>,
}

impl Habit {
    pub fn new(name: String, frequency: String, end_date: Option<NaiveDate>) -> Result<Self> {
        let (target, interval) = parse_freq(&frequency)?;

        Ok(Self {
            heading: None,
            name,
            frequency,
            target,
            interval,
            first_record: None,
            end_date,
        })
    }
}

fn parse_freq(freq: &str) -> Result<(u32, u32)> {
    // Handle tracking case
    if freq == "0" {
        return Ok((0, 0));
    }

    // Handle "Xw" shorthand for staright dailies (e.g., "2w" = weekly)
    if let Some(num) = freq.strip_suffix('w') {
        let n: u32 = num
            .parse()
            .map_err(|e| anyhow!("Invalid weekly frequency '{}': {}", freq, e))?;

        return Ok((1, n * 7));
    }

    // Handle straight up OG numeric daily intervals
    let num: u32 = freq
        .parse()
        .map_err(|e| anyhow!("Invalid daily frequency '{}': {}", freq, e))?;

    Ok((1, num))
}
