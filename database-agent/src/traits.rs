use anyhow::Result;
use std::fmt::Debug;

pub trait Table: Send + Sync + Debug {
    fn create_table_sql(&self) -> String;
    fn drop_table_sql(&self) -> String;
    fn clone_box(&self) -> Box<dyn Table>;
}

impl Clone for Box<dyn Table> {
    fn clone(&self) -> Box<dyn Table> {
        self.clone_box()
    }
}

pub trait TableRecord {
    fn create(&self) -> Result<Option<&Self>>;
}
