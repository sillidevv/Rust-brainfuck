use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
use std::path::Path;

fn main() {
	let mut tape = [0u8; 30_000];

	let mut pc: usize = 0;
	let mut ptr: usize = 0;

	let args: Vec<String> = std::env::args().skip(1).collect::<Vec<_>>();

	// if no file name supplied as first argument then show a usage message
	let file_path = match args.first() {
		Some(f) => f,
		None => {
			println!("Usage: (whatever the executable is) <file>");
			return;
		}
	};

	// check if file exists, if no then say its not a file
	{
		let path = Path::new(file_path);

		// if file doesnt exist
		if !path.exists() || !path.is_file() {
			println!("'{file_path}' is not a file");
			return;
		}
	}

	let code: String = match try_read_file(file_path) {
		Some(f) => f,
		None => {
			println!("Failed to read from file");
			return;
		}
	};

	let jump_table = build_jump_table(&code);

	// loop over the characters in the code
	let chars: Vec<char> = code.chars().collect();
	while pc < chars.len() {
		match chars[pc] {
			'+' => tape[ptr] = tape[ptr].wrapping_add(1),
			'-' => tape[ptr] = tape[ptr].wrapping_sub(1),
			'>' => ptr += 1,
			'<' => ptr -= 1,
			'.' => print!("{}", tape[ptr] as char),
			',' => {
				let mut buffer = [0u8];
				io::stdin().read_exact(&mut buffer).unwrap();
				tape[ptr] = buffer[0];
			},
			'[' => if tape[ptr] == 0 { pc = *jump_table.get(&pc).unwrap(); },
			']' => if tape[ptr] != 0 { pc = *jump_table.get(&pc).unwrap(); },
			_ => {} // ignoring undefined instructions because im lazy
		}

		pc += 1;
	}
}

// builds and returns a jump table from the code
// separated from main function cause a bit verbose
fn build_jump_table(code: &String) -> HashMap<usize, usize> {
	let mut stack = Vec::new();
	let mut table = HashMap::new();
	for (i, c) in code.chars().enumerate() {
		if c == '[' {
			stack.push(i);
		} else if c == ']' {
			let start = stack.pop().expect("unmatched ]");
			table.insert(start, i);
			table.insert(i, start);
		}
	}
	table
}

fn try_read_file(path: &str) -> Option<String> {
	fs::read_to_string(path)
		.ok()
}