use open::{that, with_command};
use std::collections::HashMap;
use std::process::{exit, Stdio};
use std::{fmt, path::PathBuf};
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

impl fmt::Display for SchWiMark {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f, 
			"name: {}
			description: {}
			url/path: {}
			default application: {}",
			self.name,
			self.description,
			self.url,
			self.application
			)
	}
}

pub struct Tag {
	markid: i64,
	tags: Vec<String>,
}

impl Tag {
	fn new(markid: i64, tags: Vec<String>) -> Tag { 
		Tag {
			markid: markid,
			tags: tags,
		}

	}
}

impl fmt::Display for Tag{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f, 
			"tags: {:?}",
			self.tags
			)
	}
}

pub fn create_database(database_path: &PathBuf) -> Result<Connection> {
	assert!(database_path.to_str().unwrap() != "");
	let sqlite_connection: Connection = Connection::open(database_path)?;

	sqlite_connection.execute_batch(
		"CREATE TABLE IF NOT EXISTS schwimark (
			markid INTEGER PRIMARY KEY,
			name TEXT NOT NULL UNIQUE,
			description TEXT NOT NULL,
			url TEXT NOT NULL,
			application TEXT NOT NULL
		);
		CREATE TABLE IF NOT EXISTS tags (
			markid INTEGER,
			tag TEXT NOT NULL,
			FOREIGN KEY (markid) REFERENCES schwimark(markid) ON DELETE CASCADE
		);
		PRAGMA foreign_keys = ON;
		",
	).unwrap();

	Ok(sqlite_connection)
}

pub fn add_mark(
	database: &Connection,
	name: String,
	description: String,
	url: String,
	application: String,
	tags: Vec<String>
	) -> Result<(SchWiMark, Tag)> {

	database.execute(
		"INSERT INTO schwimark (name, description, url, application) VALUES (?1, ?2, ?3, ?4)",
		params![name, description, url, application],
	)?;

	let last_rowid: i64 = database.last_insert_rowid();

	let mut prepare_tags = database.prepare("INSERT INTO tags (markid, tag) VALUES (?1, ?2)")?;
	for tag in tags.as_slice() {
		prepare_tags.execute(params![last_rowid, tag])?;
	}
	
	let new_mark: SchWiMark = SchWiMark::new(last_rowid, name, description, url, application);
	let new_tags: Tag = Tag::new(last_rowid, tags);

	Ok((new_mark, new_tags))
}

pub fn delete_mark(database: &Connection, id: i64) -> Result<()> {
	database.execute(
		"DELETE FROM schwimark WHERE schwimark.markid = ?1",
		params![id],
	)?;

	Ok(())
}

pub fn update_name(database: &Connection, id: i64, name: String) -> Result<()> {
	database.execute(
		"UPDATE schwimark SET name = ?1 WHERE markid = ?2",
		params![name, id],
	)?;

	Ok(())
}

pub fn update_description(database: &Connection, id: i64, description: String) -> Result<()> {
	database.execute(
		"UPDATE schwimark SET description = ?1 WHERE markid = ?2",
		params![description, id],
	)?;

	Ok(())
}

pub fn update_url(database: &Connection, id: i64, url: String) -> Result<()> {
	database.execute(
		"UPDATE schwimark SET url = ?1 WHERE markid = ?2",
		params![url, id],
	)?;

	Ok(())
}

pub fn update_application(database: &Connection, id: i64, application: String) -> Result<()> {
	database.execute(
		"UPDATE schwimark SET application = ?1 WHERE markid = ?2",
		params![application, id],
	)?;

	Ok(())
}

pub fn add_tags(database: &Connection, id: i64, tags: Vec<String>) -> Result<()> {
	let mut prepare_tags = database.prepare("INSERT INTO tags (markid, tag) VALUES (?1, ?2)")?;
	for tag in tags {
		prepare_tags.execute(params![id, tag])?;
	}

	Ok(())
}

pub fn delete_tag(database: &Connection, id: i64, tag: String) -> Result<()> {
	database.execute(
		"DELETE FROM tags WHERE tags.markid=?1 AND tags.tag=?2",
		params![id, tag],
	)?;

	Ok(())
}

pub fn clear_database(database: &Connection) -> Result<()> {
	database.execute("DELETE FROM schwimark", [])?;
	database.execute("DELETE FROM tags", [])?;
	Ok(())
}

pub fn get_marks_short(database: &Connection) -> Result<Vec<String>> {
	let mut schwimark_query = database.prepare("
		SELECT schwimark.markid, schwimark.name
		FROM schwimark"
	)?;

	let mut mark_hashmap: HashMap<i64, String> = HashMap::new();
	let query_errors = schwimark_query.query_map([], |row| {
		mark_hashmap.insert(
			row.get::<usize, i64>(0)?,
			row.get::<usize, i64>(0)?.to_string()
				+ "\t" + &row.get::<usize, String>(1)?
		);
		Ok(())
	})?;

	for err in query_errors {
		err.unwrap();
	} // this is straigth up bullshittery

	let mut tags_query = database.prepare("
		SELECT tags.markid, STRING_AGG(tags.tag, \"\t\")
		FROM tags
		GROUP BY tags.markid")?;

	let query_tags_error = tags_query.query_map([], |row| {
		let row_str: String = match mark_hashmap.get(&row.get::<usize, i64>(0)?) {
			Some(ent) => { ent.to_string() }
			None => { return Ok(()); } //this should prob be error handeled cause this would mean a tags markid entry exists for which not schwimark in the schwimark tabel
		};
		mark_hashmap.insert(
			row.get::<usize, i64>(0)?,
			row_str + "\t" + &row.get::<usize, String>(1)?
		);

		Ok(())
	})?;

	for err in query_tags_error {
		err.unwrap();
	}

	Ok(mark_hashmap.into_values().collect())
}

pub fn get_tags(database: &Connection, id: i64) -> Result<Vec<String>> {
	let mut query = database.prepare("SELECT DISTINCT(tag) FROM tags WHERE tags.markid = ?1")?;
	let tag_iter = query.query_map(params![id], |row| {
		row.get::<usize, String>(0)
	})?;
	
	Ok(tag_iter.map(|e| e.unwrap()).collect())
}

pub fn show_all_marks(database: &Connection) -> Result<()> {
	let mut marks: HashMap<i64, (String, String, String, String, String)> = Default::default();

	let mut schwimark_query = database.prepare("
		SELECT schwimark.markid, schwimark.name, schwimark.description, schwimark.url, schwimark.application
		FROM schwimark"
	)?;

	let query_errors = schwimark_query.query_map([], |row| {
		let id: i64 = row.get::<usize, i64>(0)?;
		marks.insert(
			id,
			(
				row.get::<usize, String>(1)?,
				row.get::<usize, String>(2)?,
				row.get::<usize, String>(3)?,
				row.get::<usize, String>(4)?,
				"".to_string(),
			)
		);
		Ok(())
	})?;

	for err in query_errors {
		err.unwrap();
	} // this is straigth up bullshittery

	let mut tags_query = database.prepare("
		SELECT tags.markid, STRING_AGG(tags.tag, \", \")
		FROM tags
		GROUP BY tags.markid")?;

	let query_tags_error = tags_query.query_map([], |row| {
		let id: i64 = row.get::<usize, i64>(0)?;
		let curr_entry = match marks.get(&id) {
			Some(m) => { m.clone() }
			None => { return Ok(()) }
		};
		marks.insert(
			id,
			(
				curr_entry.0,
				curr_entry.1,
				curr_entry.2,
				curr_entry.3,
				row.get::<usize, String>(1)?,
			)
		);
		Ok(())
	})?;

	for err in query_tags_error {
		err.unwrap();
	}

	println!(
		"{0: <3} | {1: <20} | {2: <80} | {3: <50} | {4: <20} | {5: <0}",
		"id", "name", "description", "url", "application", "tags"
	);
	for (id, m) in marks.iter() {
		println!(
			"{0: <3} | {1: <20} | {2: <80} | {3: <50} | {4: <20} | {5: <0}",
			id, m.0, m.1, m.2, m.3, m.4,
		);
	}
	Ok(())
}

pub fn show_mark(database: &Connection, id: i64) -> Result<()> {
	let mut name: String = Default::default();
	let mut desc: String = Default::default();
	let mut url: String = Default::default();
	let mut app: String = Default::default();
	let mut tags: String = Default::default();

	database.query_row("
		SELECT schwimark.markid, schwimark.name, schwimark.description, schwimark.url, schwimark.application
		FROM schwimark
		WHERE schwimark.markid == ?1
		GROUP BY schwimark.markid",
		[id],
		|row| {
			name = row.get::<usize, String>(1)?;
			desc = row.get::<usize, String>(2)?;
			url = row.get::<usize, String>(3)?;
			app = row.get::<usize, String>(4)?;
			Ok(())
		}
	)?;

	let tag_query_result = database.query_row("
		SELECT STRING_AGG(tags.tag, \"\t\")
		FROM tags
		WHERE tags.markid == ?1
		GROUP BY tags.markid",
		[id],
		|row| {
			tags = row.get::<usize, String>(0)?;
			Ok(())
		}
	);

	match tag_query_result {
		Ok(_) => {}
		Err(_) => { tags = "".to_string(); }
	}

	println!(
		"{0: <3} | {1: <20} | {2: <80} | {3: <50} | {4: <20} | {5: <0}",
		"id", "name", "description", "url", "application", "tags"
	);
	println!(
		"{0: <3} | {1: <20} | {2: <80} | {3: <50} | {4: <20} | {5: <0}",
		id, name, desc, url, app, tags,
	);
	Ok(())
}

pub fn open_mark(database: &Connection, id: i64) -> Result<()> {
	let mut url: String = Default::default();
	let mut application: String = Default::default();
	database.query_row("
		SELECT schwimark.url, schwimark.application
		FROM schwimark
		WHERE schwimark.markid == ?1
		GROUP BY schwimark.markid",
		[id],
		|row| {
			url = row.get::<usize, String>(0)?;
			application = row.get::<usize, String>(1)?;
			Ok(())
		},
	)?;

	if application.is_empty() {
		match that(&url) {
			Ok(_) => { }
			Err(e) => {
				println!("failed to open SchWiMark with error for default application \"{}\"", e);
				exit(1)
			}
		}
	} else {
		match with_command(&url, application).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn() {
			Ok(_) => {}
			Err(e1) => { 
				println!("failed to open SchWiMark with error for default application \"{}\"\nattempting to open via default application", e1);
				match that(&url) {
					Ok(_) => { }
					Err(e2) => {
						println!("failed to open SchWiMark with error for default application \"{}\"", e2);
						exit(1)
					}
				}
			}
		}
	}
	Ok(())
}
