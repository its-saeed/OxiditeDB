use crate::table::Table;

pub struct Select;

impl super::Statement for Select {
    fn execute(&self, table: &mut Table) -> anyhow::Result<()> {
        for row in table.into_iter() {
            println!("{row}");
        }
        Ok(())
    }
}
