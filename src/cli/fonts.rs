//! # `Timer`.
//! src/cli/fonts.rs

use ansi_control_codes::control_sequences::StringDirection;

use crate::timer::clock;

/// Colon (`:`) position in the `Font`'s `character` vector. 
pub const COLON: usize = 10;

/// # `Font`.
/// Define a font with `characters` and a `size`.
pub struct Font { 
	pub characters: [&'static str; 11],
	/// `size`: (x; y) for all characters. 
	pub size: (usize, usize)
}

impl Font {
	pub fn get_character(self: &Self, character: char) -> &'static str {
		return if let Option::Some(digit) = character.to_digit(10) {
			self.characters[digit as usize]
		} else if character == ':' {
			self.characters[COLON]
		} else {
			self.characters[0]
		}
	}
}


impl clock::Clock {
	pub fn displays_with_font(self: &Self, font: &Font) -> Vec<&'static str> {
		let mut displays: Vec<&'static str> = Vec::new();

		let hours = complete_with_zeros(&self.get_hours().to_string(), 2);
		for character in hours.chars() {
			displays.push(font.get_character(character));
		}
		displays.push(font.get_character(':'));

		let minutes = complete_with_zeros(&self.get_minutes().to_string(), 2);
		for character in minutes.chars() {
			displays.push(font.get_character(character));
		}
		displays.push(font.get_character(':'));
		
		let seconds = complete_with_zeros(&self.get_seconds().to_string(), 2);
		for character in seconds.chars() {
			displays.push(font.get_character(character));
		}
		
		displays
	}
}

/// Add zero to `string` to reach the given `size`.
pub fn complete_with_zeros(string: &String, size: usize) -> String {
	let mut new: String = string.clone();
	while new.len() < size {
		new = "0".to_owned() + &new;
	}

	new
}