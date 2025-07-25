use crate::config;
use crate::sql;
use crate::dmenu_handler;

use std::path::PathBuf;
use clap::{Parser, Subcommand};
use rusqlite::{Connection};

#[derive(Parser)]
struct StartArgs{
	#[command(subcommand)]
	operation: Operation,

	/// specify a config path to be used
	#[arg(short, long="config")]
	config_path: Option<PathBuf>,

	/// specify a path to a SchWiMark database to be used
	#[arg(short, long="database")]
	database_path: Option<PathBuf>,

	/// the command to which the list of bookmarks is piped
	#[arg(short='m', long)]
	dmenu_command: Option<String>,

	/// additional arguments used when searching through marks
	#[arg(short='s', long)]
	dmenu_mark_arguments: Option<String>,

	/// additional arguments used when searching through tags
	#[arg(short='t', long)]
	dmenu_tag_arguments: Option<String>,
}

#[derive(Subcommand)]
enum Operation {
	/// Opens the selection and will delete the entry that was selected
	Delete,
	/// Opens the selection and will continue to the update cli for the entry that was selected
	Update,
	/// Opens the add SchWiMark cli
	Add,
	/// Clears the database (WARNING: all data will be lost)
	Clear,
	/// Opens the selection and will attempt to open the url/path specified
	Open,
	/// Opens the selection and will print out the selection made
	Show,
	/// will print out all SchWImarks to the console
	ShowAll,
}

#[derive(Parser)]
struct MarkArgs {
	name: String,
	description: String,
	url: String,
	application: String,
	tags: Vec<String>,
}


pub fn start_cli() {
	let start_args: StartArgs = StartArgs::parse();
	let mut config: config::Config;

	match start_args.config_path {
		Some(config_path) => config = config::Config::new(config_path.to_path_buf()),
		None => config = config::Config::default(),
	}

	config.load_config(
		start_args.database_path,
		start_args.dmenu_command,
		start_args.dmenu_mark_arguments,
		start_args.dmenu_tag_arguments,
		);

	let database = sql::create_database(&config.database_file).expect("failed to create/open the database");

	match start_args.operation {
		Operation::Delete => {
			let id: i64 = start_mark_selection(&database, &config);
			if id < 0 { return; }
			sql::delete_mark(&database, id).expect("failed to delete this mark");
		}
		Operation::Update => {
			update_cli(&database, &config);
		}
		Operation::Add => {
			add_cli(&database);
		}
		Operation::Clear => {
			clear_cli(&database);
		}
		Operation::Open => {
			let id: i64 = start_mark_selection(&database, &config);
			if id < 0 { return; }
			sql::open_mark(&database, id).expect("failed to open mark")
		}
		Operation::Show => {
			let id: i64 = start_mark_selection(&database, &config);
			if id < 0 { return; }
			sql::show_mark(&database, id).expect("failed to print mark")
		}
		Operation::ShowAll => {
			sql::show_all_marks(&database).unwrap();
		}
	}
}

fn start_mark_selection(database: &Connection, config: &config::Config) -> i64 {
	let mut entries: Vec<String> = sql::get_marks_short(&database).expect("failed to query marks");
	let selected_item: String = match dmenu_handler::open_mark_search(&config, &mut entries) {
		Ok(s) => { s }
		Err(e) => { println!("{}", e); return -1; }
	};
	if selected_item.is_empty() { return -1; }
	selected_item.split_whitespace().nth(0).unwrap().parse::<i64>().unwrap()
}

fn database_entry_cli() -> MarkArgs {
	let mut input_vec: Vec<String> = vec![];

	input_vec.reserve(6);
	input_vec.push("".to_string());
	input_vec.push(name_cli());
	input_vec.push(description_cli());
	input_vec.push(url_cli());
	input_vec.push(application_cli());
	input_vec.append(&mut tags_cli());

	MarkArgs::try_parse_from(input_vec.iter()).unwrap()
}

fn name_cli() -> String {
	let mut name_buf: String = String::new();

	loop {
		name_buf.clear();
		println!("Enter the name of the new SchWiMark");
		eprint!("name> ");
		std::io::stdin().read_line(&mut name_buf).expect("Could not parse name input");
		if name_buf.ends_with('\n') { name_buf.pop(); };
		if name_buf.is_empty() {
			println!("name cannot be empty");
			continue;
		} else { return name_buf }
	}
}

fn description_cli() -> String {
	let mut description_buf: String = String::new();

	println!("Enter the description of the new SchWiMark");
	eprint!("description> ");
	std::io::stdin().read_line(&mut description_buf).expect("Could not parse description input");
	description_buf.trim().to_string();
	if description_buf.ends_with('\n') { description_buf.pop(); };
	description_buf
}

fn url_cli() -> String {
	let mut url_buf: String = String::new();

	loop {
		url_buf.clear();
		println!("Enter the url or path of the new SchWiMark");
		eprint!("url/path> ");
		std::io::stdin().read_line(&mut url_buf).expect("Could not parse description input");
		url_buf.trim().to_string();
		if url_buf.ends_with('\n') { url_buf.pop(); };
		if url_buf.is_empty() {
			println!("url cannot be empty");
			continue;
		}
		else { return url_buf }
	}
}

fn application_cli() -> String {
	let mut application_buf: String = String::new();

	println!("Enter the default application you want the SchwiMark to be opened with (leave empty to use default application):");
	eprint!("application> ");
	std::io::stdin().read_line(&mut application_buf).expect("Could not parse description input");
	if application_buf.ends_with('\n') { application_buf.pop(); };
	application_buf
}

fn tags_cli() -> Vec<String> {
	let mut tag_vec: Vec<String> = vec![];

	loop {
		let mut tag_buf: String = String::new();
		println!("Enter a tag of the new SchWiMark enter nothing to continue");
		eprint!("tag> ");
		std::io::stdin().read_line(&mut tag_buf).expect("Could not parse the tag input");
		if tag_buf.ends_with('\n') { tag_buf.pop(); };
		tag_buf.trim().to_string();

		if tag_buf.is_empty() {
			break;
		}
		else if tag_vec.contains(&tag_buf) { 
			println!("tag is already selected for this SchWiMark");
			continue;
		}
		else { tag_vec.push(tag_buf); }
	}
	return tag_vec
}

fn update_cli(database: &Connection, config: &config::Config) {

	let mut entries: Vec<String> = sql::get_marks_short(&database).expect("failed to query marks");
	let selected_item: String = match dmenu_handler::open_mark_search(&config, &mut entries) {
		Ok(s) => { s }
		Err(e) => { println!("{}", e); return; }
	};
	if selected_item.is_empty() { return; }
	let update_id: i64 = selected_item.split_once("\t").unwrap().0.parse::<i64>().unwrap();

	sql::show_mark(&database, update_id).expect("failed to print mark");
	
	let mut menu_buf: String = String::with_capacity(5);
	println!("What do you wish to change? (please enter the corresponding letters)\n\
		name: n)\n\
		description: d)\n\
		url/path: u)\n\
		default application: a)\n\
		add a tag +)\n\
		remove a tag -)\n\
		Hint: if you want to update multiple field you can type both e.g. \"nu\" will enter both the name update menu and the url/path update menu"
		);
	eprint!("field> ");

	std::io::stdin().read_line(&mut menu_buf).expect("Could not parse clear confirmation");

	for c in menu_buf.chars() {
		match c {
			'n' => { 
				match sql::update_name(database, update_id, name_cli()) {
					Ok(_) => {}
					Err(e) => { panic!("sql failed with error \"{}\"", e) }
				}
			}
			'd' => {
				match sql::update_description(database, update_id, description_cli()) {
					Ok(_) => {}
					Err(e) => { panic!("sql failed with error: \"{}\"", e); }
				}
			}
			'u' => {
				match sql::update_url(database, update_id, url_cli()) {
					Ok(_) => {}
					Err(e) => { panic!("sql failed with error: \"{}\"", e); }
				}
			}
			'a' => {
				match sql::update_application(database, update_id, application_cli()) {
					Ok(_) => {}
					Err(e) => { panic!("sql failed with error: \"{}\"", e); }
				}
			}
			'+' => {
				match sql::add_tags(database, update_id, tags_cli()) {
					Ok(_) => {}
					Err(e) => { panic!("sql failed with error: \"{}\"", e); }
				}
			}
			'-' => {
				loop {
					let mut tag_entries: Vec<String> = sql::get_tags(&database, update_id).expect("failed to query tags");
					let mut selected_tag: String = match dmenu_handler::open_tag_search(&config, &mut tag_entries) {
						Ok(r) => { r }
						Err(_) => { break; }
					};
					if selected_tag.ends_with('\n') { selected_tag.pop(); };
					if selected_tag.is_empty() { break; }
					sql::delete_tag(&database, update_id, selected_tag).expect("failed to delete the tag from the database");
				} 
			}
			_ => { continue; }
		};
	}

}

fn add_cli(database: &Connection) {
	let mark_entry = database_entry_cli();
	let (_schwimark, _tags): (sql::SchWiMark, sql::Tag) = sql::add_mark(database, mark_entry.name, mark_entry.description, mark_entry.url, mark_entry.application, mark_entry.tags).unwrap();
}

fn clear_cli(database: &Connection) {
	let mut input_buf: String = String::with_capacity(2);

	loop {
		input_buf.clear();
		eprint!("Are you sure you want to clear all your bookmarks? [y/N]: ");
		std::io::stdin().read_line(&mut input_buf).expect("Could not parse clear confirmation");
		let line = input_buf.trim();
		let args = shlex::split(line).ok_or("error: Invalid quoting").unwrap();

		if args.len() > 1 { continue; }
		if args[0].chars().nth(1) != None { continue; }
		let conf_char: char = match args[0].chars().nth(0) {
			Some(c) => { c }
			None => { continue; }
		};

		if conf_char == 'y' || conf_char == 'Y' {
			sql::clear_database(database).expect("failed to clear database");
			break;
		} else if conf_char == 'n' || conf_char == 'N' {
			break;
		} else {
			continue;
		}
	}
}
