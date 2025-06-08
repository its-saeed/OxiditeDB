use crate::table::{Row, Table};

// const USERNAME_COLUMN_SIZE: usize = 32;
// const EMAIL_COLUMN_SIZE: usize = 255;

pub struct Insert {
    pub row: Row,
}

impl super::Statement for Insert {
    fn execute(&self, table: &mut Table) -> anyhow::Result<()> {
        println!("Executing insert statement");
        Ok(table.insert_row(&self.row)?)
    }
}
