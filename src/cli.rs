use crate::config;
use crate::sql;
use crate::dmenu_handler;

use std::path::PathBuf;
use clap::{Parser, Subcommand};
use rusqlite::{Connection};

#[derive(Parser)]
struct StartArgs{
	#[command(subcommand)]
	operation: Option<Operation>,

	#[arg(short, long="config")]
	config_path: Option<PathBuf>,

	#[arg(short, long="database")]
	database_path: Option<PathBuf>,

	#[arg(short='m', long)]
	dmenu_command: Option<String>,
}

#[derive(Subcommand)]
enum Operation {
	Delete,
	Update,
	Add,
	Clear,
	Open,
	Show,
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
		);

	let database = sql::create_database(&config.database_file).expect("failed to create/open the database");

	dmenu_handler::open_search(&config, &mut vec!["hi".to_string(), "ho".to_string(), "he".to_string()]);

	match start_args.operation {
		Some(Operation::Delete) => {
		}
		Some(Operation::Update) => {
			update_cli(&database);
		}
		Some(Operation::Add) => {
			add_cli(&database);
		}
		Some(Operation::Clear) => {
			clear_cli(&database);
		}
		Some(Operation::Open) => {
			println!("Open");
			//TODO: open DMENU and then execute the selection
		}
		Some(Operation::Show) => {
			println!("Show");
			//TODO: open DMENU and then print the selection
		}
		None => {
			println!("Subcommand CLI");
			//TODO enter submenu cli
		}
	}
}

fn database_entry_cli() -> MarkArgs {
	let mut input_vec: Vec<String> = vec![];

	input_vec.reserve(5);
	input_vec.push(name_cli());
	input_vec.push(description_cli());
	input_vec.push(url_cli());
	input_vec.push(application_cli());
	input_vec.append(&mut tags_cli());

	//todo call the parser on the interator of input_vec
	MarkArgs::try_parse_from(input_vec.iter()).unwrap()
}

fn name_cli() -> String {
	let mut name_buf: String = String::new();

	loop {
		name_buf.clear();
		println!("Enter the name of the new SchWiMark");
		eprint!("name> ");
		std::io::stdin().read_line(&mut name_buf).expect("Could not parse name input");
		name_buf.trim().to_string();
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
	description_buf.trim().to_string()
}

fn url_cli() -> String {
	let mut url_buf: String = String::new();

	loop {
		url_buf.clear();
		println!("Enter the url or path of the new SchWiMark");
		eprint!("url/path> ");
		std::io::stdin().read_line(&mut url_buf).expect("Could not parse description input");
		url_buf.trim().to_string();
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
	application_buf.trim().to_string()
}

fn tags_cli() -> Vec<String> {
	let mut tag_vec: Vec<String> = vec![];

	loop {
		let mut tag_buf: String = String::new();
		println!("Enter a tag of the new SchWiMark enter nothing to continue");
		eprint!("tag> ");
		std::io::stdin().read_line(&mut tag_buf).expect("Could not parse the tag input");
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

fn update_cli(database: &Connection) {

	//get id through dmenu (TODO)
	let update_id: i64 = 0;

	//print entry
	
	let mut menu_buf: String = String::with_capacity(5);
	println!("What do you wish to change? (please enter the corresponding letters)
		name: n)
		description: d)
		url/path: u)
		default application: a)
		add a tag +)
		remove a tag -)
		Hint: if you want to update multiple field you can type both e.g. \"nu\" will enter both the name update menu and the url/path update menu"
		);
	eprint!("field>");

	std::io::stdin().read_line(&mut menu_buf).expect("Could not parse clear confirmation");

	for c in menu_buf.chars() {
		match c {
			'n' => { sql::update_name(database, update_id, name_cli()).unwrap(); }
			'd' => { sql::update_description(database, update_id, description_cli()).unwrap(); }
			'u' => { sql::update_url(database, update_id, url_cli()).unwrap(); }
			'a' => { sql::update_application(database, update_id, application_cli()).unwrap(); }
			'+' => { sql::add_tags(database, update_id, tags_cli()).unwrap(); }
			'-' => { /*TODO: call dmenu with all tags */ }
			_ => { continue; }
		}
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
			println!("cleared");
			sql::clear_database(database).expect("failed to clear database");
			break;
		} else if conf_char == 'n' || conf_char == 'N' {
			println!("break");
			break;
		} else {
			println!("continue");
			continue;
		}
	}
}
