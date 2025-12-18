// GUI implementation (placeholder for v0.2.0)

use crate::config::Config;
use std::io;

pub fn run(_config: Config) -> io::Result<()> {
    println!("╭─────────────────────────────────────────────────╮");
    println!("│  GUI mode is under active development          │");
    println!("│  TUI mode is fully functional - please use:    │");
    println!("│  unset DISPLAY && cascolor                     │");
    println!("╰─────────────────────────────────────────────────╯");
    Ok(())
}
