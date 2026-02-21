//! # `Timer`.
//! src/cli/base.rs

use std::{thread, time};

use crate::cli::defaults;

/// # `Cli`.
/// Print the app to the terminal and holds the state.
struct Cli {
	current_clock: time::Duration,
	refresh_rate: time::Duration,
	font: defaults::Font,
	start: time::SystemTime,
	looping: bool,
}

impl Cli {
	pub fn new(refresh_rate: time::Duration, font: defaults::Font) -> Cli {
		Cli {
			current_clock: time::Duration::ZERO,
			refresh_rate,
			font,
			start: time::SystemTime::now(), 
			looping: false,
		}
	}

	/// Start and run the main loop.
	pub fn run(self: &mut Self) -> () {
		self.start = time::SystemTime::now();

		while self.looping {
			self.update();
			self.draw();

			thread::sleep(self.refresh_rate);
		}
	}

	/// Update the state of the app, every run of the loop.
	fn update(self: &mut Self) -> () {
		match self.start.elapsed() {
			Result::Ok(elapsed) => {
				self.current_clock = elapsed;
			},
			Result::Err(error) => {eprintln!("cli::base::Cli.run() `SystemTime` error: {}", error)}
		}

	} 

	/// Draw the clock, updated by `update`.
	fn draw(self: &Self) -> () {
		println!("Time: {}", self.current_clock.as_secs());
	}
}
