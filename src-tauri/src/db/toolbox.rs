use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolboxTalkTopic {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub content: String,
    pub category: Option<String>,
    pub duration_minutes: i64,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolboxTalk {
    pub id: i64,
    pub topic_id: Option<i64>,
    pub establishment_id: i64,
    pub location_id: Option<i64>,
    pub title: String,
    pub date: String,
    pub conducted_by: String,
    pub notes: Option<String>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolboxTalkAttendee {
    pub id: i64,
    pub talk_id: i64,
    pub employee_name: String,
    pub employee_id: Option<String>,
    pub signature_data: Option<String>,
    pub signed_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateToolboxTalk {
    pub topic_id: Option<i64>,
    pub establishment_id: i64,
    pub location_id: Option<i64>,
    pub title: String,
    pub date: String,
    pub conducted_by: String,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AddAttendee {
    pub talk_id: i64,
    pub employee_name: String,
    pub employee_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SignAttendee {
    pub attendee_id: i64,
    pub signature_data: String,
}

// Topic CRUD
pub fn list_topics(conn: &Connection, include_inactive: bool) -> Result<Vec<ToolboxTalkTopic>> {
    let sql = if include_inactive {
        "SELECT id, title, description, content, category, duration_minutes, is_active
         FROM toolbox_talk_topics ORDER BY category, title"
    } else {
        "SELECT id, title, description, content, category, duration_minutes, is_active
         FROM toolbox_talk_topics WHERE is_active = 1 ORDER BY category, title"
    };

    let mut stmt = conn.prepare(sql)?;
    let rows = stmt.query_map([], |row| {
        Ok(ToolboxTalkTopic {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            content: row.get(3)?,
            category: row.get(4)?,
            duration_minutes: row.get(5)?,
            is_active: row.get::<_, i64>(6)? == 1,
        })
    })?;

    rows.collect()
}

pub fn get_topic(conn: &Connection, id: i64) -> Result<ToolboxTalkTopic> {
    conn.query_row(
        "SELECT id, title, description, content, category, duration_minutes, is_active
         FROM toolbox_talk_topics WHERE id = ?",
        [id],
        |row| {
            Ok(ToolboxTalkTopic {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                content: row.get(3)?,
                category: row.get(4)?,
                duration_minutes: row.get(5)?,
                is_active: row.get::<_, i64>(6)? == 1,
            })
        },
    )
}

// Talk CRUD
pub fn create_talk(conn: &Connection, data: CreateToolboxTalk) -> Result<ToolboxTalk> {
    conn.execute(
        "INSERT INTO toolbox_talks (topic_id, establishment_id, location_id, title, date, conducted_by, notes)
         VALUES (?, ?, ?, ?, ?, ?, ?)",
        params![
            data.topic_id,
            data.establishment_id,
            data.location_id,
            data.title,
            data.date,
            data.conducted_by,
            data.notes,
        ],
    )?;

    let id = conn.last_insert_rowid();
    get_talk(conn, id)
}

pub fn get_talk(conn: &Connection, id: i64) -> Result<ToolboxTalk> {
    conn.query_row(
        "SELECT id, topic_id, establishment_id, location_id, title, date, conducted_by, notes, status
         FROM toolbox_talks WHERE id = ?",
        [id],
        |row| {
            Ok(ToolboxTalk {
                id: row.get(0)?,
                topic_id: row.get(1)?,
                establishment_id: row.get(2)?,
                location_id: row.get(3)?,
                title: row.get(4)?,
                date: row.get(5)?,
                conducted_by: row.get(6)?,
                notes: row.get(7)?,
                status: row.get(8)?,
            })
        },
    )
}

pub fn list_talks(conn: &Connection, establishment_id: i64) -> Result<Vec<ToolboxTalk>> {
    let mut stmt = conn.prepare(
        "SELECT id, topic_id, establishment_id, location_id, title, date, conducted_by, notes, status
         FROM toolbox_talks WHERE establishment_id = ? ORDER BY date DESC",
    )?;

    let rows = stmt.query_map([establishment_id], |row| {
        Ok(ToolboxTalk {
            id: row.get(0)?,
            topic_id: row.get(1)?,
            establishment_id: row.get(2)?,
            location_id: row.get(3)?,
            title: row.get(4)?,
            date: row.get(5)?,
            conducted_by: row.get(6)?,
            notes: row.get(7)?,
            status: row.get(8)?,
        })
    })?;

    rows.collect()
}

pub fn complete_talk(conn: &Connection, talk_id: i64) -> Result<ToolboxTalk> {
    let updated = conn.execute(
        "UPDATE toolbox_talks SET status = 'completed' WHERE id = ?",
        [talk_id],
    )?;

    if updated == 0 {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    get_talk(conn, talk_id)
}

// Attendee management
pub fn add_attendee(conn: &Connection, data: AddAttendee) -> Result<ToolboxTalkAttendee> {
    conn.execute(
        "INSERT INTO toolbox_talk_attendees (talk_id, employee_name, employee_id)
         VALUES (?, ?, ?)",
        params![data.talk_id, data.employee_name, data.employee_id],
    )?;

    let id = conn.last_insert_rowid();
    get_attendee(conn, id)
}

pub fn get_attendee(conn: &Connection, id: i64) -> Result<ToolboxTalkAttendee> {
    conn.query_row(
        "SELECT id, talk_id, employee_name, employee_id, signature_data, signed_at
         FROM toolbox_talk_attendees WHERE id = ?",
        [id],
        |row| {
            Ok(ToolboxTalkAttendee {
                id: row.get(0)?,
                talk_id: row.get(1)?,
                employee_name: row.get(2)?,
                employee_id: row.get(3)?,
                signature_data: row.get(4)?,
                signed_at: row.get(5)?,
            })
        },
    )
}

pub fn list_attendees(conn: &Connection, talk_id: i64) -> Result<Vec<ToolboxTalkAttendee>> {
    let mut stmt = conn.prepare(
        "SELECT id, talk_id, employee_name, employee_id, signature_data, signed_at
         FROM toolbox_talk_attendees WHERE talk_id = ? ORDER BY employee_name",
    )?;

    let rows = stmt.query_map([talk_id], |row| {
        Ok(ToolboxTalkAttendee {
            id: row.get(0)?,
            talk_id: row.get(1)?,
            employee_name: row.get(2)?,
            employee_id: row.get(3)?,
            signature_data: row.get(4)?,
            signed_at: row.get(5)?,
        })
    })?;

    rows.collect()
}

pub fn sign_attendee(conn: &Connection, data: SignAttendee) -> Result<ToolboxTalkAttendee> {
    conn.execute(
        "UPDATE toolbox_talk_attendees
         SET signature_data = ?, signed_at = datetime('now')
         WHERE id = ?",
        params![data.signature_data, data.attendee_id],
    )?;

    get_attendee(conn, data.attendee_id)
}

pub fn delete_attendee(conn: &Connection, id: i64) -> Result<()> {
    let deleted = conn.execute("DELETE FROM toolbox_talk_attendees WHERE id = ?", [id])?;
    if deleted == 0 {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::locations::{create_establishment, CreateEstablishment};
    use crate::db::open_test_db;

    fn create_test_establishment(conn: &Connection) -> i64 {
        create_establishment(
            conn,
            CreateEstablishment {
                name: "Acme Construction".to_string(),
                street_address: None,
                city: None,
                state: None,
                zip_code: None,
                industry_description: Some("Construction".to_string()),
                naics_code: Some("236220".to_string()),
            },
        )
        .expect("failed to create test establishment")
        .id
    }

    #[test]
    fn test_toolbox_talk_attendee_signature_flow() {
        let conn = open_test_db();
        let establishment_id = create_test_establishment(&conn);

        let talk = create_talk(
            &conn,
            CreateToolboxTalk {
                topic_id: None,
                establishment_id,
                location_id: None,
                title: "Ladder Safety Briefing".to_string(),
                date: "2026-03-01".to_string(),
                conducted_by: "Site Supervisor".to_string(),
                notes: Some("Weekly toolbox safety session".to_string()),
            },
        )
        .expect("failed to create toolbox talk");
        assert_eq!(talk.status, "scheduled");

        let attendee = add_attendee(
            &conn,
            AddAttendee {
                talk_id: talk.id,
                employee_name: "Jamie Carter".to_string(),
                employee_id: None,
            },
        )
        .expect("failed to add attendee");
        assert!(attendee.signature_data.is_none());

        let signed = sign_attendee(
            &conn,
            SignAttendee {
                attendee_id: attendee.id,
                signature_data: "data:image/png;base64,signature".to_string(),
            },
        )
        .expect("failed to sign attendee");
        assert!(signed.signature_data.is_some());
        assert!(signed.signed_at.is_some());

        let attendees = list_attendees(&conn, talk.id).expect("failed to list attendees");
        assert_eq!(attendees.len(), 1);
        assert!(attendees[0].signature_data.is_some());

        let completed = complete_talk(&conn, talk.id).expect("failed to complete talk");
        assert_eq!(completed.status, "completed");

        delete_attendee(&conn, attendee.id).expect("failed to delete attendee");
        let attendees_after_delete =
            list_attendees(&conn, talk.id).expect("failed to list attendees after delete");
        assert!(attendees_after_delete.is_empty());
    }

    #[test]
    fn test_toolbox_topics_seeded_and_active_filtering() {
        let conn = open_test_db();
        let active_topics = list_topics(&conn, false).expect("failed to list active topics");
        assert!(!active_topics.is_empty());
        assert!(active_topics.iter().all(|topic| topic.is_active));
    }

    #[test]
    fn test_delete_attendee_returns_not_found_for_missing_id() {
        let conn = open_test_db();
        let err = delete_attendee(&conn, 999_999).expect_err("expected missing attendee error");
        assert!(matches!(err, rusqlite::Error::QueryReturnedNoRows));
    }
}
