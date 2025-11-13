use async_trait::async_trait;

// Interface
#[async_trait]
pub trait EmailService {
    async fn is_valid_email(&self, email: &str) -> bool;
}

// Implementation
pub struct StandardEmailService;

#[async_trait]
impl EmailService for StandardEmailService {
    async fn is_valid_email(&self, email: &str) -> bool {
        email.contains('@') && email.contains('.') && !email.is_empty() && email.len() > 3
    }
}
