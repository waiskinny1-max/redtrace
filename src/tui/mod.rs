pub mod app;
pub mod events;
pub mod ui;
pub mod views;

use anyhow::Result;

pub fn run() -> Result<()> {
    println!("redtrace TUI is planned for v0.2.");
    println!("Current v0.1 focus: reliable CLI workflow, evidence integrity, validation, and report generation.");
    Ok(())
}
