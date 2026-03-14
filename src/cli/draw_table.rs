//! # `Timer`.
//! src/cli/draw_table.rs

use crate::cli::defaults;

/// # `DrawTable`.
/// 2D table to draw large fonts onto the terminal.
pub struct DrawTable {
	table: Vec<Vec<char>>
}

impl DrawTable {
	pub fn new() -> DrawTable {
		DrawTable { table: Vec::new() }
	}

	pub fn get_table(self: &Self) -> Vec<Vec<char>> {
		self.table.clone()
	}

	/// Get one `String` from the `self.table`, ready to be printed.
	pub fn display(self: &Self) -> String {
		let mut lines: Vec<String> = Vec::new();
		
		for line in self.table.iter() {
			lines.push(line.iter().collect())
		}

		lines.join("\n")
	}

	/// Project the given `string`, accounting the line breaks, onto `table`.
	pub fn push(self: &mut Self, string: String) -> () {
		let mut index_line: usize = 0;
	 	//eprintln!("Push: ");

		for character in string.chars().into_iter() {
			if defaults::LINE_BREAKS.contains(&character) {
				index_line += 1;

				match self.table.last() {
					Option::Some(element) => {
						if element != &[] {
							self.table.push(Vec::new())
						}
					},
					Option::None => {self.table.push(Vec::new())}
				}
				// eprintln!("\tLine break, index={}", index_line);

			} else if index_line < self.table.len() {
				self.table[index_line].push(character);
				// eprintln!("\tChar: {}, index={}", character, index_line);

			} else {
				self.table.push(vec![character]);
				// eprintln!("\tChar (new line): {}, index={}", character, index_line);
			}
		}
	}

	/// Clears the `self.table`.
	pub fn clear(self: &mut Self) -> Vec<Vec<char>> {
		let old: Vec<Vec<char>> = self.table.clone();
		self.table = Vec::new();

		old
	}
}
