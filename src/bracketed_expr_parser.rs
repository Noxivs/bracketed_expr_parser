pub fn parse(src: &str, start: uint) -> (uint, uint, &str) {
	let mut index = start;
	let mut state = State::default_state();

	while state.round_depth >= 0 && state.curly_depth >= 0 && state.square_depth >= 0 {
		if index >= src.len() {
			fail!("The end of the string was reached with no closing bracket found.");
		}

		state = parse_char(src.char_at(index), state);
		index += 1;
	}

	let end = index - 1;
	(start, end, src.slice(start, end))
}


fn parse_char(c: char, mut state: State) -> State {
	let was_comment = state.block_comment || state.line_comment;

	if state.line_comment {
		if c =='\n' {
			state.line_comment = false;
		}
	} else if state.block_comment {
		if state.has_last_char('*') && c == '/' {
			state.block_comment = false;
		}
	} else if state.single_quote {
		if c == '\'' && !state.escaped {
			state.single_quote = false;
		} else if c == '\\' && !state.escaped {
			state.escaped = true;
		} else {
			state.escaped = false;
		}
	} else if state.double_quote {
		if c == '"' && !state.escaped {
			state.double_quote  = false;
		}  else if c == '\\' && !state.escaped {
			state.escaped = true;
		} else {
			state.escaped = false;
		}
	} else if state.has_last_char('/') && c == '/' {
		state.history = state.history.as_slice().slice_from(1).to_string();
		state.line_comment = true;
	} else if state.has_last_char('/') && c == '*' {
		state.history = state.history.as_slice().slice_from(1).to_string();
		state.block_comment = true;
	} else if c == '\'' {
		state.single_quote = true;
	} else if c == '"' {
		state.double_quote = true;
	} else if c == '(' {
		state.round_depth += 1;
	} else if c == ')' {
		state.round_depth -= 1;
	} else if c == '{' {
		state.curly_depth += 1;;
	} else if c == '}' {
		state.curly_depth -= 1;
	} else if c == '[' {
		state.square_depth += 1;
	} else if c == ']' {
		state.square_depth -= 1;
	}

	if !state.block_comment && !state.line_comment && !was_comment { 
		state.history = String::from_char(1, c).append(state.history.as_slice()); 
	}

	state.last_char = Some(c); 
	state
}


struct State {
	line_comment: bool,
	block_comment: bool,

	single_quote: bool,
	double_quote: bool,

	escaped: bool,

	history: String,
	last_char: Option<char>,

	round_depth: int,
	curly_depth: int,
	square_depth: int
}

impl State {
	fn default_state() -> State {
		State {
			line_comment: false,
			block_comment: false,

			single_quote: false,
			double_quote: false,

			escaped: false,

			history: String::new(),
			last_char: None,

			round_depth: 0,
			curly_depth: 0,
			square_depth: 0
		}
	}

	fn has_last_char(&self, c: char) -> bool {
		match self.last_char {
			Some(lc) => lc == c,
			None => false
		}
	}
}