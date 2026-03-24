use crate::db::incidents::{self, CreateIncident, Incident, IncidentFilter, UpdateIncident};
use crate::errors::AppError;
use anyhow::Error as AnyhowError;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

type DbState = Mutex<Connection>;

fn map_incident_error(err: AnyhowError) -> AppError {
    match err.downcast::<AppError>() {
        Ok(app_error) => app_error,
        Err(other) => match other.downcast::<rusqlite::Error>() {
            Ok(db_error) => AppError::Database(db_error),
            Err(other) => AppError::Internal(other.to_string()),
        },
    }
}

#[tauri::command]
pub fn create_incident(db: State<'_, DbState>, data: CreateIncident) -> Result<Incident, AppError> {
    use crate::validation;

    // Validate required fields
    validation::validate_not_empty(&data.employee_name, "Employee name")?;
    validation::validate_string_length(
        &data.employee_name,
        validation::MAX_NAME_LENGTH,
        "Employee name",
    )?;
    validation::validate_not_empty(&data.description, "Description")?;
    validation::validate_string_length(
        &data.description,
        validation::MAX_DESCRIPTION_LENGTH,
        "Description",
    )?;

    // Validate date format
    validation::validate_date_format(&data.incident_date, "Incident date")?;

    // Validate days counts
    if let Some(days) = data.days_away_count {
        validation::validate_days_count(days, "Days away from work")?;
    }
    if let Some(days) = data.days_restricted_count {
        validation::validate_days_count(days, "Days of restricted work")?;
    }

    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    incidents::create_incident(&conn, data).map_err(map_incident_error)
}

#[tauri::command]
pub fn get_incident(db: State<'_, DbState>, id: i64) -> Result<Incident, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    incidents::get_incident(&conn, id).map_err(map_incident_error)
}

#[tauri::command]
pub fn list_incidents(
    db: State<'_, DbState>,
    filter: IncidentFilter,
) -> Result<Vec<Incident>, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    incidents::list_incidents(&conn, filter).map_err(map_incident_error)
}

#[tauri::command]
pub fn update_incident(
    db: State<'_, DbState>,
    id: i64,
    data: UpdateIncident,
) -> Result<Incident, AppError> {
    use crate::validation;

    // Validate optional fields if present
    if let Some(ref name) = data.employee_name {
        validation::validate_not_empty(name, "Employee name")?;
        validation::validate_string_length(name, validation::MAX_NAME_LENGTH, "Employee name")?;
    }
    if let Some(ref desc) = data.description {
        validation::validate_not_empty(desc, "Description")?;
        validation::validate_string_length(
            desc,
            validation::MAX_DESCRIPTION_LENGTH,
            "Description",
        )?;
    }
    if let Some(ref date) = data.incident_date {
        validation::validate_date_format(date, "Incident date")?;
    }
    if let Some(days) = data.days_away_count {
        validation::validate_days_count(days, "Days away from work")?;
    }
    if let Some(days) = data.days_restricted_count {
        validation::validate_days_count(days, "Days of restricted work")?;
    }

    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    incidents::update_incident(&conn, id, data).map_err(map_incident_error)
}

#[tauri::command]
pub fn delete_incident(db: State<'_, DbState>, id: i64) -> Result<(), AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    incidents::delete_incident(&conn, id).map_err(map_incident_error)
}
