use std::{
	fmt,
	fs::Metadata,
	path::Path,
};

use globset::{
	Error,
	GlobBuilder,
	GlobMatcher,
};

pub enum Pattern {
	Plain(String),
	Glob(GlobMatcher),
}

impl Pattern {
	pub fn new(s: &str) -> Result<Self, Error> {
		if is_glob(s) {
			GlobBuilder::new(s)
				.case_insensitive(cfg!(windows))
				.backslash_escape(cfg!(not(windows)))
				.build()
				.map(|g| Self::Glob(g.compile_matcher()))
		} else {
			Ok(Self::Plain(s.to_string()))
		}
	}

	pub fn is_match(&self, p: &Path) -> bool {
		match self {
			Self::Plain(s) => p.file_name().map_or(false, |p| {
				if cfg!(windows) {
					p.eq_ignore_ascii_case(s)
				} else {
					p.eq(s.as_str())
				}
			}),
			Self::Glob(g) => p.file_name().map_or(false, |p| g.is_match(p)),
		}
	}
}

fn is_glob(s: &str) -> bool {
	#![cfg_attr(windows, allow(clippy::while_let_on_iterator))]
	let (mut brace, mut bracket) = (false, false);
	let mut chars = s.chars();

	while let Some(c) = chars.next() {
		match c {
			'*' | '?' => return true,
			'[' => bracket = true,
			']' if bracket => return true,
			'{' => brace = true,
			'}' if brace => return true,
			#[cfg(not(windows))]
			'\\' => {
				chars.next();
			}
			_ => (),
		}
	}
	false
}

#[derive(Copy, Clone)]
pub enum FileType {
	Any,
	File,
	Directory,
}

impl FileType {
	pub fn is_match(self, md: &Metadata) -> bool {
		match self {
			Self::File => md.is_file(),
			Self::Directory => md.is_dir(),
			Self::Any => true,
		}
	}
}

impl fmt::Display for Pattern {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Plain(s) => f.write_str(s),
			Self::Glob(g) => f.write_str(g.glob().glob()),
		}
	}
}
