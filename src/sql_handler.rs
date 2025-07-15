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

pub fn create_database() -> Result<Connection> {
	let sqlite_connection: Connection = Connection::open("temp_database.db")?;

	sqlite_connection.execute_batch(
		"CREATE TABLE IF NOT EXISTS schwimark (
			markid INTEGER PRIMAER KEY AUTOINCEMENT,
			name TEXT NOT NULL UNIQUE,
			description TEXT NOT NULL,
			url TEXT NOT NULL,
			application TEXT,
		);
		CREATE TABLE IF NOT EXISTS tag (
			FOREIGN KEY (markid) REFERENCES mark(markid) ON DELETE CASCADE,
			tag TEXT NOT NULL
		);"
	)?;

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
			"INSERT INTO tag () VALUES (?1, ?2)",
			params![last_rowid, tag],
		)?;
	}
	
	let new_mark: SchWiMark = SchWiMark::new(last_rowid, name, description, url, application);
	let new_tags: Tag = Tag::new(last_rowid, tags);

	Ok((new_mark, new_tags))
}

pub fn delete_mark(database: &Connection, id: i64) -> Result<()> {

	database.execute(
		"DELETE FROM schwimark WHERE id=?1",
		params![id],
	)?;

	Ok(())
}

pub fn update_name(database: &Connection, id: i64, name: String) -> Result<()> {
}
