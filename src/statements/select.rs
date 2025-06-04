pub struct Select;

impl super::Statement for Select {
    fn execute(&self) -> anyhow::Result<()> {
        println!("Executing select statement");
        Ok(())
    }
}
