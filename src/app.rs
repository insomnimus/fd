use clap::{
	crate_version,
	App,
	Arg,
};

pub fn new() -> App<'static> {
	let app = App::new("fd")
		.about("Find files on your filesystem.")
		.version(crate_version!());

	let case_sensitive = Arg::new("case-sensitive")
		.short('c')
		.long("case-sensitive")
		.about("Match patterns case sensitively.");

	let no_ignore = Arg::new("no-ignore")
		.short('n')
		.long("no-ignore")
		.about("Do not ignore hidden directories, do not evaluate .ignore files.");

	let file_type = Arg::new("type")
		.short('t')
		.long("type")
		.about("Look for a specific file type.")
		.possible_values(&["file", "folder"]);

	let depth = Arg::new("depth")
		.short('d')
		.long("depth")
		.about("Maximum depth to recurse.")
		.takes_value(true)
		.validator(is_pos_int);

	let root = Arg::new("root")
		.short('r')
		.long("root")
		.about("The search root.")
		.default_value(".");

	let all = Arg::new("all")
		.short('a')
		.long("all")
		.about("Do not stop after the first match.");

	let follow_links = Arg::new("follow-links")
		.short('f')
		.long("follow-links")
		.about("Follow symbolic links.");

	let args = Arg::new("file")
		.multiple_values(true)
		.required(true)
		.about("File or glob pattern to search for.");

	app.arg(root)
		.arg(all)
		.arg(case_sensitive)
		.arg(file_type)
		.arg(depth)
		.arg(no_ignore)
		.arg(follow_links)
		.arg(args)
}

fn is_pos_int(s: &str) -> Result<(), String> {
	match s.parse::<usize>() {
		Ok(0) | Err(_) => Err(String::from("the value must be a positive integer")),
		Ok(_) => Ok(()),
	}
}
