use crate::errors::AppError;
use std::path::PathBuf;

/// Maximum string lengths for various fields
pub const MAX_NAME_LENGTH: usize = 255;
pub const MAX_DESCRIPTION_LENGTH: usize = 5000;

/// OSHA maximum days away from work or restricted (180 days per regulations)
pub const MAX_OSHA_DAYS: i64 = 180;

/// Reasonable year range for incident tracking (1970-2100)
pub const MIN_YEAR: i64 = 1970;
pub const MAX_YEAR: i64 = 2100;

/// Validates that days count is non-negative and within OSHA limits
pub fn validate_days_count(days: i64, field_name: &str) -> Result<(), AppError> {
    if days < 0 {
        return Err(AppError::Validation(format!(
            "{} cannot be negative (got: {})",
            field_name, days
        )));
    }
    if days > MAX_OSHA_DAYS {
        return Err(AppError::Validation(format!(
            "{} exceeds OSHA maximum of {} days (got: {})",
            field_name, MAX_OSHA_DAYS, days
        )));
    }
    Ok(())
}

/// Validates that a year is within reasonable range
pub fn validate_year(year: i64) -> Result<(), AppError> {
    if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
        return Err(AppError::Validation(format!(
            "Year must be between {} and {} (got: {})",
            MIN_YEAR, MAX_YEAR, year
        )));
    }
    Ok(())
}

/// Validates that employee count is positive
pub fn validate_employee_count(count: i64) -> Result<(), AppError> {
    if count < 0 {
        return Err(AppError::Validation(format!(
            "Employee count cannot be negative (got: {})",
            count
        )));
    }
    if count > 1_000_000 {
        return Err(AppError::Validation(format!(
            "Employee count seems unrealistic (got: {})",
            count
        )));
    }
    Ok(())
}

/// Validates that hours worked is positive
pub fn validate_hours_worked(hours: i64) -> Result<(), AppError> {
    if hours < 0 {
        return Err(AppError::Validation(format!(
            "Total hours worked cannot be negative (got: {})",
            hours
        )));
    }
    // 1M employees * 2080 hours/year = 2.08B hours max reasonable
    if hours > 2_100_000_000 {
        return Err(AppError::Validation(format!(
            "Total hours worked seems unrealistic (got: {})",
            hours
        )));
    }
    Ok(())
}

/// Validates string length
pub fn validate_string_length(
    s: &str,
    max_length: usize,
    field_name: &str,
) -> Result<(), AppError> {
    if s.len() > max_length {
        return Err(AppError::Validation(format!(
            "{} exceeds maximum length of {} characters (got: {} characters)",
            field_name,
            max_length,
            s.len()
        )));
    }
    Ok(())
}

/// Validates that a string is not empty
pub fn validate_not_empty(s: &str, field_name: &str) -> Result<(), AppError> {
    if s.trim().is_empty() {
        return Err(AppError::Validation(format!(
            "{} cannot be empty",
            field_name
        )));
    }
    Ok(())
}

/// Validates a date string is in YYYY-MM-DD format
pub fn validate_date_format(date: &str, field_name: &str) -> Result<(), AppError> {
    if date.len() != 10 {
        return Err(AppError::Validation(format!(
            "{} must be in YYYY-MM-DD format (got: {})",
            field_name, date
        )));
    }

    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 {
        return Err(AppError::Validation(format!(
            "{} must be in YYYY-MM-DD format (got: {})",
            field_name, date
        )));
    }

    // Validate year
    let year = parts[0].parse::<i32>().map_err(|_| {
        AppError::Validation(format!(
            "Invalid year in {} (got: {})",
            field_name, parts[0]
        ))
    })?;
    if !(1970..=2100).contains(&year) {
        return Err(AppError::Validation(format!(
            "Year in {} must be between 1970 and 2100 (got: {})",
            field_name, year
        )));
    }

    // Validate month
    let month = parts[1].parse::<u32>().map_err(|_| {
        AppError::Validation(format!(
            "Invalid month in {} (got: {})",
            field_name, parts[1]
        ))
    })?;
    if !(1..=12).contains(&month) {
        return Err(AppError::Validation(format!(
            "Month in {} must be between 01 and 12 (got: {})",
            field_name, parts[1]
        )));
    }

    // Validate day
    let day = parts[2].parse::<u32>().map_err(|_| {
        AppError::Validation(format!("Invalid day in {} (got: {})", field_name, parts[2]))
    })?;
    if !(1..=31).contains(&day) {
        return Err(AppError::Validation(format!(
            "Day in {} must be between 01 and 31 (got: {})",
            field_name, parts[2]
        )));
    }

    // Basic leap year and days-in-month validation
    let max_day = match month {
        2 => {
            if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                29
            } else {
                28
            }
        }
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    };

    if day > max_day {
        return Err(AppError::Validation(format!(
            "Invalid day for month in {} (day {} doesn't exist in month {})",
            field_name, day, month
        )));
    }

    Ok(())
}

/// Sanitizes a filename to prevent path traversal attacks
/// Removes path separators, null bytes, and other dangerous characters
pub fn sanitize_filename(name: &str) -> String {
    name.chars()
        .filter(|c| {
            !matches!(
                c,
                '/' | '\\' | '\0' | ':' | '*' | '?' | '"' | '<' | '>' | '|'
            )
        })
        .take(MAX_NAME_LENGTH)
        .collect::<String>()
        .replace("..", "")
        .trim()
        .to_string()
}

/// Validates and constructs a safe export file path within Downloads directory
pub fn safe_export_path(base_name: &str, extension: &str) -> Result<PathBuf, AppError> {
    let sanitized = sanitize_filename(base_name);

    if sanitized.is_empty() {
        return Err(AppError::Validation(
            "Filename would be empty after sanitization".to_string(),
        ));
    }

    // Get user's home directory
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| AppError::Internal("Could not determine home directory".to_string()))?;

    let downloads_dir = PathBuf::from(&home).join("Downloads");

    // Ensure Downloads directory exists
    if !downloads_dir.exists() {
        std::fs::create_dir_all(&downloads_dir).map_err(|e| {
            AppError::Internal(format!("Failed to create Downloads directory: {}", e))
        })?;
    }

    // Construct final path with sanitized name
    let filename = format!("{}.{}", sanitized, extension.trim_start_matches('.'));
    let mut path = downloads_dir.clone();
    path.push(filename);

    // Final safety check: ensure resolved path is still within Downloads
    let canonical_downloads = downloads_dir
        .canonicalize()
        .map_err(|e| AppError::Internal(format!("Failed to resolve Downloads path: {}", e)))?;

    let canonical_target = path
        .parent()
        .ok_or_else(|| AppError::Internal("Invalid path".to_string()))?
        .canonicalize()
        .map_err(|e| AppError::Internal(format!("Failed to resolve target path: {}", e)))?;

    if !canonical_target.starts_with(&canonical_downloads) {
        return Err(AppError::Validation(
            "Export path outside of Downloads directory is not allowed".to_string(),
        ));
    }

    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_days_count() {
        assert!(validate_days_count(0, "days").is_ok());
        assert!(validate_days_count(180, "days").is_ok());
        assert!(validate_days_count(-1, "days").is_err());
        assert!(validate_days_count(181, "days").is_err());
    }

    #[test]
    fn test_validate_year() {
        assert!(validate_year(2024).is_ok());
        assert!(validate_year(1970).is_ok());
        assert!(validate_year(2100).is_ok());
        assert!(validate_year(1969).is_err());
        assert!(validate_year(2101).is_err());
    }

    #[test]
    fn test_validate_date_format() {
        assert!(validate_date_format("2024-01-15", "date").is_ok());
        assert!(validate_date_format("2024-02-29", "date").is_ok()); // leap year
        assert!(validate_date_format("2023-02-29", "date").is_err()); // not leap year
        assert!(validate_date_format("2024-13-01", "date").is_err()); // invalid month
        assert!(validate_date_format("2024-01-32", "date").is_err()); // invalid day
        assert!(validate_date_format("not-a-date", "date").is_err());
        assert!(validate_date_format("2024/01/15", "date").is_err());
    }

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("valid-name"), "valid-name");
        assert_eq!(sanitize_filename("../../etc/passwd"), "etcpasswd");
        assert_eq!(sanitize_filename("file/with\\slashes"), "filewithslashes");
        assert_eq!(sanitize_filename("test<>:|?*.txt"), "test.txt");
        assert_eq!(sanitize_filename("  spaces  "), "spaces");
    }

    #[test]
    fn test_validate_employee_count() {
        assert!(validate_employee_count(0).is_ok());
        assert!(validate_employee_count(100).is_ok());
        assert!(validate_employee_count(-1).is_err());
        assert!(validate_employee_count(2_000_000).is_err());
    }

    #[test]
    fn test_validate_hours_worked() {
        assert!(validate_hours_worked(0).is_ok());
        assert!(validate_hours_worked(100_000).is_ok());
        assert!(validate_hours_worked(-1).is_err());
        assert!(validate_hours_worked(3_000_000_000).is_err());
    }
}
