mod insert;
mod select;

pub use insert::Insert;
use pest::Parser;
use pest_derive::Parser;
pub use select::Select;

use anyhow::{anyhow, Result};

use crate::statements::insert::Row;

pub trait Statement {
    fn execute(&self) -> Result<()>;
}

#[derive(Parser)]
#[grammar = "statements/statements.pest"]
struct StatementParser;

pub fn parse_statement(cmd: &str) -> Result<Box<dyn Statement>> {
    let statement = StatementParser::parse(Rule::statement, cmd.trim())
        .unwrap()
        .next()
        .unwrap();

    let inner_statement = statement.into_inner().next().unwrap();

    match inner_statement.as_rule() {
        Rule::INSERT => {
            let mut inner = inner_statement.into_inner();
            let id = inner.next().unwrap().as_str().parse::<u32>().unwrap();
            let username = inner.next().unwrap().as_str();
            let email = inner.next().unwrap().as_str();
            Ok(Box::new(Insert {
                row: Row {
                    id,
                    username: username.to_string(),
                    email: email.to_string(),
                },
            }))
        }
        Rule::SELECT => Ok(Box::new(Select)),
        _ => Err(anyhow!("Unrecognized command: {cmd}")),
    }
}
