use std::path::PathBuf;

use clap::{
	arg,
	crate_version,
	App,
	Arg,
};

use crate::pattern::*;

pub struct Cmd {
	pub file_type: FileType,
	pub root: PathBuf,
	pub depth: Option<usize>,
	pub follow_links: bool,
	pub ignore: bool,
	pub hidden: bool,
	pub quiet: bool,
	pub n: usize,
	pub args: Vec<Pattern>,
}

impl Cmd {
	pub fn from_args() -> Result<Self, globset::Error> {
		let m = App::new("fd")
		.about("Find files and directories.")
		.version(crate_version!())
		.args(&[
		arg!(root: -p --path <ROOT> "The search root.").default_value("."),
		arg!(depth: -r --recursion-depth [DEPTH] "the recursion depth. Unspecified or 0 means no limit.")
		.validator(validate_usize),
		arg!(-l --follow-links "Follow symbolic links."),
		arg!(-f --file "Search for plain files."),
		arg!(-d --directory "Search for directories.")
		.conflicts_with("file")
		.visible_alias("dir"),
		arg!(-I --no-ignore "Do not read .ignore files."),
		arg!(-a --hidden "Do not ignore hidden files and directories."),
		arg!(-q --quiet "Do not report non fatal errors."),
		arg!(n: -n <N> "Show top N matches for each argument past. A value of 0 means all.")
		.default_value("1")
		.validator(validate_usize),
		Arg::new("args")
		.help("File/folder to search for. Glob patterns are allowed.")
		.validator(validate_filename)
		.required(true)
		.multiple_values(true),
		])
		.get_matches();

		let file_type = if m.is_present("file") {
			FileType::File
		} else if m.is_present("directory") {
			FileType::Directory
		} else {
			FileType::Any
		};

		let n = m.value_of("n").unwrap().parse::<usize>().unwrap();
		let args = m
			.values_of("args")
			.unwrap()
			.map(Pattern::new)
			.collect::<Result<Vec<_>, _>>()?;

		let depth = m
			.value_of("depth")
			.and_then(|s| s.parse::<usize>().ok().filter(|&n| n > 0));
		let root = m.value_of("root").map(PathBuf::from).unwrap();

		Ok(Self {
			file_type,
			args,
			n,
			depth,
			root,
			quiet: m.is_present("quiet"),
			follow_links: m.is_present("follow-links"),
			hidden: m.is_present("hidden"),
			ignore: m.is_present("ignore"),
		})
	}
}

fn validate_usize(s: &str) -> Result<(), String> {
	s.parse::<usize>()
		.map(|_| {})
		.map_err(|_| String::from("the value must be a non-negative integer"))
}

fn validate_filename(s: &str) -> Result<(), String> {
	if cfg!(windows) && s.contains(|c| c == '\\' || c == '/' || c == ':') {
		Err(String::from(
			"the file name must not contain any path or drive separators",
		))
	} else if cfg!(not(windows)) && s.contains('/') {
		Err(String::from("the file name can't contain path separators"))
	} else {
		Ok(())
	}
}
