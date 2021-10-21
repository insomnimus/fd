use std::{
	path::PathBuf,
	process,
	sync::{
		atomic::{
			AtomicBool,
			Ordering,
		},
		Arc,
	},
};

use ignore::{
	DirEntry,
	WalkBuilder,
	WalkState::Continue,
};
use wax::Glob;

enum FileType {
	File,
	Folder,
	Any,
}

impl FileType {
	fn is_match(&self, entry: &DirEntry) -> bool {
		match self {
			Self::Any => true,
			Self::File => entry.file_type().map_or(false, |t| t.is_file()),
			Self::Folder => entry.file_type().map_or(false, |t| t.is_dir()),
		}
	}
}

struct Pattern {
	glob: Glob<'static>,
	found: AtomicBool,
	has_sep: bool,
	original: String,
}

impl Pattern {
	fn new(s: &str, case_sensitive: bool) -> Result<Self, wax::GlobError> {
		let mut glob = s.replace("\\", "/");
		let has_sep = glob.contains('/');
		if has_sep && !glob.starts_with("**/") && !glob.starts_with("./**/") {
			glob.insert_str(0, "**/");
		}
		if !case_sensitive {
			glob.insert_str(0, "(?i)");
		}
		Glob::new(&glob)
			.map_err(|e| e.into_owned())
			.map(|glob| Self {
				glob: glob.into_owned(),
				found: AtomicBool::new(false),
				has_sep,
				original: String::from(s),
			})
	}

	fn matches(&self, entry: &DirEntry) -> bool {
		if self.has_sep {
			self.glob.is_match(entry.path())
		} else {
			let fname = entry.file_name();
			self.glob.is_match(fname)
		}
	}
}

pub struct Cmd {
	root: PathBuf,
	args: Arc<Vec<Pattern>>,
	depth: Option<usize>,
	follow_links: bool,
	file_type: FileType,
	all: bool,
	no_ignore: bool,
	quiet: bool,
}

impl Cmd {
	pub fn from_args() -> Self {
		let m = crate::app::new().get_matches();

		let root = PathBuf::from(m.value_of("root").unwrap());
		let all = m.is_present("all");
		let case_sensitive = m.is_present("case-sensitive");
		let file_type = match m.value_of("type") {
			Some("file") => FileType::File,
			Some("folder") => FileType::Folder,
			_ => FileType::Any,
		};

		let depth = m.value_of("depth").map(|s| s.parse::<usize>().unwrap());
		let no_ignore = m.is_present("no-ignore");
		let follow_links = m.is_present("follow-links");
		let quiet = m.is_present("quiet");

		let args = Arc::new(
			m.values_of("file")
				.unwrap()
				.map(|s| {
					Pattern::new(s, case_sensitive).unwrap_or_else(|e| {
						eprintln!("pattern error({}): {}", s, e);
						process::exit(2);
					})
				})
				.collect::<Vec<_>>(),
		);

		Self {
			root,
			all,
			file_type,
			depth,
			no_ignore,
			follow_links,
			args,
			quiet,
		}
	}

	pub fn run(&self) -> bool {
		WalkBuilder::new(&self.root)
			.max_depth(self.depth)
			.follow_links(self.follow_links)
			// .add_custom_ignore_filename(".fdignore")
			.standard_filters(!self.no_ignore)
			.require_git(true)
			.build_parallel()
			.run(|| {
				let pats = Arc::clone(&self.args);
				Box::new(move |entry| {
					let entry = match entry {
						Ok(e) => e,
						Err(_) if self.quiet => return Continue,
						Err(e) => {
							eprintln!("error: {}", e);
							return Continue;
						}
					};

					if self.file_type.is_match(&entry) {
						for p in pats.iter() {
							if p.matches(&entry) {
								println!("{}", entry.path().display());
								p.found.store(true, Ordering::Release);
								if !self.all {
									process::exit(0);
								}
							}
						}
					}
					Continue
				})
			});

		self.args.iter().all(|p| {
			if p.found.load(Ordering::Relaxed) {
				true
			} else {
				eprintln!("{}: not found", &p.original);
				false
			}
		})
	}
}
