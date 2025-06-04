pub struct Insert;
impl super::Statement for Insert {
    fn execute(&self) -> anyhow::Result<()> {
        println!("Executing insert statement");
        Ok(())
    }
}
