use std::borrow::Cow;
use std::env::{var as env, var_os as env_os};
use std::path::Path;

const SEPARATOR: char = '\u{E0B0}';

fn color(code: u8) -> String {
	format!("\x1b[{}m", code)
}

// const color_reset: String = color(0);
// const color_bright: String = color(1);
// const color_dim: String = color(2);
// const color_underscore: String = color(4);
// const color_blink: String = color(5);
// const color_reverse: String = color(7);
// const color_hidden: String = color(8);

const black: u8 = 0;
const red: u8 = 1;
const green: u8 = 2;
const yellow: u8 = 3;
const blue: u8 = 4;
const magenta: u8 = 5;
const cyan: u8 = 6;
const white: u8 = 7;

const bright: u8 = 60;

const brblack: u8 = black + bright;
const brred: u8 = black + bright;
const brgreen: u8 = black + bright;
const bryellow: u8 = black + bright;
const brblue: u8 = black + bright;
const brmagenta: u8 = black + bright;
const brcyan: u8 = black + bright;
const brwhite: u8 = black + bright;

#[derive(Debug)]
struct Segment<'a> {
	bg: u8,
	fg: u8,
	text: &'a str,
}

macro_rules! segment {
	($bg:ident/$fg:ident $text:expr) => {{
		Segment {
			bg: $bg + 40,
			fg: $fg + 30,
			text: $text,
		}
	}};
}

fn status_block<'a>() -> Option<Segment<'a>> {
	let success = env("STATUS")
		.map(|value| value.is_empty() || value == "0")
		.unwrap_or(true);

	if success {
		None
	} else {
		Some(segment!(black/red "✘"))
	}
}

fn root_block<'a>() -> Option<Segment<'a>> {
	let root = env("USER").map(|value| value == "root").unwrap_or(false);

	if root {
		Some(segment!(black/yellow "⚡"))
	} else {
		None
	}
}

fn pwd_block<'a>() -> Option<Segment<'a>> {
	if let Some(pwd) = env_os("PWD") {
		let path = Path::new(&pwd);

		let path = env_os("HOME")
			.and_then(|home| {
				if home.is_empty() {
					return None;
				}

				if let Ok(path) = path.strip_prefix(home) {
					return Some(format!("~/{}", path.to_string_lossy()));
				}
				None
			})
			.unwrap_or(path.to_string_lossy().to_string());

		Some(segment!(brblack / white & path))
	} else {
		None
	}
}

fn main() {
	if let Some(Segment { bg, fg, text }) = status_block() {
		println!("{}{}{}", color(bg), color(fg), text)
	}

	if let Some(Segment { bg, fg, text }) = root_block() {
		println!("{}{}{}", color(bg), color(fg), text)
	}

	if let Some(Segment { bg, fg, text }) = pwd_block() {
		println!("{}{}{}", color(bg), color(fg), text)
	}

	// println!("wow");
}
