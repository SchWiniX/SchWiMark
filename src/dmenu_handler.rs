use std::process::{Command, Stdio};
use crate::config;

pub fn open_mark_search(config: &config::Config, entries: &mut Vec<String>) -> Option<String> {
	if entries.is_empty() { return None }
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

	let mark_search_args: Vec<String> = shlex::split(&config.dmenu_mark_search_args).ok_or("error: Invalid quoting").unwrap();

	let dmenu_child = Command::new(&dmenu_command_iter[0])
		.args(general_args)
		.args(mark_search_args)
		.stdin(Stdio::from(echo_out))
		.stdout(Stdio::piped())
		.spawn()
		.expect("failed to execute the dmenu");

	let output = dmenu_child.wait_with_output().expect("failed to get output of the dmenu command");
	Some(String::from_utf8(output.stdout).unwrap())
}

pub fn open_tag_search(config: &config::Config, entries: &mut Vec<String>) -> String {
	let entries_string: String = entries
		.iter_mut()
		.reduce(|acc: &mut String, s: &mut String| {acc.push_str("\n"); acc.push_str(s); acc})
		.expect("failed to get dmenu list").to_string();

	let echo_child = Command::new("echo")
			.arg("-ne")
			.arg(&entries_string)
			.stdout(Stdio::piped())
			.spawn()
			.expect("failed to execute the dmenu");

	let echo_out = echo_child.stdout.expect("failed to execute echo");

	let mut dmenu_command_iter: Vec<String> = shlex::split(&config.dmenu_command).ok_or("error: Invalid quoting").unwrap();
	let general_args: Vec<String> = dmenu_command_iter.split_off(1);

	let tag_search_args: Vec<String> = shlex::split(&config.dmenu_tag_search_args).ok_or("error: Invalid quoting").unwrap();

	let dmenu_child = Command::new(&dmenu_command_iter[0])
		.args(general_args)
		.args(tag_search_args)
		.stdin(Stdio::from(echo_out))
		.stdout(Stdio::piped())
		.spawn()
		.expect("failed to execute the dmenu");

	let output = dmenu_child.wait_with_output().expect("failed to get output of the dmenu command");
	String::from_utf8(output.stdout).unwrap()
}
