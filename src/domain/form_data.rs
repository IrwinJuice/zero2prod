#[derive(serde::Deserialize)]
pub struct FormData {
    pub(crate) email: String,
    pub(crate) name: String,
}
