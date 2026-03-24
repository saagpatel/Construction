use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

use crate::errors::AppError;

// ── RCA Sessions ──

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RcaSession {
    pub id: i64,
    pub incident_id: i64,
    pub method: String,
    pub status: String,
    pub root_cause_summary: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FiveWhysStep {
    pub id: i64,
    pub rca_session_id: i64,
    pub step_number: i64,
    pub question: String,
    pub answer: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FishboneCategory {
    pub id: i64,
    pub rca_session_id: i64,
    pub category: String,
    pub sort_order: i64,
    pub causes: Vec<FishboneCause>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FishboneCause {
    pub id: i64,
    pub category_id: i64,
    pub cause_text: String,
    pub is_root_cause: bool,
    pub sort_order: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CorrectiveAction {
    pub id: i64,
    pub incident_id: i64,
    pub rca_session_id: Option<i64>,
    pub description: String,
    pub assigned_to: Option<String>,
    pub due_date: Option<String>,
    pub status: String,
    pub completed_date: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateRcaSession {
    pub incident_id: i64,
    pub method: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateFiveWhysStep {
    pub rca_session_id: i64,
    pub step_number: i64,
    pub question: String,
    pub answer: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateFishboneCategory {
    pub rca_session_id: i64,
    pub category: String,
    pub sort_order: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateFishboneCause {
    pub category_id: i64,
    pub cause_text: String,
    pub is_root_cause: Option<bool>,
    pub sort_order: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCorrectiveAction {
    pub incident_id: i64,
    pub rca_session_id: Option<i64>,
    pub description: String,
    pub assigned_to: Option<String>,
    pub due_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCorrectiveAction {
    pub description: Option<String>,
    pub assigned_to: Option<String>,
    pub due_date: Option<String>,
    pub status: Option<String>,
    pub completed_date: Option<String>,
    pub notes: Option<String>,
}

// ── RCA Session CRUD ──

pub fn create_rca_session(conn: &Connection, data: CreateRcaSession) -> Result<RcaSession> {
    conn.execute(
        "INSERT INTO rca_sessions (incident_id, method) VALUES (?1, ?2)",
        params![data.incident_id, data.method],
    )
    .context("Failed to create RCA session")?;

    let id = conn.last_insert_rowid();
    get_rca_session(conn, id)
}

pub fn get_rca_session(conn: &Connection, id: i64) -> Result<RcaSession> {
    conn.query_row(
        "SELECT id, incident_id, method, status, root_cause_summary, created_at, updated_at
         FROM rca_sessions WHERE id = ?1",
        [id],
        |row| {
            Ok(RcaSession {
                id: row.get(0)?,
                incident_id: row.get(1)?,
                method: row.get(2)?,
                status: row.get(3)?,
                root_cause_summary: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        },
    )
    .map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => {
            AppError::NotFound(format!("RCA session {id} not found")).into()
        }
        _ => anyhow::Error::new(e),
    })
}

pub fn list_rca_sessions(conn: &Connection, incident_id: i64) -> Result<Vec<RcaSession>> {
    let mut stmt = conn.prepare(
        "SELECT id, incident_id, method, status, root_cause_summary, created_at, updated_at
         FROM rca_sessions WHERE incident_id = ?1 ORDER BY created_at DESC",
    )?;

    let rows = stmt
        .query_map([incident_id], |row| {
            Ok(RcaSession {
                id: row.get(0)?,
                incident_id: row.get(1)?,
                method: row.get(2)?,
                status: row.get(3)?,
                root_cause_summary: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(rows)
}

pub fn complete_rca_session(
    conn: &Connection,
    id: i64,
    root_cause_summary: &str,
) -> Result<RcaSession> {
    let changes = conn
        .execute(
            "UPDATE rca_sessions SET status = 'completed', root_cause_summary = ?1, updated_at = datetime('now')
             WHERE id = ?2",
            params![root_cause_summary, id],
        )
        .context("Failed to complete RCA session")?;

    if changes == 0 {
        return Err(AppError::NotFound(format!("RCA session {id} not found")).into());
    }
    get_rca_session(conn, id)
}

pub fn delete_rca_session(conn: &Connection, id: i64) -> Result<()> {
    let changes = conn
        .execute("DELETE FROM rca_sessions WHERE id = ?1", [id])
        .context("Failed to delete RCA session")?;

    if changes == 0 {
        return Err(AppError::NotFound(format!("RCA session {id} not found")).into());
    }
    Ok(())
}

// ── Five Whys ──

pub fn add_five_whys_step(conn: &Connection, data: CreateFiveWhysStep) -> Result<FiveWhysStep> {
    conn.execute(
        "INSERT INTO five_whys_steps (rca_session_id, step_number, question, answer)
         VALUES (?1, ?2, ?3, ?4)",
        params![
            data.rca_session_id,
            data.step_number,
            data.question,
            data.answer
        ],
    )
    .context("Failed to add 5 Whys step")?;

    let id = conn.last_insert_rowid();
    conn.query_row(
        "SELECT id, rca_session_id, step_number, question, answer FROM five_whys_steps WHERE id = ?1",
        [id],
        |row| {
            Ok(FiveWhysStep {
                id: row.get(0)?,
                rca_session_id: row.get(1)?,
                step_number: row.get(2)?,
                question: row.get(3)?,
                answer: row.get(4)?,
            })
        },
    )
    .map_err(anyhow::Error::new)
}

pub fn list_five_whys_steps(conn: &Connection, rca_session_id: i64) -> Result<Vec<FiveWhysStep>> {
    let mut stmt = conn.prepare(
        "SELECT id, rca_session_id, step_number, question, answer
         FROM five_whys_steps WHERE rca_session_id = ?1 ORDER BY step_number",
    )?;

    let rows = stmt
        .query_map([rca_session_id], |row| {
            Ok(FiveWhysStep {
                id: row.get(0)?,
                rca_session_id: row.get(1)?,
                step_number: row.get(2)?,
                question: row.get(3)?,
                answer: row.get(4)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(rows)
}

pub fn update_five_whys_step(
    conn: &Connection,
    id: i64,
    question: &str,
    answer: &str,
) -> Result<FiveWhysStep> {
    let changes = conn
        .execute(
            "UPDATE five_whys_steps SET question = ?1, answer = ?2 WHERE id = ?3",
            params![question, answer, id],
        )
        .context("Failed to update 5 Whys step")?;

    if changes == 0 {
        return Err(AppError::NotFound(format!("Five Whys step {id} not found")).into());
    }

    conn.query_row(
        "SELECT id, rca_session_id, step_number, question, answer FROM five_whys_steps WHERE id = ?1",
        [id],
        |row| {
            Ok(FiveWhysStep {
                id: row.get(0)?,
                rca_session_id: row.get(1)?,
                step_number: row.get(2)?,
                question: row.get(3)?,
                answer: row.get(4)?,
            })
        },
    )
    .map_err(anyhow::Error::new)
}

// ── Fishbone ──

pub fn add_fishbone_category(
    conn: &Connection,
    data: CreateFishboneCategory,
) -> Result<FishboneCategory> {
    conn.execute(
        "INSERT INTO fishbone_categories (rca_session_id, category, sort_order)
         VALUES (?1, ?2, ?3)",
        params![
            data.rca_session_id,
            data.category,
            data.sort_order.unwrap_or(0)
        ],
    )
    .context("Failed to add fishbone category")?;

    let id = conn.last_insert_rowid();
    Ok(FishboneCategory {
        id,
        rca_session_id: data.rca_session_id,
        category: data.category,
        sort_order: data.sort_order.unwrap_or(0),
        causes: vec![],
    })
}

pub fn list_fishbone_categories(
    conn: &Connection,
    rca_session_id: i64,
) -> Result<Vec<FishboneCategory>> {
    let mut cat_stmt = conn.prepare(
        "SELECT id, rca_session_id, category, sort_order
         FROM fishbone_categories WHERE rca_session_id = ?1 ORDER BY sort_order",
    )?;

    let mut cause_stmt = conn.prepare(
        "SELECT id, category_id, cause_text, is_root_cause, sort_order
         FROM fishbone_causes WHERE category_id = ?1 ORDER BY sort_order",
    )?;

    let cats: Vec<FishboneCategory> = cat_stmt
        .query_map([rca_session_id], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, i64>(3)?,
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|(id, rca_id, category, sort_order)| {
            let causes = cause_stmt
                .query_map([id], |row| {
                    Ok(FishboneCause {
                        id: row.get(0)?,
                        category_id: row.get(1)?,
                        cause_text: row.get(2)?,
                        is_root_cause: row.get::<_, i32>(3)? != 0,
                        sort_order: row.get(4)?,
                    })
                })
                .ok()
                .map(|rows| rows.filter_map(|r| r.ok()).collect())
                .unwrap_or_default();

            FishboneCategory {
                id,
                rca_session_id: rca_id,
                category,
                sort_order,
                causes,
            }
        })
        .collect();

    Ok(cats)
}

pub fn add_fishbone_cause(conn: &Connection, data: CreateFishboneCause) -> Result<FishboneCause> {
    conn.execute(
        "INSERT INTO fishbone_causes (category_id, cause_text, is_root_cause, sort_order)
         VALUES (?1, ?2, ?3, ?4)",
        params![
            data.category_id,
            data.cause_text,
            data.is_root_cause.unwrap_or(false) as i32,
            data.sort_order.unwrap_or(0),
        ],
    )
    .context("Failed to add fishbone cause")?;

    let id = conn.last_insert_rowid();
    Ok(FishboneCause {
        id,
        category_id: data.category_id,
        cause_text: data.cause_text,
        is_root_cause: data.is_root_cause.unwrap_or(false),
        sort_order: data.sort_order.unwrap_or(0),
    })
}

pub fn update_fishbone_cause(
    conn: &Connection,
    id: i64,
    cause_text: Option<&str>,
    is_root_cause: Option<bool>,
) -> Result<FishboneCause> {
    if let Some(text) = cause_text {
        conn.execute(
            "UPDATE fishbone_causes SET cause_text = ?1 WHERE id = ?2",
            params![text, id],
        )
        .context("Failed to update fishbone cause")?;
    }
    if let Some(root) = is_root_cause {
        conn.execute(
            "UPDATE fishbone_causes SET is_root_cause = ?1 WHERE id = ?2",
            params![root as i32, id],
        )
        .context("Failed to update fishbone cause")?;
    }

    conn.query_row(
        "SELECT id, category_id, cause_text, is_root_cause, sort_order FROM fishbone_causes WHERE id = ?1",
        [id],
        |row| {
            Ok(FishboneCause {
                id: row.get(0)?,
                category_id: row.get(1)?,
                cause_text: row.get(2)?,
                is_root_cause: row.get::<_, i32>(3)? != 0,
                sort_order: row.get(4)?,
            })
        },
    )
    .map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => {
            AppError::NotFound(format!("Fishbone cause {id} not found")).into()
        }
        _ => anyhow::Error::new(e),
    })
}

pub fn delete_fishbone_cause(conn: &Connection, id: i64) -> Result<()> {
    let changes = conn.execute("DELETE FROM fishbone_causes WHERE id = ?1", [id])?;
    if changes == 0 {
        return Err(AppError::NotFound(format!("Fishbone cause {id} not found")).into());
    }
    Ok(())
}

// ── Corrective Actions ──

pub fn create_corrective_action(
    conn: &Connection,
    data: CreateCorrectiveAction,
) -> Result<CorrectiveAction> {
    conn.execute(
        "INSERT INTO corrective_actions (incident_id, rca_session_id, description, assigned_to, due_date)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            data.incident_id,
            data.rca_session_id,
            data.description,
            data.assigned_to,
            data.due_date,
        ],
    )
    .context("Failed to create corrective action")?;

    let id = conn.last_insert_rowid();
    get_corrective_action(conn, id)
}

pub fn get_corrective_action(conn: &Connection, id: i64) -> Result<CorrectiveAction> {
    conn.query_row(
        "SELECT id, incident_id, rca_session_id, description, assigned_to, due_date,
                status, completed_date, notes, created_at, updated_at
         FROM corrective_actions WHERE id = ?1",
        [id],
        |row| {
            Ok(CorrectiveAction {
                id: row.get(0)?,
                incident_id: row.get(1)?,
                rca_session_id: row.get(2)?,
                description: row.get(3)?,
                assigned_to: row.get(4)?,
                due_date: row.get(5)?,
                status: row.get(6)?,
                completed_date: row.get(7)?,
                notes: row.get(8)?,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
            })
        },
    )
    .map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => {
            AppError::NotFound(format!("Corrective action {id} not found")).into()
        }
        _ => anyhow::Error::new(e),
    })
}

pub fn list_corrective_actions(
    conn: &Connection,
    incident_id: i64,
) -> Result<Vec<CorrectiveAction>> {
    let mut stmt = conn.prepare(
        "SELECT id, incident_id, rca_session_id, description, assigned_to, due_date,
                status, completed_date, notes, created_at, updated_at
         FROM corrective_actions WHERE incident_id = ?1 ORDER BY created_at",
    )?;

    let rows = stmt
        .query_map([incident_id], |row| {
            Ok(CorrectiveAction {
                id: row.get(0)?,
                incident_id: row.get(1)?,
                rca_session_id: row.get(2)?,
                description: row.get(3)?,
                assigned_to: row.get(4)?,
                due_date: row.get(5)?,
                status: row.get(6)?,
                completed_date: row.get(7)?,
                notes: row.get(8)?,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(rows)
}

pub fn update_corrective_action(
    conn: &Connection,
    id: i64,
    data: UpdateCorrectiveAction,
) -> Result<CorrectiveAction> {
    let _existing = get_corrective_action(conn, id)?;

    let mut sets = Vec::new();
    let mut values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    if let Some(ref v) = data.description {
        sets.push("description = ?");
        values.push(Box::new(v.clone()));
    }
    if let Some(ref v) = data.assigned_to {
        sets.push("assigned_to = ?");
        values.push(Box::new(v.clone()));
    }
    if let Some(ref v) = data.due_date {
        sets.push("due_date = ?");
        values.push(Box::new(v.clone()));
    }
    if let Some(ref v) = data.status {
        sets.push("status = ?");
        values.push(Box::new(v.clone()));
    }
    if let Some(ref v) = data.completed_date {
        sets.push("completed_date = ?");
        values.push(Box::new(v.clone()));
    }
    if let Some(ref v) = data.notes {
        sets.push("notes = ?");
        values.push(Box::new(v.clone()));
    }

    if !sets.is_empty() {
        sets.push("updated_at = datetime('now')");
        let sql = format!(
            "UPDATE corrective_actions SET {} WHERE id = ?",
            sets.join(", ")
        );
        values.push(Box::new(id));
        let params: Vec<&dyn rusqlite::types::ToSql> = values.iter().map(|v| v.as_ref()).collect();
        conn.execute(&sql, params.as_slice())
            .context("Failed to update corrective action")?;
    }

    get_corrective_action(conn, id)
}

pub fn delete_corrective_action(conn: &Connection, id: i64) -> Result<()> {
    let changes = conn
        .execute("DELETE FROM corrective_actions WHERE id = ?1", [id])
        .context("Failed to delete corrective action")?;

    if changes == 0 {
        return Err(AppError::NotFound(format!("Corrective action {id} not found")).into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::incidents::{create_incident, CreateIncident};
    use crate::db::locations::{
        create_establishment, create_location, CreateEstablishment, CreateLocation,
    };
    use crate::db::open_test_db;

    fn setup(conn: &Connection) -> i64 {
        let est = create_establishment(
            conn,
            CreateEstablishment {
                name: "Test Co".into(),
                street_address: None,
                city: None,
                state: None,
                zip_code: None,
                industry_description: None,
                naics_code: None,
            },
        )
        .unwrap();
        let loc = create_location(
            conn,
            CreateLocation {
                establishment_id: est.id,
                name: "Site".into(),
                address: None,
                city: None,
                state: None,
            },
        )
        .unwrap();
        let inc = create_incident(
            conn,
            CreateIncident {
                establishment_id: est.id,
                location_id: Some(loc.id),
                employee_name: "Jane".into(),
                incident_date: "2026-01-01".into(),
                description: "Test".into(),
                employee_job_title: None,
                employee_address: None,
                employee_city: None,
                employee_state: None,
                employee_zip: None,
                employee_dob: None,
                employee_hire_date: None,
                employee_gender: None,
                is_privacy_case: None,
                incident_time: None,
                work_start_time: None,
                where_occurred: None,
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
                outcome_severity: None,
                days_away_count: None,
                days_restricted_count: None,
                date_of_death: None,
                injury_illness_type: None,
                is_recordable: None,
            },
        )
        .unwrap();
        inc.id
    }

    #[test]
    fn test_five_whys_flow() {
        let conn = open_test_db();
        let inc_id = setup(&conn);

        let session = create_rca_session(
            &conn,
            CreateRcaSession {
                incident_id: inc_id,
                method: "five_whys".into(),
            },
        )
        .unwrap();
        assert_eq!(session.status, "in_progress");

        add_five_whys_step(
            &conn,
            CreateFiveWhysStep {
                rca_session_id: session.id,
                step_number: 1,
                question: "Why did the worker fall?".into(),
                answer: "The scaffold was not secured".into(),
            },
        )
        .unwrap();

        let steps = list_five_whys_steps(&conn, session.id).unwrap();
        assert_eq!(steps.len(), 1);

        let completed =
            complete_rca_session(&conn, session.id, "Inadequate scaffold inspection").unwrap();
        assert_eq!(completed.status, "completed");
    }

    #[test]
    fn test_corrective_actions() {
        let conn = open_test_db();
        let inc_id = setup(&conn);

        let action = create_corrective_action(
            &conn,
            CreateCorrectiveAction {
                incident_id: inc_id,
                rca_session_id: None,
                description: "Implement scaffold checklist".into(),
                assigned_to: Some("Safety Manager".into()),
                due_date: Some("2026-02-01".into()),
            },
        )
        .unwrap();
        assert_eq!(action.status, "open");

        let updated = update_corrective_action(
            &conn,
            action.id,
            UpdateCorrectiveAction {
                status: Some("completed".into()),
                completed_date: Some("2026-01-25".into()),
                description: None,
                assigned_to: None,
                due_date: None,
                notes: None,
            },
        )
        .unwrap();
        assert_eq!(updated.status, "completed");

        let actions = list_corrective_actions(&conn, inc_id).unwrap();
        assert_eq!(actions.len(), 1);
    }
}
