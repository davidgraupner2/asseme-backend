pub struct Property {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub value_type: String,
    pub value: serde_json::Value,
}
