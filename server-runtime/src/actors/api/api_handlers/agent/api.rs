use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WSConnect {
    pub id: String,
    pub token: String,
}
