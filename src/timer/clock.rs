//! # `Timer`.
//! src/timer/clock.rs

pub struct Clock {
	hours: u64,
	minutes: u64,
	seconds: u64,
}

impl Clock {
	pub fn from_secs(seconds: u64) -> Clock {
		Clock {
			hours: seconds / 3600_u64,
			minutes: seconds % 3600 / 60,
			seconds: seconds % 3600 % 60
		}
	}

	pub fn get_hours(self: &Self) -> u64 {
		self.hours
	}
	
	pub fn get_minutes(self: &Self) -> u64 {
		self.minutes
	}

	pub fn get_seconds(self: &Self) -> u64 {
		self.seconds
	}
}
