//! # `Timer`.
//! src/cli/base.rs

use std::{thread, time};

use crate::timer::clock;
use crate::cli::{fonts, controls, draw_table};

/// # `Cli`.
/// Print the app to the terminal and holds the state.
pub struct Cli {
	current_clock: clock::Clock,
	refresh_rate: time::Duration,
	font: fonts::Font,
	start: time::SystemTime,
	looping: bool,
	draw_table: draw_table::DrawTable,
}

impl Cli {
	pub fn new(refresh_rate: time::Duration, font: fonts::Font) -> Cli {
		Cli {
			current_clock: clock::Clock::from_secs(0),
			refresh_rate,
			font,
			start: time::SystemTime::now(), 
			looping: false,
			draw_table: draw_table::DrawTable::new(),
		}
	}

	/// Start and run the main loop.
	pub fn run(self: &mut Self) -> () {
		self.start = time::SystemTime::now();
		self.looping = true;

		print!("{}", self.font.characters[0]);

		while self.looping {
			self.update();
			self.draw();

			thread::sleep(self.refresh_rate);
		}
	}

	/// Update the state of the app, every run of the loop.
	fn update(self: &mut Self) -> () {
		self.draw_table.clear();

		match self.start.elapsed() {
			Result::Ok(elapsed) => {
				self.current_clock = clock::Clock::from_secs(elapsed.as_secs());
				
				/* eprintln!("Time: {}h {}m {}s", 
					self.current_clock.get_hours(), 
					self.current_clock.get_minutes(),
					self.current_clock.get_seconds(),
				); */
				
				// self.draw_table.push(self.font.get_character('0').to_string())
				for character in self.current_clock.displays_with_font(&self.font) {
					// print!("{}", character);
					self.draw_table.push(character.to_string());
				}
			},
			Result::Err(error) => {eprintln!("cli::base::Cli.run() `SystemTime` error: {}", error)}
		}
	} 

	/// Draw the clock, updated by `update`.
	fn draw(self: &Self) -> () {
		controls::move_up(self.font.size.1 as u32 + 1);
		// controls::clear_to_bottom();
		// println!("{:?}", self.draw_table.get_table());
		print!("{}", self.draw_table.display());
	}
}
