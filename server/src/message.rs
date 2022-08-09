#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub user_id: i32,
    pub message: String,
    pub created_at: chrono::NaiveDateTime,
}

impl ToString for ChatMessage {
    fn to_string(&self) -> String {
        format!(
            "[user_id:{} at: {}]: {}",
            self.user_id,
            self.created_at.format("%Y-%m-%d %H:%M"),
            self.message
        )
    }
}
