mod insert;
mod select;

pub use insert::Insert;
pub use select::Select;

use anyhow::{anyhow, Result};

pub trait Statement {
    fn execute(&self) -> Result<()>;
}

pub fn parse_statement(cmd: &str) -> Result<Box<dyn Statement>> {
    if cmd.starts_with("select") {
        Ok(Box::new(Select))
    } else if cmd.starts_with("insert") {
        Ok(Box::new(Insert))
    } else {
        Err(anyhow!("Unrecognized command: {cmd}"))
    }
}
