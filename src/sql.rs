use std::path::PathBuf;
use rusqlite::{params, Connection, Result};

pub struct SchWiMark {
	id: i64,
	name: String,
	description: String,
	url: String,
	application: String,
}

impl SchWiMark {
	fn new(id: i64, name: String, description: String, url: String, application: String) -> SchWiMark {
		SchWiMark {
			id: id,
			name: name,
			description: description,
			url: url,
			application: application,
		}
	}
}

pub struct Tag {
	mark_id: i64,
	tags: Vec<String>,
}

impl Tag {
	fn new(mark_id: i64, tags: Vec<String>) -> Tag { 
		Tag {
			mark_id: mark_id,
			tags: tags,
		}

	}
}

pub fn create_database(database_path: PathBuf) -> Result<Connection> {
	assert!(database_path.to_str().unwrap() != "");
	let sqlite_connection: Connection = Connection::open(database_path)?;

	sqlite_connection.execute_batch(
		"CREATE TABLE IF NOT EXISTS schwimark (
			markid INTEGER PRIMARY KEY,
			name TEXT NOT NULL UNIQUE,
			description TEXT NOT NULL,
			url TEXT NOT NULL,
			application TEXT
		);
		CREATE TABLE IF NOT EXISTS tags (
			markid INTEGER,
			tag TEXT NOT NULL,
			FOREIGN KEY (markid) REFERENCES mark(markid) ON DELETE CASCADE
		);",
	).unwrap();

	Ok(sqlite_connection)
}

pub fn add_mark(database: &Connection, name: String, description: String, url: String, application: String, tags: Vec<String>) -> Result<(SchWiMark, Tag)> {

	database.execute(
		"INSERT INTO schwimark (name, description, url, application) VALUES (?1, ?2, ?3, ?4)",
		params![name, description, url, application],
	)?;

	let last_rowid: i64 = database.last_insert_rowid();

	for tag in tags.as_slice() {
		database.execute(
			"INSERT INTO tags (markid, tag) VALUES (?1, ?2)",
			params![last_rowid, tag],
		)?;
	}
	
	let new_mark: SchWiMark = SchWiMark::new(last_rowid, name, description, url, application);
	let new_tags: Tag = Tag::new(last_rowid, tags);

	Ok((new_mark, new_tags))
}

pub fn delete_mark(database: &Connection, id: i64) -> Result<()> {
	database.execute(
		"DELETE FROM schwimark s WHERE s.id=?1",
		params![id],
	)?;

	Ok(())
}

pub fn update_name(database: &Connection, id: i64, name: String) -> Result<()> {
	database.execute(
		"UPDATE schwimark s SET s.name=?1 WHERE s.id=?2",
		params![name, id],
	)?;

	Ok(())
}

pub fn update_description(database: &Connection, id: i64, description: String) -> Result<()> {
	database.execute(
		"UPDATE schwimark s SET s.description=?1 WHERE s.id=?2",
		params![description, id],
	)?;

	Ok(())
}

pub fn update_url(database: &Connection, id: i64, url: String) -> Result<()> {
	database.execute(
		"UPDATE schwimark s SET s.url=?1 WHERE s.id=?2",
		params![url, id],
	)?;

	Ok(())
}

pub fn update_application(database: &Connection, id: i64, application: String) -> Result<()> {
	database.execute(
		"UPDATE schwimark s SET s.application=?1 WHERE s.id=?2",
		params![application, id],
	)?;

	Ok(())
}

pub fn add_tag(database: &Connection, id: i64, tag: String) -> Result<()> {
	database.execute(
		"INSERT INTO tags (markid, tag) VALUES (?1, ?2)",
		params![id, tag],
	)?;

	Ok(())
}

pub fn delete_tag(database: &Connection, id: i64, tag: String) -> Result<()> {
	database.execute(
		"DELETE FROM tags t WHERE t.id=?1 AND t.tag=?2",
		params![id, tag],
	)?;

	Ok(())
}

pub fn clear_database(database: &Connection) -> Result<()> {
	database.execute("DELETE FROM schwimark", [])?;
	database.execute("DELETE FROM tags", [])?;
	Ok(())
}
