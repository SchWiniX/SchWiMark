use std::io::Read;
use std::path::PathBuf;
use std::{default, env}; //needed later for std::env::consts::OS to have special behavior on WINDOWS / MACOS
use std::fs::{OpenOptions, File};
use xdg::BaseDirectories;
use serde::{Serialize, Deserialize};

const CONFIG_FILE_NAME: &str = "config.toml";
const DATABASE_FILE_NAME: &str = "schwimark.db";
const DEFAULT_DMENU_COMMAND: &str = "dmenu_run";

#[derive(Serialize, Deserialize)]
#[serde[default]]
pub struct Config {
	#[serde(skip)]
	config_file: PathBuf,

	pub database_file: PathBuf,
	pub dmenu_command: String,
}

impl Config {
	pub fn new(config_file: PathBuf) -> Config { 
		Config {
			config_file: config_file,
			database_file: get_default_database_file(),
			dmenu_command: DEFAULT_DMENU_COMMAND.to_string()
		}
	}

	pub fn load_config(&mut self, database_file: Option<PathBuf>, dmenu_command: Option<String>) {
		assert!(self.config_file.to_str().unwrap() != "");
		let mut config_file: File = File::options()
			.read(true)
			.create(true)
			.append(true)
			.open(&self.config_file)
			.expect("failed to open the config file");

		let mut config_contents = String::new();
		config_file.read_to_string(&mut config_contents)
			.expect("failed to write config file to string");

		let read_config: Config = toml::from_str(&config_contents)
			.expect("failed to parse config file");
		self.database_file = database_file.unwrap_or(read_config.database_file);
		self.dmenu_command = dmenu_command.unwrap_or(read_config.dmenu_command);

		assert!(self.database_file.to_str().unwrap() != "");
	}
}

impl Default for Config {
	fn default() -> Config {
		Config {
			config_file: get_default_config_file(),
			database_file: get_default_database_file(),
			dmenu_command: DEFAULT_DMENU_COMMAND.to_string()
		}
	}
}


fn get_default_config_file() -> PathBuf {
	let xdg_dirs = xdg::BaseDirectories::with_prefix("schwimark");
	let config_path: PathBuf = xdg_dirs
		.find_config_file(CONFIG_FILE_NAME)
		.unwrap_or(
			xdg_dirs
				.place_config_file(CONFIG_FILE_NAME)
				.expect("failed to aquire a default config path")
		);

	return config_path
}

fn get_default_database_file() -> PathBuf {
	let xdg_dirs = xdg::BaseDirectories::with_prefix("schwimark");
	let database_path: PathBuf = xdg_dirs
		.find_data_file(DATABASE_FILE_NAME)
		.unwrap_or(
			xdg_dirs
				.place_data_file(DATABASE_FILE_NAME)
				.expect("failed to aquire a default database path")
		);

	return database_path
}
