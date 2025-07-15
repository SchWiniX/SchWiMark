use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct StartArgs{
	#[command(subcommand)]
	pub operation: Option<Operation>,
	#[arg(short, long)]
	pub config: Option<PathBuf>,
	#[arg(short, long)]
	pub dmenu_command: Option<String>
}

#[derive(Subcommand)]
pub enum Operation {
	Delete { id: u32 },
	Update { id: u32 },
	Add,
	Clear,
	Open,
}

pub fn enter_cli() {
	let start_args: StartArgs = StartArgs::parse();

	if let Some(config_path) = start_args.config.as_deref() {
		println!("has a config path at: {}", config_path.display());
	}

	if let Some(dmenu_command) = start_args.dmenu_command.as_deref() {
		println!("dmenu is : {}", dmenu_command) 
	}

	match start_args.operation {
		Some(Operation::Delete { id }) => {
			println!("Delete with {id}");
			//TODO: enter delete cli
		}
		Some(Operation::Update{ id }) => {
			println!("Update with {id}");
			//TODO: enter update cli
		}
		Some(Operation::Add) => {
			println!("add");
			//TODO: enter add cli
		}
		Some(Operation::Clear) => {
			println!("clear");
			//TODO: ask for confirmation then clear the data base
		}
		Some(Operation::Open) => {
			println!("Open");
			//TODO: open DMENU and then execute the selection
		}
		None => {
			println!("Subcommand CLI");
			//TODO enter submenu cli
		}
	}
}
