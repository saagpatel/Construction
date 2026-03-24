use crate::db::osha::{
    self, AnnualStats, Osha300ASummary, Osha300Row, Osha301Report, UpsertAnnualStats,
};
use crate::errors::AppError;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

type DbState = Mutex<Connection>;

#[tauri::command]
pub fn get_osha_300_log(
    db: State<'_, DbState>,
    establishment_id: i64,
    year: i64,
) -> Result<Vec<Osha300Row>, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    osha::get_osha_300_log(&conn, establishment_id, year)
        .map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn get_osha_300a_summary(
    db: State<'_, DbState>,
    establishment_id: i64,
    year: i64,
) -> Result<Osha300ASummary, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    osha::get_osha_300a_summary(&conn, establishment_id, year)
        .map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn get_osha_301_report(
    db: State<'_, DbState>,
    incident_id: i64,
) -> Result<Osha301Report, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    osha::get_osha_301_report(&conn, incident_id).map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn export_osha_300_csv(
    db: State<'_, DbState>,
    establishment_id: i64,
    year: i64,
    establishment_name: String,
) -> Result<String, AppError> {
    use crate::validation;

    // Validate year
    validation::validate_year(year)?;

    // Create safe export path using sanitized establishment name
    let file_base = format!("OSHA_300_{}_{}", establishment_name, year);
    let safe_path = validation::safe_export_path(&file_base, "csv")?;

    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    let rows = osha::get_osha_300_log(&conn, establishment_id, year)
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let mut wtr =
        csv::Writer::from_path(&safe_path).map_err(|e| AppError::Internal(e.to_string()))?;

    wtr.write_record([
        "Case No.",
        "Employee Name",
        "Job Title",
        "Date of Injury/Illness",
        "Where Event Occurred",
        "Description of Injury/Illness",
        "Death",
        "Days Away From Work",
        "Job Transfer or Restriction",
        "Other Recordable Cases",
        "Days Away From Work (Count)",
        "Days of Restricted Work (Count)",
        "Injury",
        "Skin Disorder",
        "Respiratory Condition",
        "Poisoning",
        "Hearing Loss",
        "All Other Illnesses",
    ])
    .map_err(|e| AppError::Internal(e.to_string()))?;

    for row in &rows {
        wtr.write_record([
            &row.case_number.to_string(),
            &row.employee_name,
            &row.job_title,
            &row.incident_date,
            &row.where_occurred,
            &row.description,
            &bool_to_x(row.outcome_death),
            &bool_to_x(row.outcome_days_away),
            &bool_to_x(row.outcome_job_transfer),
            &bool_to_x(row.outcome_other_recordable),
            &row.days_away_count.to_string(),
            &row.days_restricted_count.to_string(),
            &bool_to_x(row.type_injury),
            &bool_to_x(row.type_skin_disorder),
            &bool_to_x(row.type_respiratory),
            &bool_to_x(row.type_poisoning),
            &bool_to_x(row.type_hearing_loss),
            &bool_to_x(row.type_other_illness),
        ])
        .map_err(|e| AppError::Internal(e.to_string()))?;
    }

    wtr.flush().map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(safe_path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn export_osha_300a_csv(
    db: State<'_, DbState>,
    establishment_id: i64,
    year: i64,
    establishment_name: String,
) -> Result<String, AppError> {
    use crate::validation;

    validation::validate_year(year)?;

    let file_base = format!("OSHA_300A_{}_{}", establishment_name, year);
    let safe_path = validation::safe_export_path(&file_base, "csv")?;

    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    let summary = osha::get_osha_300a_summary(&conn, establishment_id, year)
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let mut wtr =
        csv::Writer::from_path(&safe_path).map_err(|e| AppError::Internal(e.to_string()))?;

    wtr.write_record([
        "Year",
        "Establishment Name",
        "Street Address",
        "City",
        "State",
        "ZIP",
        "Industry Description",
        "NAICS Code",
        "Total Deaths",
        "Total Days Away Cases",
        "Total Transfer/Restriction Cases",
        "Total Other Recordable Cases",
        "Total Days Away",
        "Total Days Restricted",
        "Total Injuries",
        "Total Skin Disorders",
        "Total Respiratory",
        "Total Poisonings",
        "Total Hearing Loss",
        "Total Other Illnesses",
        "Average Employees",
        "Total Hours Worked",
        "Certifier Name",
        "Certifier Title",
        "Certifier Phone",
        "Certification Date",
    ])
    .map_err(|e| AppError::Internal(e.to_string()))?;

    wtr.write_record([
        &summary.year.to_string(),
        &summary.establishment_name,
        &summary.street_address,
        &summary.city,
        &summary.state,
        &summary.zip_code,
        &summary.industry_description,
        &summary.naics_code,
        &summary.total_deaths.to_string(),
        &summary.total_days_away_cases.to_string(),
        &summary.total_transfer_restriction_cases.to_string(),
        &summary.total_other_recordable_cases.to_string(),
        &summary.total_days_away.to_string(),
        &summary.total_days_restricted.to_string(),
        &summary.total_injuries.to_string(),
        &summary.total_skin_disorders.to_string(),
        &summary.total_respiratory.to_string(),
        &summary.total_poisonings.to_string(),
        &summary.total_hearing_loss.to_string(),
        &summary.total_other_illnesses.to_string(),
        &summary.avg_employees.unwrap_or_default().to_string(),
        &summary.total_hours_worked.unwrap_or_default().to_string(),
        summary.certifier_name.as_deref().unwrap_or(""),
        summary.certifier_title.as_deref().unwrap_or(""),
        summary.certifier_phone.as_deref().unwrap_or(""),
        summary.certification_date.as_deref().unwrap_or(""),
    ])
    .map_err(|e| AppError::Internal(e.to_string()))?;

    wtr.flush().map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(safe_path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn export_osha_301_csv(db: State<'_, DbState>, incident_id: i64) -> Result<String, AppError> {
    use crate::validation;

    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    let report = osha::get_osha_301_report(&conn, incident_id)
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let file_base = format!("OSHA_301_Case_{}", report.case_number);
    let safe_path = validation::safe_export_path(&file_base, "csv")?;

    let mut wtr =
        csv::Writer::from_path(&safe_path).map_err(|e| AppError::Internal(e.to_string()))?;

    wtr.write_record([
        "Case No.",
        "Employee Name",
        "Employee Address",
        "Employee City",
        "Employee State",
        "Employee ZIP",
        "Employee DOB",
        "Employee Hire Date",
        "Employee Gender",
        "Physician Name",
        "Treatment Facility",
        "Facility Address",
        "Facility City/State/ZIP",
        "Treated In ER",
        "Hospitalized Overnight",
        "Incident Date",
        "Incident Time",
        "Work Start Time",
        "Where Occurred",
        "Activity Before Incident",
        "How Injury Occurred",
        "Injury Description",
        "Object/Substance",
        "Date Of Death",
        "Completed By",
        "Completed By Title",
        "Completed By Phone",
        "Completed Date",
    ])
    .map_err(|e| AppError::Internal(e.to_string()))?;

    wtr.write_record([
        &report.case_number.to_string(),
        &report.employee_name,
        &report.employee_address,
        &report.employee_city,
        &report.employee_state,
        &report.employee_zip,
        &report.employee_dob,
        &report.employee_hire_date,
        &report.employee_gender,
        &report.physician_name,
        &report.treatment_facility,
        &report.facility_address,
        &report.facility_city_state_zip,
        &bool_to_x(report.treated_in_er),
        &bool_to_x(report.hospitalized_overnight),
        &report.incident_date,
        &report.incident_time,
        &report.work_start_time,
        &report.where_occurred,
        &report.activity_before_incident,
        &report.how_injury_occurred,
        &report.injury_description,
        &report.object_substance,
        &report.date_of_death,
        &report.completed_by,
        &report.completed_by_title,
        &report.completed_by_phone,
        &report.completed_date,
    ])
    .map_err(|e| AppError::Internal(e.to_string()))?;

    wtr.flush().map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(safe_path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn upsert_annual_stats(
    db: State<'_, DbState>,
    data: UpsertAnnualStats,
) -> Result<AnnualStats, AppError> {
    use crate::validation;

    // Validate year
    validation::validate_year(data.year)?;

    // Validate employee count and hours
    validation::validate_employee_count(data.avg_employees)?;
    validation::validate_hours_worked(data.total_hours_worked)?;

    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    osha::upsert_annual_stats(&conn, data).map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn get_annual_stats(
    db: State<'_, DbState>,
    establishment_id: i64,
    year: i64,
) -> Result<Option<AnnualStats>, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    osha::get_annual_stats(&conn, establishment_id, year)
        .map_err(|e| AppError::Internal(e.to_string()))
}

fn bool_to_x(v: bool) -> String {
    if v {
        "X".to_string()
    } else {
        String::new()
    }
}
