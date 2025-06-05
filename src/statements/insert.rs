const USERNAME_COLUMN_SIZE: usize = 32;
const EMAIL_COLUMN_SIZE: usize = 255;

pub struct Row {
    pub id: u32,
    pub username: String,
    pub email: String,
}

pub struct Insert {
    pub row: Row,
}

impl super::Statement for Insert {
    fn execute(&self) -> anyhow::Result<()> {
        println!("Executing insert statement");
        Ok(())
    }
}
