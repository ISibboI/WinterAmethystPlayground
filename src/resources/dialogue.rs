#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dialogue {
    pub text: String,
    pub sound: Option<String>,
}
