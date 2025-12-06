use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::tags)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Tags {
    pub id: i32,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}
