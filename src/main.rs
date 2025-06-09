mod statements;
mod table;

use anyhow::{anyhow, Result};
use rustyline::DefaultEditor;

use crate::{statements::parse_statement, table::Table};

trait MetaCommand {
    fn execute(&self, table: &Table) -> Result<()>;
}

struct ExitMetaCommand;
impl MetaCommand for ExitMetaCommand {
    fn execute(&self, table: &Table) -> Result<()> {
        table.persist()?;
        std::process::exit(0)
    }
}

fn parse_meta_command(cmd: &str) -> Result<Box<dyn MetaCommand>> {
    match cmd {
        ".exit" => Ok(Box::new(ExitMetaCommand)),
        cmd => Err(anyhow!("Unrecognized command: {cmd}")),
    }
}

fn main() -> Result<()> {
    let db_path = std::env::args()
        .nth(1)
        .ok_or(anyhow!("Please specify the database file path."))?;
    let mut rl = DefaultEditor::new()?;
    let mut table = table::Table::open(&db_path)?;
    loop {
        let readline = rl.readline("db > ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                // Non-SQL statements like .exit are called “meta-commands”.
                if line.starts_with('.') {
                    match parse_meta_command(&line) {
                        Ok(cmd) => {
                            if let Err(e) = cmd.execute(&table) {
                                eprintln!("{e}");
                                continue;
                            }
                        }
                        Err(e) => {
                            eprintln!("{e}");
                            continue;
                        }
                    }
                } else {
                    match parse_statement(&line) {
                        Ok(statement) => match statement.execute(&mut table) {
                            Ok(_) => println!("Executed"),
                            Err(e) => {
                                println!("{e}");
                                continue;
                            }
                        },
                        Err(e) => {
                            eprintln!("{e}");
                            continue;
                        }
                    }
                }
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
