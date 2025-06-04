use anyhow::{anyhow, Result};
use rustyline::DefaultEditor;

trait Statement {
    fn execute(&self) -> Result<()>;
}

trait MetaCommand {
    fn execute(&self) -> Result<()>;
}

struct ExitMetaCommand;
impl MetaCommand for ExitMetaCommand {
    fn execute(&self) -> Result<()> {
        std::process::exit(0)
    }
}

struct InsertStatement;
struct SelectStatement;

impl Statement for InsertStatement {
    fn execute(&self) -> Result<()> {
        println!("Executing insert statement");
        Ok(())
    }
}

impl Statement for SelectStatement {
    fn execute(&self) -> Result<()> {
        println!("Executing select statement");
        Ok(())
    }
}

fn parse_meta_command(cmd: &str) -> Result<Box<dyn MetaCommand>> {
    match cmd {
        ".exit" => Ok(Box::new(ExitMetaCommand)),
        cmd => Err(anyhow!("Unrecognized command: {cmd}")),
    }
}

fn parse_statement(cmd: &str) -> Result<Box<dyn Statement>> {
    if cmd.starts_with("select") {
        Ok(Box::new(SelectStatement))
    } else if cmd.starts_with("insert") {
        Ok(Box::new(InsertStatement))
    } else {
        Err(anyhow!("Unrecognized command: {cmd}"))
    }
}

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    loop {
        let readline = rl.readline("db > ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                // Non-SQL statements like .exit are called “meta-commands”.
                if line.starts_with('.') {
                    match parse_meta_command(&line) {
                        Ok(cmd) => {
                            if let Err(e) = cmd.execute() {
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
                        Ok(statement) => {
                            if let Err(e) = statement.execute() {
                                eprintln!("{e}");
                                continue;
                            }
                        }
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
