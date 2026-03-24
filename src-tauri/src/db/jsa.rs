use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JsaTemplate {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub trade: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsaInstance {
    pub id: i64,
    pub template_id: Option<i64>,
    pub establishment_id: i64,
    pub location_id: Option<i64>,
    pub job_name: String,
    pub job_date: String,
    pub prepared_by: String,
    pub reviewed_by: Option<String>,
    pub approved_by: Option<String>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsaStep {
    pub id: i64,
    pub jsa_instance_id: i64,
    pub step_number: i64,
    pub task_description: String,
    pub is_completed: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateJsaInstance {
    pub template_id: Option<i64>,
    pub establishment_id: i64,
    pub location_id: Option<i64>,
    pub job_name: String,
    pub job_date: String,
    pub prepared_by: String,
}

#[derive(Debug, Deserialize)]
pub struct AddJsaStep {
    pub jsa_instance_id: i64,
    pub step_number: i64,
    pub task_description: String,
}

// Templates
pub fn list_templates(conn: &Connection) -> Result<Vec<JsaTemplate>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, description, trade, is_active
         FROM jsa_templates WHERE is_active = 1 ORDER BY name",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(JsaTemplate {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            trade: row.get(3)?,
            is_active: row.get::<_, i64>(4)? == 1,
        })
    })?;

    rows.collect()
}

// Instances
pub fn create_instance(conn: &Connection, data: CreateJsaInstance) -> Result<JsaInstance> {
    conn.execute(
        "INSERT INTO jsa_instances (template_id, establishment_id, location_id, job_name, job_date, prepared_by)
         VALUES (?, ?, ?, ?, ?, ?)",
        params![
            data.template_id,
            data.establishment_id,
            data.location_id,
            data.job_name,
            data.job_date,
            data.prepared_by,
        ],
    )?;

    let id = conn.last_insert_rowid();
    get_instance(conn, id)
}

pub fn get_instance(conn: &Connection, id: i64) -> Result<JsaInstance> {
    conn.query_row(
        "SELECT id, template_id, establishment_id, location_id, job_name, job_date,
                prepared_by, reviewed_by, approved_by, status
         FROM jsa_instances WHERE id = ?",
        [id],
        |row| {
            Ok(JsaInstance {
                id: row.get(0)?,
                template_id: row.get(1)?,
                establishment_id: row.get(2)?,
                location_id: row.get(3)?,
                job_name: row.get(4)?,
                job_date: row.get(5)?,
                prepared_by: row.get(6)?,
                reviewed_by: row.get(7)?,
                approved_by: row.get(8)?,
                status: row.get(9)?,
            })
        },
    )
}

pub fn list_instances(conn: &Connection, establishment_id: i64) -> Result<Vec<JsaInstance>> {
    let mut stmt = conn.prepare(
        "SELECT id, template_id, establishment_id, location_id, job_name, job_date,
                prepared_by, reviewed_by, approved_by, status
         FROM jsa_instances WHERE establishment_id = ? ORDER BY job_date DESC",
    )?;

    let rows = stmt.query_map([establishment_id], |row| {
        Ok(JsaInstance {
            id: row.get(0)?,
            template_id: row.get(1)?,
            establishment_id: row.get(2)?,
            location_id: row.get(3)?,
            job_name: row.get(4)?,
            job_date: row.get(5)?,
            prepared_by: row.get(6)?,
            reviewed_by: row.get(7)?,
            approved_by: row.get(8)?,
            status: row.get(9)?,
        })
    })?;

    rows.collect()
}

pub fn update_instance_status(conn: &Connection, id: i64, status: &str) -> Result<JsaInstance> {
    conn.execute(
        "UPDATE jsa_instances SET status = ? WHERE id = ?",
        params![status, id],
    )?;
    get_instance(conn, id)
}

// Steps
pub fn add_step(conn: &Connection, data: AddJsaStep) -> Result<JsaStep> {
    conn.execute(
        "INSERT INTO jsa_steps (jsa_instance_id, step_number, task_description)
         VALUES (?, ?, ?)",
        params![
            data.jsa_instance_id,
            data.step_number,
            data.task_description
        ],
    )?;

    let id = conn.last_insert_rowid();
    get_step(conn, id)
}

pub fn get_step(conn: &Connection, id: i64) -> Result<JsaStep> {
    conn.query_row(
        "SELECT id, jsa_instance_id, step_number, task_description, is_completed
         FROM jsa_steps WHERE id = ?",
        [id],
        |row| {
            Ok(JsaStep {
                id: row.get(0)?,
                jsa_instance_id: row.get(1)?,
                step_number: row.get(2)?,
                task_description: row.get(3)?,
                is_completed: row.get::<_, i64>(4)? == 1,
            })
        },
    )
}

pub fn list_steps(conn: &Connection, jsa_instance_id: i64) -> Result<Vec<JsaStep>> {
    let mut stmt = conn.prepare(
        "SELECT id, jsa_instance_id, step_number, task_description, is_completed
         FROM jsa_steps WHERE jsa_instance_id = ? ORDER BY step_number",
    )?;

    let rows = stmt.query_map([jsa_instance_id], |row| {
        Ok(JsaStep {
            id: row.get(0)?,
            jsa_instance_id: row.get(1)?,
            step_number: row.get(2)?,
            task_description: row.get(3)?,
            is_completed: row.get::<_, i64>(4)? == 1,
        })
    })?;

    rows.collect()
}

pub fn toggle_step_completion(conn: &Connection, id: i64) -> Result<JsaStep> {
    conn.execute(
        "UPDATE jsa_steps SET is_completed = NOT is_completed WHERE id = ?",
        [id],
    )?;
    get_step(conn, id)
}
