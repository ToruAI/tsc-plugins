use crate::error::{TimerError, TimerResult};

/// Parsed schedule information
#[derive(Debug, Clone, PartialEq)]
pub enum Schedule {
    /// OnCalendar expression (e.g., "Mon-Fri 08-21:00")
    Calendar { expression: String },

    /// OnBootSec (runs N seconds after boot)
    OnBoot { seconds: u64 },

    /// OnUnitActiveSec (runs N seconds after unit activation)
    Recurring { seconds: u64 },

    /// Multiple schedules
    Multiple(Vec<Schedule>),
}

impl Schedule {
    /// Parse a systemd schedule from timer unit properties
    pub fn parse(on_calendar: Option<&str>, on_boot: Option<&str>, on_active: Option<&str>) -> TimerResult<Self> {
        let mut schedules = Vec::new();

        if let Some(expr) = on_calendar {
            schedules.push(Schedule::Calendar {
                expression: expr.to_string(),
            });
        }

        if let Some(expr) = on_boot {
            let seconds = Self::parse_time_span(expr)?;
            schedules.push(Schedule::OnBoot { seconds });
        }

        if let Some(expr) = on_active {
            let seconds = Self::parse_time_span(expr)?;
            schedules.push(Schedule::Recurring { seconds });
        }

        match schedules.len() {
            0 => Err(TimerError::ParseError {
                source: "schedule".to_string(),
                reason: "No schedule information found".to_string(),
            }),
            1 => Ok(schedules.into_iter().next().unwrap()),
            _ => Ok(Schedule::Multiple(schedules)),
        }
    }

    /// Humanize the schedule for display
    pub fn humanize(&self) -> String {
        match self {
            Schedule::Calendar { expression } => Self::humanize_calendar(expression),
            Schedule::OnBoot { seconds } => format!("{} after boot", Self::humanize_duration(*seconds)),
            Schedule::Recurring { seconds } => format!("Every {}", Self::humanize_duration(*seconds)),
            Schedule::Multiple(schedules) => {
                schedules.iter()
                    .map(|s| s.humanize())
                    .collect::<Vec<_>>()
                    .join(", ")
            }
        }
    }

    /// Parse time span (e.g., "5min", "1h", "30s")
    fn parse_time_span(expr: &str) -> TimerResult<u64> {
        let expr = expr.trim();

        if expr.ends_with("min") || expr.ends_with("m") {
            let num_str = expr.trim_end_matches("min").trim_end_matches('m');
            let minutes: u64 = num_str.parse()
                .map_err(|_| TimerError::ParseError {
                    source: "time_span".to_string(),
                    reason: format!("Invalid minutes: {}", expr),
                })?;
            Ok(minutes * 60)
        } else if expr.ends_with("hour") || expr.ends_with("hours") || expr.ends_with("h") {
            let num_str = expr.trim_end_matches("hours").trim_end_matches("hour").trim_end_matches('h');
            let hours: u64 = num_str.parse()
                .map_err(|_| TimerError::ParseError {
                    source: "time_span".to_string(),
                    reason: format!("Invalid hours: {}", expr),
                })?;
            Ok(hours * 3600)
        } else if expr.ends_with("sec") || expr.ends_with("s") {
            let num_str = expr.trim_end_matches("sec").trim_end_matches('s');
            let seconds: u64 = num_str.parse()
                .map_err(|_| TimerError::ParseError {
                    source: "time_span".to_string(),
                    reason: format!("Invalid seconds: {}", expr),
                })?;
            Ok(seconds)
        } else {
            // Assume raw seconds
            expr.parse()
                .map_err(|_| TimerError::ParseError {
                    source: "time_span".to_string(),
                    reason: format!("Invalid time span: {}", expr),
                })
        }
    }

    /// Humanize a duration in seconds
    fn humanize_duration(seconds: u64) -> String {
        if seconds < 60 {
            format!("{}s", seconds)
        } else if seconds < 3600 {
            let minutes = seconds / 60;
            let secs = seconds % 60;
            if secs == 0 {
                format!("{}min", minutes)
            } else {
                format!("{}min {}s", minutes, secs)
            }
        } else if seconds < 86400 {
            let hours = seconds / 3600;
            let minutes = (seconds % 3600) / 60;
            if minutes == 0 {
                format!("{}h", hours)
            } else {
                format!("{}h {}min", hours, minutes)
            }
        } else {
            let days = seconds / 86400;
            let hours = (seconds % 86400) / 3600;
            if hours == 0 {
                format!("{}d", days)
            } else {
                format!("{}d {}h", days, hours)
            }
        }
    }

    /// Humanize OnCalendar expression
    fn humanize_calendar(expression: &str) -> String {
        let expr = expression.trim();

        // Common patterns
        if expr == "*-*-* *:*:*" || expr == "hourly" {
            return "Hourly".to_string();
        }
        if expr == "daily" || expr.starts_with("*-*-*") && expr.contains("00:00") {
            return "Daily at midnight".to_string();
        }
        if expr == "weekly" || expr.starts_with("Mon") && expr.contains("00:00") {
            return "Weekly on Monday".to_string();
        }
        if expr == "monthly" {
            return "Monthly".to_string();
        }

        // Day patterns
        if expr.starts_with("Mon-Fri") {
            let time_part = expr.strip_prefix("Mon-Fri").unwrap_or("").trim();
            if time_part.contains("08-21") || time_part.contains("08:00-21:00") {
                return "Mon-Fri, 8 AM - 9 PM".to_string();
            }
            return format!("Mon-Fri {}", time_part);
        }

        if expr.contains("Mon,Wed,Fri") {
            let time_part = expr.split("Mon,Wed,Fri").nth(1).unwrap_or("").trim();
            return format!("Mon, Wed, Fri {}", time_part);
        }

        // Hourly during specific times
        if expr.contains("*:00:00") || expr.contains("*:00") {
            if expr.contains("08-21") || expr.contains("08:00-21:00") {
                return "Hourly, 8 AM - 9 PM".to_string();
            }
        }

        // Default: return as-is
        expression.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_time_span() {
        assert_eq!(Schedule::parse_time_span("5min").unwrap(), 300);
        assert_eq!(Schedule::parse_time_span("5m").unwrap(), 300);
        assert_eq!(Schedule::parse_time_span("2h").unwrap(), 7200);
        assert_eq!(Schedule::parse_time_span("2hour").unwrap(), 7200);
        assert_eq!(Schedule::parse_time_span("30s").unwrap(), 30);
        assert_eq!(Schedule::parse_time_span("30sec").unwrap(), 30);
        assert_eq!(Schedule::parse_time_span("120").unwrap(), 120);
    }

    #[test]
    fn test_humanize_duration() {
        assert_eq!(Schedule::humanize_duration(30), "30s");
        assert_eq!(Schedule::humanize_duration(60), "1min");
        assert_eq!(Schedule::humanize_duration(90), "1min 30s");
        assert_eq!(Schedule::humanize_duration(3600), "1h");
        assert_eq!(Schedule::humanize_duration(3660), "1h 1min");
        assert_eq!(Schedule::humanize_duration(86400), "1d");
        assert_eq!(Schedule::humanize_duration(90000), "1d 1h");
    }

    #[test]
    fn test_humanize_calendar() {
        assert_eq!(Schedule::humanize_calendar("hourly"), "Hourly");
        assert_eq!(Schedule::humanize_calendar("daily"), "Daily at midnight");
        assert_eq!(Schedule::humanize_calendar("Mon-Fri 08-21:00"), "Mon-Fri, 8 AM - 9 PM");
        assert_eq!(Schedule::humanize_calendar("Mon,Wed,Fri 14:00"), "Mon, Wed, Fri 14:00");
    }

    #[test]
    fn test_parse_schedule() {
        let schedule = Schedule::parse(Some("Mon-Fri 08-21:00"), None, None).unwrap();
        assert!(matches!(schedule, Schedule::Calendar { .. }));
        assert_eq!(schedule.humanize(), "Mon-Fri, 8 AM - 9 PM");

        let schedule = Schedule::parse(None, Some("5min"), None).unwrap();
        assert!(matches!(schedule, Schedule::OnBoot { seconds: 300 }));
        assert_eq!(schedule.humanize(), "5min after boot");

        let schedule = Schedule::parse(None, None, Some("1h")).unwrap();
        assert!(matches!(schedule, Schedule::Recurring { seconds: 3600 }));
        assert_eq!(schedule.humanize(), "Every 1h");
    }

    #[test]
    fn test_parse_multiple_schedules() {
        let schedule = Schedule::parse(Some("hourly"), Some("5min"), None).unwrap();
        assert!(matches!(schedule, Schedule::Multiple(_)));
        let humanized = schedule.humanize();
        assert!(humanized.contains("Hourly"));
        assert!(humanized.contains("5min after boot"));
    }
}
