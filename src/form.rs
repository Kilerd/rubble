
#[derive_FromForm]
#[derive(Debug)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}