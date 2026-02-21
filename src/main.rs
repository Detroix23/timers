//! # `Timer`.
//! src/main.rs
//! 
//! Terminal time tools for focusing.

use std::time;

mod cli;

/// Main entry point.
fn main() -> () {
    println!("# Timer.");

    let mut app: cli::Cli = cli::Cli::new(
        time::Duration::from_secs(1),
        cli::defaults::FONT_ANSI_SHADOW,
    );

    app.run();
}
