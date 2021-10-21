pub struct Glob {
	expr: String,
	re: Regex,
}

impl Glob {
	pub fn new(s: &str, case_sensitive: bool) -> Result<Self, regex::Error> {
		let mut buf = String::with_capacity(64);
		if case_insensitive {
			buf.push_str("(?i)");
		}
		let mut p = Parser::new(s);
		buf.extend(p.to_regex());
		Regex::new(buf)
	}
}

struct Parser {
	buf: String,
	chars: Peekable<char>,
}

impl Parser {
	fn new(s: &str) -> Self {
		
	}
	
	fn next(&mut self) -> bool {
		let c = match self.chars.next() {
			None => return false,
			Some(c) => c,
		};
		
		match c{
			'*' if self.chars.next_if_eq('*').is_some() => {
				self.buf.push_str(".*");
			}
			'*' => {
				#[cfg(windows)]
				self.buf.push_str(r"[^/\\]*");
				#[cfg(not(windows))]
				self.buf.push_str(r"[^/]*");
			}
			'?' => {
				#[cfg(windows)]
				self.buf.push_str(r"[^/\\]?");
				#[cfg(not(windows))]
				self.buf.push_str(r"[^/]?");
			}
		}
	}
}