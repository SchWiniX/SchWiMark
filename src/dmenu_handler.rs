use std::process::{exit, Command, Stdio};
use crate::config;

pub fn open_mark_search(config: &config::Config, entries: &mut Vec<String>) -> Result<String, String> {
	let mark_search_args: Vec<String> = shlex::split(&config.dmenu_mark_search_args).ok_or("error: Invalid quoting").unwrap();
	open_search(config, entries, mark_search_args)
}

pub fn open_tag_search(config: &config::Config, entries: &mut Vec<String>) -> Result<String, String> {
	let tag_search_args: Vec<String> = match shlex::split(&config.dmenu_tag_search_args).ok_or("error: Invalid quoting") {
		Ok(r) => { r }
		Err(e) => { return Err(e.to_string()) }
	};
	open_search(config, entries, tag_search_args)
}

fn open_search(config: &config::Config, entries: &mut Vec<String>, additional_dmenu_args: Vec<String>) -> Result<String, String> {
	if entries.is_empty() { return Err("no entries to display".to_string()); }

	let entries_string: String = entries
		.iter_mut()
		.reduce(|acc: &mut String, s: &mut String| {acc.push_str("\n"); acc.push_str(s); acc})
		.expect("failed to get dmenu list")
		.to_string();

	let echo_child = Command::new("echo")
			.arg("-ne")
			.arg(&entries_string)
			.stdout(Stdio::piped())
			.spawn()
			.expect("failed to execute the dmenu");
	let echo_out = echo_child.stdout.expect("failed to execute echo");

	let mut dmenu_command_iter: Vec<String> = shlex::split(&config.dmenu_command).ok_or("error: Invalid quoting").unwrap();
	let general_args: Vec<String> = dmenu_command_iter.split_off(1);

	let dmenu_child_res = Command::new(&dmenu_command_iter[0])
		.args(general_args)
		.args(additional_dmenu_args)
		.stdin(Stdio::from(echo_out))
		.stdout(Stdio::piped())
		.spawn();

	let dmenu_child;
	match dmenu_child_res {
		Ok(d) => {
			dmenu_child = d;
		}
		Err(e) => {
			println!("Executing \"{}\" failed with error: \"{}\"", dmenu_command_iter[0], e);
			exit(1)
		}
	}

	let output = dmenu_child.wait_with_output().expect("failed to get output of the dmenu command");
	String::from_utf8(output.stdout).map_err(|e| e.to_string())
}
