use crate::db::incidents::{self, CreateIncident};
use crate::errors::AppError;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::State;

type DbState = Mutex<Connection>;

#[derive(Debug, Serialize)]
pub struct CsvPreview {
    pub headers: Vec<String>,
    pub sample_rows: Vec<Vec<String>>,
    pub total_rows: usize,
}

#[derive(Debug, Deserialize)]
pub struct ColumnMapping {
    pub employee_name: Option<String>,
    pub employee_job_title: Option<String>,
    pub incident_date: Option<String>,
    pub description: Option<String>,
    pub where_occurred: Option<String>,
    pub outcome_severity: Option<String>,
    pub days_away_count: Option<String>,
    pub days_restricted_count: Option<String>,
    pub injury_illness_type: Option<String>,
    pub employee_gender: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ImportResult {
    pub imported: usize,
    pub errors: Vec<String>,
}

#[tauri::command]
pub fn preview_csv(file_path: String) -> Result<CsvPreview, AppError> {
    let mut rdr = csv::Reader::from_path(&file_path)
        .map_err(|e| AppError::Internal(format!("Failed to open CSV: {e}")))?;

    let headers: Vec<String> = rdr
        .headers()
        .map_err(|e| AppError::Internal(format!("Failed to read CSV headers: {e}")))?
        .iter()
        .map(|s| s.to_string())
        .collect();

    let mut sample_rows = Vec::new();
    let mut total_rows = 0;

    for result in rdr.records() {
        let record = result.map_err(|e| AppError::Internal(format!("CSV parse error: {e}")))?;
        total_rows += 1;
        if sample_rows.len() < 5 {
            sample_rows.push(record.iter().map(|s| s.to_string()).collect());
        }
    }

    Ok(CsvPreview {
        headers,
        sample_rows,
        total_rows,
    })
}

#[tauri::command]
pub fn import_csv(
    db: State<'_, DbState>,
    file_path: String,
    establishment_id: i64,
    location_id: Option<i64>,
    mapping: ColumnMapping,
) -> Result<ImportResult, AppError> {
    use crate::validation;

    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;

    let mut rdr = csv::Reader::from_path(&file_path)
        .map_err(|e| AppError::Internal(format!("Failed to open CSV: {e}")))?;

    let headers: Vec<String> = rdr
        .headers()
        .map_err(|e| AppError::Internal(format!("Failed to read CSV headers: {e}")))?
        .iter()
        .map(|s| s.to_string())
        .collect();

    let header_index: HashMap<String, usize> = headers
        .iter()
        .enumerate()
        .map(|(i, h)| (h.clone(), i))
        .collect();

    let mut imported = 0;
    let mut errors = Vec::new();

    for (row_num, result) in rdr.records().enumerate() {
        let record = match result {
            Ok(r) => r,
            Err(e) => {
                errors.push(format!("Row {}: {e}", row_num + 2));
                continue;
            }
        };

        let get_field = |col_name: &Option<String>| -> Option<String> {
            col_name
                .as_ref()
                .and_then(|name| header_index.get(name))
                .and_then(|&idx| record.get(idx))
                .map(|s| s.to_string())
                .filter(|s| !s.is_empty())
        };

        let employee_name = match get_field(&mapping.employee_name) {
            Some(name) => name,
            None => {
                errors.push(format!("Row {}: Missing employee name", row_num + 2));
                continue;
            }
        };

        let incident_date = match get_field(&mapping.incident_date) {
            Some(date) => date,
            None => {
                errors.push(format!("Row {}: Missing incident date", row_num + 2));
                continue;
            }
        };

        let description =
            get_field(&mapping.description).unwrap_or_else(|| "Imported incident".to_string());

        if let Err(e) = validation::validate_date_format(&incident_date, "Incident date") {
            errors.push(format!("Row {}: {e}", row_num + 2));
            continue;
        }

        if let Err(e) = validation::validate_not_empty(&employee_name, "Employee name") {
            errors.push(format!("Row {}: {e}", row_num + 2));
            continue;
        }

        if let Err(e) = validation::validate_string_length(
            &employee_name,
            validation::MAX_NAME_LENGTH,
            "Employee name",
        ) {
            errors.push(format!("Row {}: {e}", row_num + 2));
            continue;
        }

        if let Err(e) = validation::validate_not_empty(&description, "Description") {
            errors.push(format!("Row {}: {e}", row_num + 2));
            continue;
        }

        if let Err(e) = validation::validate_string_length(
            &description,
            validation::MAX_DESCRIPTION_LENGTH,
            "Description",
        ) {
            errors.push(format!("Row {}: {e}", row_num + 2));
            continue;
        }

        let days_away_count = match get_field(&mapping.days_away_count) {
            Some(value) => match value.parse::<i64>() {
                Ok(days) => {
                    if let Err(e) = validation::validate_days_count(days, "Days away from work") {
                        errors.push(format!("Row {}: {e}", row_num + 2));
                        continue;
                    }
                    Some(days)
                }
                Err(_) => {
                    errors.push(format!(
                        "Row {}: Invalid days away count '{}' (expected whole number)",
                        row_num + 2,
                        value
                    ));
                    continue;
                }
            },
            None => None,
        };

        let days_restricted_count = match get_field(&mapping.days_restricted_count) {
            Some(value) => match value.parse::<i64>() {
                Ok(days) => {
                    if let Err(e) = validation::validate_days_count(days, "Days of restricted work")
                    {
                        errors.push(format!("Row {}: {e}", row_num + 2));
                        continue;
                    }
                    Some(days)
                }
                Err(_) => {
                    errors.push(format!(
                        "Row {}: Invalid restricted days count '{}' (expected whole number)",
                        row_num + 2,
                        value
                    ));
                    continue;
                }
            },
            None => None,
        };

        let data = CreateIncident {
            establishment_id,
            location_id,
            employee_name,
            employee_job_title: get_field(&mapping.employee_job_title),
            incident_date,
            description,
            where_occurred: get_field(&mapping.where_occurred),
            outcome_severity: get_field(&mapping.outcome_severity),
            days_away_count,
            days_restricted_count,
            injury_illness_type: get_field(&mapping.injury_illness_type),
            employee_gender: get_field(&mapping.employee_gender),
            employee_address: None,
            employee_city: None,
            employee_state: None,
            employee_zip: None,
            employee_dob: None,
            employee_hire_date: None,
            is_privacy_case: None,
            incident_time: None,
            work_start_time: None,
            activity_before_incident: None,
            how_injury_occurred: None,
            injury_description: None,
            object_substance: None,
            physician_name: None,
            treatment_facility: None,
            facility_address: None,
            facility_city_state_zip: None,
            treated_in_er: None,
            hospitalized_overnight: None,
            date_of_death: None,
            is_recordable: None,
        };

        match incidents::create_incident(&conn, data) {
            Ok(_) => imported += 1,
            Err(e) => errors.push(format!("Row {}: {e}", row_num + 2)),
        }
    }

    Ok(ImportResult { imported, errors })
}
