use crate::errors::AppError;
use rusqlite::{params, Connection};
use serde::Serialize;
use std::sync::Mutex;
use tauri::State;

type DbState = Mutex<Connection>;

#[derive(Debug, Serialize)]
pub struct DashboardSummary {
    pub total_incidents: i64,
    pub open_incidents: i64,
    pub total_recordable: i64,
    pub days_since_last_incident: Option<i64>,
    pub trir: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct IncidentsByMonth {
    pub month: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct IncidentsBySeverity {
    pub severity: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct IncidentsByLocation {
    pub location_name: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct IncidentsByType {
    pub injury_type: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct CorrectiveActionSummary {
    pub open: i64,
    pub in_progress: i64,
    pub completed: i64,
    pub overdue: i64,
}

#[tauri::command]
pub fn get_dashboard_summary(
    db: State<'_, DbState>,
    establishment_id: i64,
    year: i64,
) -> Result<DashboardSummary, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    let year_str = format!("{year}%");

    let (total, open, recordable): (i64, i64, i64) = conn
        .query_row(
            "SELECT
                COUNT(*),
                COALESCE(SUM(CASE WHEN status = 'open' THEN 1 ELSE 0 END), 0),
                COALESCE(SUM(CASE WHEN is_recordable = 1 THEN 1 ELSE 0 END), 0)
             FROM incidents
             WHERE establishment_id = ?1 AND incident_date LIKE ?2",
            params![establishment_id, year_str],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .map_err(AppError::Database)?;

    let days_since: Option<i64> = conn
        .query_row(
            "SELECT CAST(julianday('now') - julianday(MAX(incident_date)) AS INTEGER)
             FROM incidents WHERE establishment_id = ?1",
            [establishment_id],
            |row| row.get(0),
        )
        .ok()
        .flatten();

    // TRIR = (Total Recordable Incidents * 200,000) / Total Hours Worked
    let trir: Option<f64> = conn
        .query_row(
            "SELECT total_hours_worked FROM annual_stats
             WHERE establishment_id = ?1 AND year = ?2",
            params![establishment_id, year],
            |row| row.get::<_, i64>(0),
        )
        .ok()
        .filter(|&hours| hours > 0)
        .map(|hours| (recordable as f64 * 200_000.0) / hours as f64);

    Ok(DashboardSummary {
        total_incidents: total,
        open_incidents: open,
        total_recordable: recordable,
        days_since_last_incident: days_since,
        trir,
    })
}

#[tauri::command]
pub fn get_incidents_by_month(
    db: State<'_, DbState>,
    establishment_id: i64,
    year: i64,
) -> Result<Vec<IncidentsByMonth>, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    let year_str = format!("{year}%");

    let mut stmt = conn
        .prepare(
            "SELECT strftime('%Y-%m', incident_date) as month, COUNT(*) as cnt
             FROM incidents
             WHERE establishment_id = ?1 AND incident_date LIKE ?2
             GROUP BY month ORDER BY month",
        )
        .map_err(AppError::Database)?;

    let rows = stmt
        .query_map(params![establishment_id, year_str], |row| {
            Ok(IncidentsByMonth {
                month: row.get(0)?,
                count: row.get(1)?,
            })
        })
        .map_err(AppError::Database)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(AppError::Database)?;

    Ok(rows)
}

#[tauri::command]
pub fn get_incidents_by_severity(
    db: State<'_, DbState>,
    establishment_id: i64,
    year: i64,
) -> Result<Vec<IncidentsBySeverity>, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    let year_str = format!("{year}%");

    let mut stmt = conn
        .prepare(
            "SELECT outcome_severity, COUNT(*)
             FROM incidents
             WHERE establishment_id = ?1 AND incident_date LIKE ?2 AND is_recordable = 1
             GROUP BY outcome_severity",
        )
        .map_err(AppError::Database)?;

    let rows = stmt
        .query_map(params![establishment_id, year_str], |row| {
            Ok(IncidentsBySeverity {
                severity: row.get(0)?,
                count: row.get(1)?,
            })
        })
        .map_err(AppError::Database)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(AppError::Database)?;

    Ok(rows)
}

#[tauri::command]
pub fn get_incidents_by_location(
    db: State<'_, DbState>,
    establishment_id: i64,
    year: i64,
) -> Result<Vec<IncidentsByLocation>, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    let year_str = format!("{year}%");

    let mut stmt = conn
        .prepare(
            "SELECT COALESCE(l.name, 'Unassigned'), COUNT(*)
             FROM incidents i
             LEFT JOIN locations l ON i.location_id = l.id
             WHERE i.establishment_id = ?1 AND i.incident_date LIKE ?2
             GROUP BY l.name",
        )
        .map_err(AppError::Database)?;

    let rows = stmt
        .query_map(params![establishment_id, year_str], |row| {
            Ok(IncidentsByLocation {
                location_name: row.get(0)?,
                count: row.get(1)?,
            })
        })
        .map_err(AppError::Database)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(AppError::Database)?;

    Ok(rows)
}

#[tauri::command]
pub fn get_incidents_by_type(
    db: State<'_, DbState>,
    establishment_id: i64,
    year: i64,
) -> Result<Vec<IncidentsByType>, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;
    let year_str = format!("{year}%");

    let mut stmt = conn
        .prepare(
            "SELECT injury_illness_type, COUNT(*)
             FROM incidents
             WHERE establishment_id = ?1 AND incident_date LIKE ?2 AND is_recordable = 1
             GROUP BY injury_illness_type",
        )
        .map_err(AppError::Database)?;

    let rows = stmt
        .query_map(params![establishment_id, year_str], |row| {
            Ok(IncidentsByType {
                injury_type: row.get(0)?,
                count: row.get(1)?,
            })
        })
        .map_err(AppError::Database)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(AppError::Database)?;

    Ok(rows)
}

#[tauri::command]
pub fn get_corrective_action_summary(
    db: State<'_, DbState>,
    establishment_id: i64,
) -> Result<CorrectiveActionSummary, AppError> {
    let conn = db.lock().map_err(|e| AppError::Internal(e.to_string()))?;

    let (open, in_progress, completed, overdue): (i64, i64, i64, i64) = conn
        .query_row(
            "SELECT
                COALESCE(SUM(CASE WHEN ca.status = 'open' THEN 1 ELSE 0 END), 0),
                COALESCE(SUM(CASE WHEN ca.status = 'in_progress' THEN 1 ELSE 0 END), 0),
                COALESCE(SUM(CASE WHEN ca.status = 'completed' THEN 1 ELSE 0 END), 0),
                COALESCE(SUM(CASE WHEN ca.status != 'completed' AND ca.due_date < date('now') THEN 1 ELSE 0 END), 0)
             FROM corrective_actions ca
             JOIN incidents i ON ca.incident_id = i.id
             WHERE i.establishment_id = ?1",
            [establishment_id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
        )
        .map_err(AppError::Database)?;

    Ok(CorrectiveActionSummary {
        open,
        in_progress,
        completed,
        overdue,
    })
}
