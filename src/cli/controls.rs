//! # `Timer`.
//! src/cli/controls.rs  

use ansi_control_codes as ansi;

/// Move the terminal/ tty cursor up. Sequence: `CUU`.
pub fn move_up(times: u32) -> () {
	print!("{}", ansi::control_sequences::CUU(Option::Some(times)));
}

/// Move the terminal/ tty cursor down. Sequence: `CUD`.
pub fn move_down(times: u32) -> () {
	print!("{}", ansi::control_sequences::CUD(Option::Some(times)));
}

/// Move the terminal/ tty cursor right. Sequence: `CUF`.
pub fn move_right(times: u32) -> () {
	print!("{}", ansi::control_sequences::CUF(Option::Some(times)));
}

/// Move the terminal/ tty cursor left. Sequence: `CUB`.
pub fn move_left(times: u32) -> () {
	print!("{}", ansi::control_sequences::CUB(Option::Some(times)));
}

/// Sequence: `EL`.
pub fn clear_line() -> () {
	print!("{}", ansi::control_sequences::EL(Option::Some(
		ansi::control_sequences::EraseLine::ActivePositionToEnd
	)));
}

/// Sequence: `ED`
pub fn clear_to_bottom() -> () {
	print!("{}", ansi::control_sequences::ED(Option::Some(
		ansi::control_sequences::ErasePage::ActivePositionToEnd
	)));
}
